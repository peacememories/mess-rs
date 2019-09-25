use chrono::Datelike;
use chrono::Local;
use directories::ProjectDirs;
use std::error::Error;
use std::fs::create_dir_all;

fn main() -> Result<(), Box<dyn Error>> {
    let dirs = ProjectDirs::from("", "peacememories", "mess")
        .ok_or("Could not find application directory")?;

    let base_dir = dirs.data_dir();
    let today = Local::today();
    let year = today.year();
    let week_num = today.iso_week().week();

    let dir = base_dir
        .join(format!("{}", year))
        .join(format!("{}", week_num));
    if !dir.exists() {
        create_dir_all(&dir)?;
    }
    println!("{}", dir.display());
    Ok(())
}
