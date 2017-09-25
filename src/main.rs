extern crate clap;
extern crate github_rs;
extern crate reqwest;
extern crate serde;
extern crate rpassword;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate text_io;
extern crate toml;

use std::io::{self, Write};

use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use github_rs::client::Github;
use rpassword::prompt_password_stdout;

mod config;
mod github;
mod todoist;

use config::Config;
use config::Repo;
use config::Repos;
use todoist::Task;
use todoist::NewTask;
use todoist::NewComment;


fn get_args<'a>() -> ArgMatches<'a> {
    App::new("repo-notify")
        .version("0.1.0")
        .about("Add todoist task if Github repositories are updated")
        .author("Kyle D. <kdeal@kyledeal.com>")
        .arg(
            Arg::with_name("config_file")
                .help("Config file for repo-notify")
                .default_value("~/.config/repo-notify.toml")
                .long("config-file")
                .short("c")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a repository to the watch list")
                .arg(
                    Arg::with_name("repository")
                        .help("Repository to add")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("check").about(
            "Check for repo updates",
        ))
        .subcommand(SubCommand::with_name("setup").about("Do the initial setup"))
        .get_matches()
}

fn main() {
    let args = get_args();
    let config_file = args.value_of("config_file").unwrap();
    let (repos, config) = config::load(config_file);
    let (repos, config) = match args.subcommand() {
        ("add", Some(sub_args)) => (add(sub_args, repos), config),
        ("check", Some(_)) => check(repos, config),
        ("setup", Some(_)) => setup(repos, config),
        _ => (repos, config),
    };
    config::save(config_file, repos, config);
}

fn add<'a>(args: &'a ArgMatches, mut repos: Repos) -> Repos {
    let repo_name = args.value_of("repository").unwrap().to_string();
    println!("Adding {} to watched repositories", repo_name);
    repos.insert(repo_name, Repo::default());
    repos
}

fn check(mut repos: Repos, config: Config) -> (Repos, Config) {
    // TODO: Make a client instead
    let todoist_token = &config.todoist_token.clone().unwrap();
    let tasks = todoist::get_tasks(todoist_token);

    let client = Github::new(config.github_token.clone().unwrap().as_str()).unwrap();
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
                todoist_token,
            );
            println!("{} was updated", name);
        }
    }
    (repos, config)
}

fn setup(mut repos: Repos, mut config: Config) -> (Repos, Config) {
    // TODO: Make this work for updating single token
    let token = prompt_password_stdout("Github Token: ").unwrap();
    config.github_token = Some(token);
    let token = prompt_password_stdout("Todoist Token: ").unwrap();
    config.todoist_token = Some(token);

    if repos.is_empty() {
        print!("Repository: ");
        io::stdout().flush().unwrap();
        let repository: String = read!("{}\n");
        repos.insert(repository, Repo::default());
    }
    (repos, config)
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
