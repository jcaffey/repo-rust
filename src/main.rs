use std::path::PathBuf;
use clap::Parser;
use anyhow::{Result, anyhow, Ok, Context};
use git2::{Repository, RepositoryState, StatusOptions};

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
                return Err(anyhow!("open operation expects 1 arg but got {}", value.len() - 1));
            }

            return Ok(Operation::Open(value.pop().expect("exists")));
        }

        if term == "status" {
            if value.len() != 2 {
                return Err(anyhow!("status operation expects 1 arg but got {}", value.len() - 1));
            }

            return Ok(Operation::Status(value.pop().expect("exists")));
        }

        if term == "push" {
            if value.len() != 2 {
                return Err(anyhow!("push operation expects 1 arg but got {}", value.len() - 1));
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
    let loc = std::env::var("XDG_CONFIG_HOME")
        .context("unable to get XDG_CONFIG_HOME")?;

    let mut loc = PathBuf::from(loc);
    loc.push("repo");
    loc.push(".repo.json");

    return Ok(loc);
}

fn get_repo_status(target: &str) {
    let repo = match Repository::open(target) {
        std::result::Result::Ok(repo) => {
            println!("im ok");
            println!("repo state: {:?}", repo.state());
            if repo.state() == RepositoryState::Clean {
                println!("repo is clean")
            }

            let index = repo.index().expect("cant open index").is_empty();
            println!("index: {}", index);

            let mut so = StatusOptions::new();
            // so.include_untracked(true);
            // so.include_ignored(true);
            // so.include_unmodified(true);
            // so.include_unreadable(true);
            // so.include_unreadable_as_untracked(true);

            let statuses = repo.statuses(Some(&mut so));
            println!("statuses...");
            statuses.unwrap().iter().for_each( |x| {
                println!("path: {:?}, status: {}", x.path(), x.status().bits());
            })
        },
        std::result::Result::Err(repo) => {
            // todo: anyhow
            println!("wtf... {:?}", repo);
            println!("something bad happened");
        },
    };
}

fn main() -> Result<()> {
    let cli: Cli = CommandLineOptions::parse().try_into()?;

    match cli.operation {
        Operation::Open(target) => {
            let editor = std::env::var("EDITOR").expect("No $EDITOR set");
            let output = std::process::Command::new(editor)
                .arg("-c")
                .arg(format!(":cd {}", target))
                .spawn()?
                .wait()
                .expect("failed to execute process");
            println!("output: {:?}", output);
        },
        Operation::Status(target) => {
            println!("status target! {}", target);
            get_repo_status(&target);
        },
        Operation::Push(target) => {
            println!("push target! {}", target);
        },
        _ => todo!(),
    }

    return Ok(());
}

