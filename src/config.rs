use std::collections::BTreeMap;
use std::env::home_dir;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use toml;
use toml::value::Table;
use toml::value::Value;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Repo {
    pub message: Option<String>,
    pub sha: Option<String>,
}

#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct Config {
    pub github_token: Option<String>,
    pub todoist_token: Option<String>,
}

pub type Repos = BTreeMap<String, Repo>;

fn expand_config_path<'a>(config_file: &'a str) -> PathBuf {
    let mut path = String::from(config_file);
    if path.starts_with("~") {
        // TODO: Make this better
        path.remove(0);
        path.remove(0);
        let mut home_dir = home_dir().expect("Can't get home directory");
        home_dir.push(&path);
        return home_dir;
    }
    PathBuf::from(config_file)
}

pub fn load<'a>(config_file: &'a str) -> (Repos, Config) {
    let mut repos: Repos = BTreeMap::new();
    let mut config = Config::default();

    let config_file = expand_config_path(config_file);
    let config_file = config_file.as_path();
    if !config_file.exists() {
        return (repos, config);
    }
    let mut config_file = File::open(config_file).expect("file not found");

    let mut contents = String::new();
    config_file.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );
    let table: Table = toml::from_str(&contents).unwrap();

    for (name, repo) in table {
        if name == "config" {
            config = repo.try_into().unwrap();
        } else {
            let repo: Repo = repo.try_into().unwrap();
            repos.insert(name, repo);
        }
    }
    (repos, config)
}

pub fn save<'a>(config_file: &'a str, repos: Repos, config: Config) {
    let mut config_table: Table = Table::new();
    config_table.insert("config".to_string(), Value::try_from(config).unwrap());

    let config_contents = toml::to_string_pretty(&config_table).unwrap();
    let repos_contents = toml::to_string_pretty(&repos).unwrap();

    let config_file = expand_config_path(config_file);
    let config_file = config_file.as_path();
    create_dir_all(config_file.parent().unwrap()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let mut config_file = File::create(config_file).expect("couldn't open file for write");
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
