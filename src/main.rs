use anyhow::{anyhow, Context, Ok, Result};
use clap::Parser;
use git2::{Repository, RepositoryState, StatusOptions, StatusEntry};
use std::path::PathBuf;
use repo::config::Config;

#[derive(Debug)]
struct Cli {
    config: PathBuf,
    operation: Operation,
}

#[derive(Debug)]
enum Operation {
    Open(String),
    List,
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
        if term == "open" || term == "o" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "open operation expects 1 arg but got {}",
                    value.len() - 1
                ));
            }

            return Ok(Operation::Open(value.pop().expect("exists")));
        }

        if term == "list" || term == "l" {
            if value.len() != 1 {
                return Err(anyhow!(
                    "list operation expects 1 arg but got {}",
                    value.len() - 1
                ));
            }

            return Ok(Operation::List);
        }

        if term == "status" || term == "s" {
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

            let mut so = StatusOptions::new();
            let statuses = repo.statuses(Some(&mut so)).expect("statuses should exist");
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

    match cli.operation {
        Operation::Open(target) => {
            let target = config.get_target_type(&target);
            let editor = config.get_editor();
            let args   = config.get_editor_args(&target);

            let mut process = std::process::Command::new(editor);
            process.args(args);
            process.spawn()?
                   .wait()
                   .expect("failed to execute process");
        },
        Operation::List => {
            println!("root: {}", config.get_root_path().unwrap_or(&"".to_string()));
            println!();
            println!("Aliases:");
            for alias in config.get_aliases() {
                println!("{:?}", alias);
            }

            println!();
            println!("Sets:");
            for set in config.get_sets() {
                println!("{:?}", set);
            }
        },
        Operation::Status(target) => {
            for path in config.get_paths_for_target(&target) {
                let path = path.to_str().expect("invalid path");
                let status = get_repo_status(path);
                println!("{:?}: {path}", status);
            }
        },
        Operation::Push(target) => {
            for path in config.get_paths_for_target(&target) {
                let output = push_repo(path.to_str().unwrap());
                println!("output: {:?}", output);
            }
        },
        Operation::Pull(_target) => {
            todo!();
        }
    }

    return Ok(());
}
