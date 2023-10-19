use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, Result};
use dirs::data_dir;
use walkdir::{DirEntry, WalkDir};

fn is_git(entry: &DirEntry) -> bool {
    entry.file_name() == ".git" || entry.file_name() == ".github"
}

fn get_file_name(entry: &DirEntry) -> Option<String> {
    Some(entry.path().file_stem()?.to_str()?.to_string())
}

type IgnoreMap = HashMap<String, String>;

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
        let Some(file_name) = get_file_name(&entry) else {
            continue;
        };
        let file_path = entry.path().display().to_string();
        data.insert(file_name.to_lowercase(), fs::read_to_string(file_path)?);
    }

    Ok(data)
}

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
        "  -h, --help       Print this help message",
        "  -V, --version    Print version info and exit",
        "      --repo       Print gitignore repository path and exit",
        "      --config     (WIP) Print config info and exit",
        "      --list       List all available gitignore files",
    ]
    .join("\n")
}

#[derive(Debug)]
struct Config {
    gitignore_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        // default config
        let mut config = Config {
            gitignore_path: data_dir().unwrap().join("gitignore"),
        };

        // load config from git config
        config.load()?;

        Ok(config)
    }

    fn load_git_config(key: &str) -> String {
        let output = Command::new("git")
            .args(["config", "--get", key])
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    pub fn load(&mut self) -> Result<()> {
        // load ignore.path
        let gitignore_path = Config::load_git_config("ignore.path");
        if !gitignore_path.is_empty() {
            self.gitignore_path = PathBuf::from(gitignore_path);
        }

        Ok(())
    }
}

fn main() {
    let commandline_args: Vec<String> = env::args().collect();
    if commandline_args.len() < 2 {
        println!("{}", help());
        return;
    }

    // get args from commandline
    let args = &commandline_args[1..];

    // check args options
    if args[0] == "--help" || args[0] == "-h" {
        println!("{}", help());
        return;
    }
    if args[0] == "--version" || args[0] == "-V" {
        println!("git-ignore v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args[0] == "--repo" {
        let config = Config::new().unwrap();
        println!("{}", config.gitignore_path.display());
        return;
    }
    if args[0] == "--config" {
        let config = Config::new().unwrap();
        println!("{:?}", config);
        return;
    }
    if args[0] == "--list" {
        let config = Config::new().unwrap();
        let ignore_data = load_gitignore(&config.gitignore_path).unwrap();
        for (name, _) in ignore_data {
            println!("{}", name);
        }
        return;
    }

    let config = Config::new().unwrap();

    let ignore_data = load_gitignore(&config.gitignore_path).unwrap();
    let gitignore = gen_gitignore(&ignore_data, args).unwrap();

    println!("{}", gitignore);
}
