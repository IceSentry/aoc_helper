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
    println!("Downloading inputs...");

    let session = std::env::var("COOKIE_SESSION")?;

    let response = ureq::get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .set("COOKIE", &format!("session={}", session))
    .call();

    match response {
        Ok(response) => {
            fs::create_dir_all(&format!("./inputs/{}/", year))?;
            fs::write(file_path, response.into_string()?)?;
            println!("Input downloaded to {}", file_path.display());
            Ok(())
        }
        Err(ureq::Error::Status(code, _response)) => {
            bail!("Failed to download inputs. status_code={}", code)
        }
        Err(_) => bail!("Unknown error while downloading input"),
    }
}

pub fn submit(year: usize, day: usize, level: usize, answer: &str) -> Result<()> {
    println!("Sending answer...");

    let response = ureq::post(&format!(
        "https://adventofcode.com/{}/day/{}/answer",
        year, day
    ))
    .send_string(&format!("level={}&answer={}", level, answer));

    match response {
        Ok(_response) => {
            println!("Answer sent succesfully!");
            // TODO parse response to know if it's a wrong answer
            Ok(())
        }
        Err(ureq::Error::Status(code, _response)) => {
            bail!("Failed to send answer. status_code={}", code)
        }
        Err(_) => bail!("Unknown error while sending answer"),
    }
}
