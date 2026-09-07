#![cfg(unix)]

use std::{
    env, fs,
    os::unix::fs::PermissionsExt,
    process::{self, Command},
};

#[test]
fn clone_failure_is_reported_without_leaving_repository_directory() {
    let test_root = env::temp_dir().join(format!("git-ignore-clone-failure-{}", process::id()));
    let bin_dir = test_root.join("bin");
    let repo_path = test_root.join("repository");
    fs::create_dir_all(&bin_dir).expect("temporary bin directory should be creatable");

    let git_path = bin_dir.join("git");
    fs::write(
        &git_path,
        "#!/bin/sh\nif [ \"$1\" = config ]; then\n  printf '%s\\0' \"$IGNORE_TEST_REPO\"\n  exit 0\nfi\necho 'simulated clone failure' >&2\nexit 42\n",
    )
    .expect("fake git should be writable");
    let mut permissions = fs::metadata(&git_path)
        .expect("fake git metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&git_path, permissions).expect("fake git should be executable");

    let output = Command::new(env!("CARGO_BIN_EXE_git-ignore"))
        .arg("rust")
        .env("PATH", &bin_dir)
        .env("IGNORE_TEST_REPO", &repo_path)
        .output()
        .expect("git-ignore should run");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("simulated clone failure"));
    assert!(!repo_path.exists());
    fs::remove_dir_all(&test_root).expect("temporary directory should be removable");
}
