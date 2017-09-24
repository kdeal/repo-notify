use github_rs::client::Github;
use serde_json;

#[derive(Deserialize, Debug)]
pub struct GithubCommit {
    pub url: String,
    pub sha: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: Commit,
    pub parents: Vec<Parent>,
}

#[derive(Deserialize, Debug)]
pub struct Parent {
    pub url: String,
    pub sha: String,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub url: String,
    pub author: User,
    pub committer: User,
    pub message: String,
    pub tree: Tree,
    pub comment_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Deserialize, Debug)]
pub struct Tree {
    url: String,
    sha: String,
}

pub fn get_top_commit(client: &Github, owner: &str, repo: &str) -> Result<GithubCommit, ()> {
    let req = client
        .get()
        .repos()
        .owner(owner)
        .repo(repo)
        .commits()
        .execute();
    if let Ok((_, _, json)) = req {
        if let Some(json) = json {
            let mut commits: Vec<GithubCommit> = serde_json::from_value(json).unwrap();
            return Ok(commits.remove(0));
        }
    }
    Err(())
}
