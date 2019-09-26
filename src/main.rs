use chrono::Datelike;
use chrono::Local;
use directories::ProjectDirs;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::string::ToString;
use structopt::StructOpt;

#[derive(Debug)]
enum NameParseError {
    ContainsPathSeparator,
}

impl ToString for NameParseError {
    fn to_string(&self) -> String {
        match self {
            NameParseError::ContainsPathSeparator => {
                String::from("Project name cannot contain path separator")
            }
        }
    }
}

fn parse_project_name(src: &str) -> Result<PathBuf, NameParseError> {
    let path = PathBuf::from(src);
    if path.components().count() != 1 {
        return Err(NameParseError::ContainsPathSeparator);
    }

    Ok(path)
}

#[derive(StructOpt)]
struct Options {
    #[structopt(parse(try_from_str = parse_project_name))]
    name: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::from_args();

    let dirs = ProjectDirs::from("", "peacememories", "mess")
        .ok_or("Could not find application directory")?;

    let base_dir = dirs.data_dir();
    let today = Local::today();

    let mut dir = base_dir
        .join(format!("{}", today.year()))
        .join(format!("{}", today.iso_week().week()));

    if let Some(name) = opts.name {
        dir = dir.join(name);
    }
    if !dir.exists() {
        create_dir_all(&dir)?;
    }
    println!("{}", dir.display());
    Ok(())
}
