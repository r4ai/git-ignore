use std::{
    env, fs,
    process::{self, Command},
};

fn assert_repo_path(case: &str, suffix: &str) {
    let config_path = env::temp_dir().join(format!("git-ignore-{case}-config-{}", process::id()));
    let repo_path = env::temp_dir().join(format!(
        "git ignore {case} repository {}{suffix}",
        process::id()
    ));
    let configured = Command::new("git")
        .args(["config", "--file"])
        .arg(&config_path)
        .arg("ignore.path")
        .arg(&repo_path)
        .status()
        .expect("git should be available");
    assert!(configured.success());

    let output = Command::new(env!("CARGO_BIN_EXE_git-ignore"))
        .arg("--repo")
        .env("GIT_CONFIG_GLOBAL", &config_path)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .expect("git-ignore should run");

    fs::remove_file(config_path).expect("temporary config should be removable");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(
        output.stdout,
        format!("{}\n", repo_path.display()).as_bytes()
    );
}

#[test]
fn repo_option_prints_configured_path_once() {
    assert_repo_path("ordinary", "");
}

#[cfg(unix)]
#[test]
fn repo_option_preserves_configured_line_breaks() {
    assert_repo_path("line-break", "\r\n");
}
