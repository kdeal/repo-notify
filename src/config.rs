use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use toml;
use toml::value::Table;
use toml::value::Value;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Repo {
    pub message: Option<String>,
    pub sha: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Config {
    pub github_token: String,
    pub todoist_token: String,
}

type Repos = BTreeMap<String, Repo>;

pub fn load() -> (Repos, Option<Config>) {
    let mut config_file = File::open("./repos.toml").expect("file not found");

    let mut contents = String::new();
    config_file.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );
    let mut repos: Repos = BTreeMap::new();
    let table: Table = toml::from_str(&contents).unwrap();
    let mut config = None;
    for (name, repo) in table {
        if name == "config" {
            config = Some(repo.try_into().unwrap());
        } else {
            let repo: Repo = repo.try_into().unwrap();
            repos.insert(name, repo);
        }
    }
    (repos, config)
}

pub fn save(repos: Repos, config: Config) {
    let mut config_table: Table = Table::new();
    config_table.insert("config".to_string(), Value::try_from(config).unwrap());

    let config_contents = toml::to_string_pretty(&config_table).unwrap();
    let repos_contents = toml::to_string_pretty(&repos).unwrap();

    let mut config_file = File::create("./repos.toml").expect("couldn't open file for write");
    config_file.write_all(config_contents.as_bytes()).expect(
        "failed to write config",
    );
    config_file.write_all("\n".as_bytes()).expect(
        "failed to write config",
    );
    config_file.write_all(repos_contents.as_bytes()).expect(
        "failed to write config",
    );
}
