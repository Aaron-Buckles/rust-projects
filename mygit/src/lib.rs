use std::path::{Path, PathBuf};
use std::fmt::{Result as FmtResult, Formatter, Display};
use std::collections::BTreeMap;
use clap::{Args, Parser, Subcommand};
use configparser::ini::Ini;
use sha1::Sha1;
use regex::Regex;

#[derive(Debug, Parser)]
#[clap(name = "mygit")]
#[clap(about = "A stupid content tracker", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init {
        #[clap(default_value = ".")]
        path: String,
    }
}

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Init { path } => {
            println!("Running init! path={:?}", path);
        }
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    NotAGitRepository,
}

impl RepositoryError {
    fn message(&self) -> &str {
        match self {
            Self::NotAGitRepository => "Not a Git repository",
        }
    }
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error: {}", self.message())
    }
}

struct Repository {
    worktree: PathBuf,
    gitdir: PathBuf,
    conf: Ini,
}

impl Repository {
    pub fn new(path: &str, force: bool) -> Result<Self, RepositoryError> {
        let worktree = PathBuf::from(path);
        let gitdir = Path::new(path).join(".git");

        if !(force || gitdir.is_dir()) {
            return Err(RepositoryError::NotAGitRepository)
        }

        let conf = Ini::new();

        // TODO: Much more todo

        Ok(Self { worktree, gitdir, conf })
    }
}