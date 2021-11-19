use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

pub fn get_input(year: u16, day: u8) -> Result<String> {
    let filename = format!("./inputs/{}/{:02}.txt", year, day);
    let file_path = Path::new(&filename);
    if !file_path.exists() {
        download_input(file_path, year, day)?;
    }
    fs::read_to_string(&file_path).context("Failed to read input file")
}

pub fn download_input(file_path: &Path, year: u16, day: u8) -> Result<()> {
    println!("downloading inputs...");
    let path = &format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let session = std::env::var("COOKIE_SESSION")?;

    let request = ureq::get(path).set("COOKIE", &format!("session={}", session));
    match request.call() {
        Ok(response) => {
            fs::create_dir_all(&format!("./inputs/{}/", year))?;
            fs::write(file_path, response.into_string()?)?;
            println!("Input downloaded to {}", file_path.display());
            Ok(())
        }
        Err(ureq::Error::Status(code, _response)) => {
            bail!("Failed to download inputs. status={}", code)
        }
        Err(_) => bail!("Unknown error while downloading input"),
    }
}
