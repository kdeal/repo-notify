use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use toml;
use toml::value::Table;

#[derive(Clone,Deserialize,Debug,Serialize)]
pub struct Repo {
    pub message: Option<String>,
    pub sha: Option<String>,
}

type Repos = BTreeMap<String, Repo>;

pub fn load() -> Repos {
    let mut config_file = File::open("./repos.toml")
        .expect("file not found");

    let mut contents = String::new();
    config_file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut repos: Repos = BTreeMap::new();
    let tokens: Table = toml::from_str(&contents).unwrap();
    for (name, repo) in tokens {
        let repo: Repo = repo.try_into().unwrap();
        repos.insert(name, repo);
    }
    repos
}

pub fn save(repos: Repos) {
    let mut config_file = File::create("./repos.toml")
        .expect("couldn't open file for write");

    let config_contents = toml::to_string_pretty(&repos).unwrap();
    config_file.write_all(config_contents.as_bytes())
        .expect("failed to write config");
}
