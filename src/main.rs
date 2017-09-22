extern crate github_rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
use github_rs::client::Github;

mod config;

#[derive(Deserialize, Debug)]
struct GithubCommit {
    url: String,
    sha: String,
    html_url: String,
    comments_url: String,
    commit: Commit,
    parents: Vec<Parent>,
}

#[derive(Deserialize, Debug)]
struct Parent {
    url: String,
    sha: String,
}

#[derive(Deserialize, Debug)]
struct Commit {
    url: String,
    author: User,
    committer: User,
    message: String,
    tree: Tree,
    comment_count: u32,
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    date: String,
}

#[derive(Deserialize, Debug)]
struct Tree {
    url: String,
    sha: String,
}

fn get_top_commit(client: &Github, owner: &str, repo: &str) -> Result<GithubCommit, ()> {
    let req = client.get().repos().owner(owner).repo(repo).commits().execute();
    if let Ok((_, _, json)) = req {
        if let Some(json) = json {
            let mut commits: Vec<GithubCommit> = serde_json::from_value(json).unwrap();
            return Ok(commits.remove(0));
        }
    }
    Err(())
}

fn main() {
    let mut repos = config::load();
    let client = Github::new("<token>").unwrap();
    for (name, repo) in repos.iter_mut() {
        let split: Vec<&str> = name.split('/').collect();
        let owner = split[0];
        let repo_name = split[1];

        let gh_commit = get_top_commit(&client, owner, repo_name).unwrap();
        let cur_sha = match repo.clone().sha {
            Some(sha) => sha,
            None => "!".to_string(),
        };

        if gh_commit.sha != cur_sha {
            repo.message = Some(gh_commit.commit.message.to_string());
            repo.sha = Some(gh_commit.sha.to_string());
            println!("{} was updated", name);
        }
    }

    config::save(repos);
}
