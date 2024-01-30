pub struct CommitResult {
    pub files: Vec<String>,
    pub issues: Vec<String>,
}

impl CommitResult {
    pub fn default() -> CommitResult {
        return CommitResult {
            files: Vec::new(),
            issues: Vec::new(),
        };
    }
}

pub struct Config {
    pub repo_path: String,
    pub depth: i32,
    pub issue_regex: String,
}

impl Config {
    pub fn default() -> Config {
        return Config {
            repo_path: String::from("."),
            depth: 10240,
            issue_regex: String::from(r"(#\d+)"),
        };
    }
}