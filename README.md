# repo-notify

[![Latest Travis CI build status](https://travis-ci.org/kdeal/repo-notify.png)](https://travis-ci.org/kdeal/repo-notify)

Checks for new commits in configured Github repositories and makes a task in
[Todoist](https://en.todoist.com/) for each updated repository.  This is
intended to be ran as a cron job every day/hour.

## Install

Download the latest [release](https://github.com/kdeal/alfred-datetime/releases)
and put it on your path.

I recommend moving repo-notify to `~/.local/bin/` and adding
`export PATH=$PATH:~/.local/bin/` to your `~/.bashrc`.

## Usage

### Setup
1. `repo-notify setup` - set your Github and Todoist api token and add
   the first repository to your repository watch list (settings saved to
    `~/.config/repo-notify.toml`)

1. `repo-notify add <repository>` - add the rest of the repositories to
   your repository watch list


### Check
`repo-notify check` to check for repository updates or add cron to do this
automatically

#### Example Cron

    0 0 * * * ~/.local/bin/repo-notify check >/dev/null 2>&1
