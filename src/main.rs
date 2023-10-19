mod config;

use crate::config::Config;
use anyhow::{anyhow, Result};
use std::{collections::HashMap, env, ffi::OsStr, fs, io, os, path::Path, process::Command};
use walkdir::{DirEntry, WalkDir};

fn is_git(entry: &DirEntry) -> bool {
    entry.file_name() == ".git" || entry.file_name() == ".github"
}

fn get_file_name_without_extension(entry: &DirEntry) -> Option<String> {
    Some(entry.path().file_stem()?.to_str()?.to_string())
}

type IgnoreMap = HashMap<String, String>;

/// Initialize gitignore repository
/// - If `config.gitignore_path` exists, do nothing.
/// - Otherwise, clone gitignore repository to `config.gitignore_path`.
fn init_gitignore(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    // create gitignore directory
    fs::create_dir(path)?;

    // clone gitignore repository
    Command::new("git")
        .args([
            "clone",
            "https://github.com/github/gitignore.git",
            path.display().to_string().as_str(),
        ])
        .output()
        .unwrap_or_else(|_| panic!("Failed to clone gitignore repository.\nExecuted command: `git clone https://github.com/github/gitignore.git {}`", path.display()));

    Ok(())
}

/// Load gitignore files recursively from `config.gitignore_path`
fn load_gitignore(path: &Path) -> Result<IgnoreMap> {
    init_gitignore(path)?;
    let mut data: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(path).into_iter().filter_entry(|e| !is_git(e)) {
        // check if entry is file and has .gitignore extension
        let entry = entry?;
        if entry.file_type().is_dir() || entry.path().extension() != Some(OsStr::new("gitignore")) {
            continue;
        }

        // insert file name and content to data
        let Some(file_name) = get_file_name_without_extension(&entry) else {
            continue;
        };
        let file_path = entry.path().display().to_string();
        data.insert(file_name.to_lowercase(), fs::read_to_string(file_path)?);
    }

    Ok(data)
}

/// Generate gitignore file specified by args
fn gen_gitignore(data: &IgnoreMap, args: &[String]) -> Result<String> {
    let mut gitignore = String::new();

    for (i, arg) in args.iter().enumerate() {
        if let Some(content) = data.get(arg) {
            if i > 0 {
                gitignore.push('\n');
            }
            gitignore.push_str(&format!("### {} ###\n", arg));
            gitignore.push_str(content);
        } else {
            return Err(anyhow!("{} is not found in gitignore repository.", arg));
        }
    }

    Ok(gitignore)
}

fn help() -> String {
    [
        "Generate gitignore",
        "",
        "Usage: git ignore <lang1> <lang2> ...",
        "",
        "Example:",
        "    # Generate gitignore for nodejs and save it to `.gitignore`",
        "    $ git ignore node > .gitignore",
        "",
        "    # Generate gitignore for rust and python and append it to `.gitignore`",
        "    $ git ignore rust python >> .gitignore",
        "",
        "    # Configure gitignore repository path",
        "    $ git config --global ignore.path <path>",
        "",
        "Options:",
        "  -h, --help                       Print this help message",
        "  -V, --version                    Print version information and exit",
        "      --repo                       Print gitignore repository path and exit",
        "      --list                       List all available gitignore files",
        "  -c, --completion <bash|zsh|fish> Generate completion script for bash, zsh or fish",
        "      --register                   Register `git-ignore` command as git subcommand",
    ]
    .join("\n")
}

enum ParseResult {
    Continue,
    Break,
}

fn parse_args(args: &[String]) -> Result<ParseResult> {
    // check args options
    if args[0] == "--help" || args[0] == "-h" {
        println!("{}", help());
        return Ok(ParseResult::Break);
    }
    if args[0] == "--version" || args[0] == "-V" {
        println!("git-ignore v{}", env!("CARGO_PKG_VERSION"));
        return Ok(ParseResult::Break);
    }
    if args[0] == "--repo" {
        let config = Config::new().unwrap();
        println!("{}", config.gitignore_path.display());
        return Ok(ParseResult::Break);
    }
    if args[0] == "--list" {
        let config = Config::new().unwrap();
        let ignore_data = load_gitignore(&config.gitignore_path).unwrap();
        for (name, _) in ignore_data {
            println!("{}", name);
        }
        return Ok(ParseResult::Break);
    }
    if args[0] == "-c" || args[0] == "--completion" {
        fn shell_help() -> String {
            [
                "Please specify correct shell name.",
                "",
                "Usage:   git ignore --completion <bash|zsh|fish>",
                "Example: git ignore --completion bash",
            ]
            .join("\n")
        }
        let Some(shell) = args.get(1) else {
            println!("{}", shell_help());
            return Ok(ParseResult::Break);
        };
        match shell.as_str() {
            "bash" => println!("{}", include_str!("completions/bash_completions.bash")),
            "zsh" => println!("{}", include_str!("completions/zsh_completions.zsh")),
            "fish" => println!("{}", include_str!("completions/fish_completions.fish")),
            _ => println!("{}", shell_help()),
        }
        return Ok(ParseResult::Break);
    }
    if args[0] == "--register" {
        let output = Command::new("git").args(["--exec-path"]).output().unwrap();
        let git_exec_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let git_subcommand_path = Path::new(&git_exec_path).join("git-ignore");

        if git_subcommand_path.exists() {
            if git_subcommand_path.is_file() {
                match fs::remove_file(&git_subcommand_path) {
                    Ok(_) => {}
                    Err(err) => {
                        if err.kind() == io::ErrorKind::PermissionDenied {
                            let msg = [
                                "Failed to register git-ignore command.",
                                "Please run this command as root user.",
                                "Example:",
                                "    $ sudo git-ignore --register",
                            ]
                            .join("\n");
                            return Err(anyhow!(msg));
                        }
                        return Err(err.into());
                    }
                }
            } else {
                return Err(anyhow!(
                    "Failed to register git-ignore command.\n{} is already exists.\nPlease remove it manually to register the command.",
                    git_subcommand_path.display()
                ));
            }
        }

        #[cfg(target_os = "windows")]
        {
            os::windows::fs::symlink_file(env::current_exe().unwrap(), git_subcommand_path)
                .expect("Failed to register git-ignore command.")
        }

        #[cfg(not(target_os = "windows"))]
        {
            os::unix::fs::symlink(env::current_exe().unwrap(), git_subcommand_path)
                .expect("Failed to register git-ignore command.")
        }

        println!("Successfully registered git-ignore command.");
        return Ok(ParseResult::Break);
    }

    Ok(ParseResult::Continue)
}

fn main() {
    let commandline_args: Vec<String> = env::args().collect();
    if commandline_args.len() < 2 {
        println!("{}", help());
        return;
    }

    let args = &commandline_args[1..];
    match parse_args(args) {
        Ok(ParseResult::Continue) => {}
        Ok(ParseResult::Break) => {
            return;
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    let config = Config::new().unwrap();

    let ignore_data = load_gitignore(&config.gitignore_path).unwrap();
    let gitignore = match gen_gitignore(&ignore_data, args) {
        Ok(gitignore) => gitignore,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    println!("{}", gitignore);
}
