use chrono::Datelike;
use chrono::Local;
use directories::ProjectDirs;
use std::error::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::string::ToString;
use structopt::StructOpt;

#[derive(Debug)]
enum AppError {
    FailedToParseProjectName,
}

impl ToString for AppError {
    fn to_string(&self) -> String {
        match self {
            AppError::FailedToParseProjectName => String::from("Failed to parse project name"),
        }
    }
}

fn parse_project_name(src: &str) -> Result<PathBuf, AppError> {
    let path = PathBuf::from(src);
    if path.components().count() != 1 {
        return Err(AppError::FailedToParseProjectName);
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
    let year = today.year();
    let week_num = today.iso_week().week();

    let mut dir = base_dir
        .join(format!("{}", year))
        .join(format!("{}", week_num));

    if let Some(name) = opts.name {
        dir = dir.join(name);
    }
    if !dir.exists() {
        create_dir_all(&dir)?;
    }
    println!("{}", dir.display());
    Ok(())
}
