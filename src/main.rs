use chrono::Datelike;
use chrono::Local;
use directories::ProjectDirs;
use std::convert::AsRef;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;
use structopt::StructOpt;

struct Directory(PathBuf);

impl AsRef<Path> for Directory {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl FromStr for Directory {
    type Err = NameParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(src);
        if path.components().count() != 1 {
            return Err(NameParseError::ContainsPathSeparator);
        }

        Ok(Directory(path))
    }
}

#[derive(Debug)]
enum NameParseError {
    ContainsPathSeparator,
}

impl ToString for NameParseError {
    fn to_string(&self) -> String {
        match self {
            NameParseError::ContainsPathSeparator => {
                String::from("Directory name cannot contain path separator")
            }
        }
    }
}

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "b", long = "basepath", env = "MESS_BASE_PATH")]
    base_path: Option<PathBuf>,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    New { name: Option<Directory> },
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::from_args();

    let dirs = ProjectDirs::from("", "peacememories", "mess");

    let base_dir = match &opts.base_path {
        Some(path) => path,
        None => dirs
            .as_ref()
            .map(ProjectDirs::data_dir)
            .ok_or("Could not find application directory")?,
    };

    match opts.command {
        Command::New { name } => {
            let today = Local::today();

            let mut dir = base_dir
                .join(format!("{}", today.year()))
                .join(format!("{}", today.iso_week().week()));

            if let Some(name) = name {
                dir = dir.join(name);
            }
            if !dir.exists() {
                create_dir_all(&dir)?;
            }
            println!("{}", dir.display());
        }
    }
    Ok(())
}
