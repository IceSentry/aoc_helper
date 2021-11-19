use anyhow::{anyhow, Context, Result};
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
    let resp = ureq::get(path)
        .set("COOKIE", &format!("session={}", session))
        .call();

    if resp.ok() {
        fs::create_dir_all(&format!("./inputs/{}/", year))?;
        fs::write(file_path, resp.into_string()?)?;

        println!("Input downloaded to {}", file_path.to_str().unwrap());

        Ok(())
    } else {
        Err(anyhow!("{} Failed to download inputs", resp.status()))
    }
}
