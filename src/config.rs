use {anyhow::Result, dirs::data_dir, std::path::PathBuf, std::process::Command};

#[derive(Debug)]
pub struct Config {
    pub gitignore_path: PathBuf,
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
            .args(["config", "--null", "--get", key])
            .output()
            .unwrap();
        let value = String::from_utf8_lossy(&output.stdout);
        value
            .strip_suffix('\0')
            .unwrap_or(value.as_ref())
            .to_string()
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
