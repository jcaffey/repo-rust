use anyhow::{anyhow, Context, Ok, Result};
use clap::Parser;
use git2::{Repository, RepositoryState, StatusOptions, StatusEntry};
use std::path::PathBuf;
use crate::config::Config;
mod config;

#[derive(Debug)]
struct Cli {
    config: PathBuf,
    operation: Operation,
}

#[derive(Debug)]
enum Operation {
    Open(String),
    Status(String),
    Pull(String),
    Push(String),
}

#[derive(Parser, Debug)]
#[clap()]
/// repo --config <path> <operation> <project|set>
pub struct CommandLineOptions {
    /// <operation> <project|set>
    pub args: Vec<String>,

    /// config path
    #[clap(short = 'c', long = "config")]
    pub config: Option<PathBuf>,
}

impl TryFrom<CommandLineOptions> for Cli {
    type Error = anyhow::Error;

    fn try_from(value: CommandLineOptions) -> Result<Self> {
        return Ok(Cli {
            config: get_config_path(value.config)?,
            operation: value.args.try_into()?,
        });
    }
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.len() == 0 {
            return Err(anyhow!("No arguments provided"));
        }

        let term = value.get(0).expect("must exist");
        if term == "open" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "open operation expects 1 arg but got {}",
                    value.len() - 1
                ));
            }

            return Ok(Operation::Open(value.pop().expect("exists")));
        }

        if term == "status" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "status operation expects 1 arg but got {}",
                    value.len() - 1
                ));
            }

            return Ok(Operation::Status(value.pop().expect("exists")));
        }

        if term == "push" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "push operation expects 1 arg but got {}",
                    value.len() - 1
                ));
            }

            return Ok(Operation::Push(value.pop().expect("exists")));
        }

        return Err(anyhow!("dont know how to handle args: {:?}", value));
    }
}

fn get_config_path(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    // TODO how does context work in anyhow?
    let loc = std::env::var("XDG_CONFIG_HOME").context("unable to get XDG_CONFIG_HOME")?;

    let mut loc = PathBuf::from(loc);
    loc.push("repo");
    loc.push("config.json");

    return Ok(loc);
}

#[derive(Debug)]
enum RepoStatus {
    Clean,
    Dirty,
}

fn get_repo_status(target: &str) -> Result<RepoStatus> {
    match Repository::open(target) {
        std::result::Result::Ok(repo) => {
            if repo.state() != RepositoryState::Clean {
                return Ok(RepoStatus::Dirty);
            }

            // TODO: refactor without unwrap. shame on you.
            let mut so = StatusOptions::new();
            let statuses = repo.statuses(Some(&mut so)).unwrap();
            let statuses: Vec<StatusEntry> = statuses.iter().collect();
            if statuses.len() == 0 {
                return Ok(RepoStatus::Clean);
            }

            return Ok(RepoStatus::Dirty);
        },
        std::result::Result::Err(err) => {
            return Err(anyhow!("Error while getting repo status")).context(err);
        },
    };
}

// TODO: this needs work. it seems to work on second try.
fn push_repo(target: &str) -> std::process::Output {
    std::process::Command::new("git")
        .arg("add")
        .arg("-A")
        .spawn()
        .expect(format!("failed to stage all files for repo {}", target).as_str());

    std::process::Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Default commit message from repo")
        .spawn()
        .expect(format!("failed to commit files for repo {}", target).as_str());

    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(target)
        .arg("push")
        .arg("--porcelain")
        .output()
        .expect("failed to execute process");

    return output;
}

fn main() -> Result<()> {
    let cli: Cli = CommandLineOptions::parse().try_into()?;
    let config = Config::from_path_or_default(cli.config);
    println!("config: {:?}", config);

    // TODO: update operations to use vec of paths
    // see operation::status
    match cli.operation {
        Operation::Open(target) => {
            // TODO: load from config
            // replace {{target}} with target
            let editor = std::env::var("EDITOR").expect("No $EDITOR set");
            std::process::Command::new(editor)
                .arg("-c")
                .arg(format!(":cd {}", target))
                .spawn()?
                .wait()
                .expect("failed to execute process");
        },
        Operation::Status(target) => {
            for path in config.paths_for_target(&target) {
                let path = path.to_str().expect("invalid path");
                let status = get_repo_status(path);
                println!("{:?}: {path}", status);
            }
        },
        Operation::Push(target) => {
            let output = push_repo(&target);
            println!("output: {:?}", output);
        },
        _ => todo!(),
    }

    return Ok(());
}
