extern crate github_rs;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

use github_rs::client::Github;

mod config;
mod github;
mod todoist;

use todoist::Task;
use todoist::NewTask;
use todoist::NewComment;


fn main() {
    let (mut repos, config) = config::load();
    if let None = config {
        println!("Must set token in config");
        return;
    }
    let config = config.unwrap();

    // TODO: Make a client instead
    let tasks = todoist::get_tasks(&config.todoist_token);

    let client = Github::new(config.github_token.as_str()).unwrap();
    for (name, repo) in repos.iter_mut() {
        let split: Vec<&str> = name.split('/').collect();
        let owner = split[0];
        let repo_name = split[1];

        let gh_commit = github::get_top_commit(&client, owner, repo_name).unwrap();
        let cur_sha = match repo.clone().sha {
            Some(sha) => sha,
            None => "!".to_string(),
        };

        if gh_commit.sha != cur_sha {
            repo.message = Some(gh_commit.commit.message.to_string());
            repo.sha = Some(gh_commit.sha.to_string());
            report_update(
                name.clone(),
                gh_commit.commit.message.to_string(),
                gh_commit.sha.to_string(),
                &tasks,
                &config.todoist_token,
            );
            println!("{} was updated", name);
        }
    }
    config::save(repos, config);
}

fn report_update(
    name: String,
    message: String,
    sha: String,
    tasks: &Vec<Task>,
    token: &String,
) -> () {
    for task in tasks {
        if task.content.contains(name.as_str()) & !task.completed {
            let comment = NewComment {
                task_id: Some(task.id),
                content: format!("{}\n\n sha: {}", message, sha),
                ..NewComment::default()
            };
            todoist::comment(comment, token);
            return;
        }
    }
    let content = format!("Update {}", name);
    let task_id = todoist::create_task(
        NewTask {
            content: content,
            ..NewTask::default()
        },
        token,
    );
    let comment = NewComment {
        task_id: Some(task_id),
        content: format!("{}\n\n sha: {}", message, sha),
        ..NewComment::default()
    };
    todoist::comment(comment, token);
}
