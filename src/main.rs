use ansi_term::Style;
use chrono::Datelike;
use chrono::Local;
use directories::ProjectDirs;
use fuzzy_matcher::skim::SkimMatcherV2;
use glob::{Pattern, PatternError};
use std::convert::AsRef;
use std::error::Error;
use std::fs::{create_dir_all, read_dir, DirEntry};
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

struct Search(Pattern);

impl FromStr for Search {
    type Err = PatternError;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(Search(Pattern::from_str(src)?))
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
    New {
        name: Option<Directory>,
    },
    Rescue {
        #[structopt(long = "to", env = "MESS_TARGET_PATH")]
        to: Option<PathBuf>,
        search: Option<String>,
    },
    Prune,
    Install,
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
                .join(format!("{}", today.iso_week().year()))
                .join(format!("{}", today.iso_week().week()));

            if let Some(name) = name {
                dir = dir.join(name);
            }
            if !dir.exists() {
                create_dir_all(&dir)?;
            }
            println!("{}", dir.display());
        }
        Command::Rescue { to, search } => match search {
            Some(search) => {
                let today = Local::today();

                let today_dir = base_dir
                    .join(format!("{}", today.year()))
                    .join(format!("{}", today.iso_week().week()));
                for year in read_dir(base_dir)? {
                    let year = year?;
                    if !year.path().is_dir() {
                        continue;
                    }
                    for week in read_dir(year.path())? {
                        let week = week?;
                        if !week.path().is_dir() {
                            continue;
                        }
                        if week.path() == today_dir {
                            continue;
                        }
                        let matcher = SkimMatcherV2::default();
                        let projects: Vec<DirEntry> = read_dir(week.path())?
                            .filter(|dir_res| {
                                dir_res
                                    .as_ref()
                                    .map(|dir| !dir.file_name().to_string_lossy().starts_with("."))
                                    .unwrap_or(true)
                            })
                            .filter(|dir_res| {
                                dir_res
                                    .as_ref()
                                    .map(|dir| {
                                        matcher
                                            .fuzzy(
                                                dir.file_name().to_string_lossy().as_ref(),
                                                search.as_str(),
                                                false,
                                            )
                                            .is_some()
                                    })
                                    .unwrap_or(false)
                            })
                            .collect::<std::io::Result<Vec<DirEntry>>>()?;
                        if !projects.is_empty() {
                            println!(
                                "{}/{}",
                                Style::new()
                                    .bold()
                                    .paint(year.file_name().to_string_lossy()),
                                Style::new()
                                    .dimmed()
                                    .paint(week.file_name().to_string_lossy())
                            );
                            for project in projects {
                                println!("\t{}", project.file_name().to_string_lossy());
                            }
                        }
                    }
                }
            }
            None => {
                let today = Local::today();

                let today_dir = base_dir
                    .join(format!("{}", today.year()))
                    .join(format!("{}", today.iso_week().week()));
                for year in read_dir(base_dir)? {
                    let year = year?;
                    if !year.path().is_dir() {
                        continue;
                    }
                    for week in read_dir(year.path())? {
                        let week = week?;
                        if !week.path().is_dir() {
                            continue;
                        }
                        if week.path() == today_dir {
                            continue;
                        }
                        let projects: Vec<DirEntry> = read_dir(week.path())?
                            .filter(|dir_res| {
                                dir_res
                                    .as_ref()
                                    .map(|dir| !dir.file_name().to_string_lossy().starts_with("."))
                                    .unwrap_or(true)
                            })
                            .collect::<std::io::Result<Vec<DirEntry>>>()?;
                        if !projects.is_empty() {
                            println!(
                                "{}/{}",
                                Style::new()
                                    .bold()
                                    .paint(year.file_name().to_string_lossy()),
                                Style::new()
                                    .dimmed()
                                    .paint(week.file_name().to_string_lossy())
                            );
                            for project in projects {
                                println!("\t{}", project.file_name().to_string_lossy());
                            }
                        }
                    }
                }
            }
        },
        Command::Prune => unimplemented!(),
        Command::Install => unimplemented!(),
    }
    Ok(())
}

struct MessDir {
    week: u8,
    year: u16,
    projects: Vec<DirEntry>,
}
