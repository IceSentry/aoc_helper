use std::{
    cmp::min,
    fmt::Display,
    time::{Duration, Instant},
};

use anyhow::{self, bail, Context, Result};
use colored::*;
use criterion::{BatchSize, BenchmarkId, Criterion};
use std::{fs, path::Path};

const DISPLAY_WIDTH: usize = 32;

pub fn print_with_duration(line: &str, output: Option<&str>, duration: Duration) {
    let duration = format!("({:.2?})", duration);
    print!("  - {} {}", line, duration.bright_black());

    if let Some(output) = output {
        let width = "  - ".len() + line.chars().count() + 1 + duration.chars().count();
        let dots = DISPLAY_WIDTH - min(DISPLAY_WIDTH - 5, width) - 2;
        let dots = ".".repeat(dots);
        print!(" {}", dots.bright_black());
        if output.contains('\n') {
            println!();
            for line in output.trim_matches('\n').lines() {
                println!("    {}", line.bold());
            }
        } else {
            println!(" {}", output.bold());
        }
    } else {
        println!();
    }
}

#[allow(clippy::type_complexity)]
pub fn run_single_day<ParserOutput, SolutionOutput>(
    input: &str,
    parser: fn(&str) -> Vec<ParserOutput>,
    part_1: fn(&[ParserOutput]) -> SolutionOutput,
    part_2: fn(&[ParserOutput]) -> SolutionOutput,
) where
    SolutionOutput: Display,
{
    let start = Instant::now();
    let input = parser(input);
    let elapsed = start.elapsed();

    print_with_duration("parser", None, elapsed);

    let start = Instant::now();
    let answer = part_1(&input);
    let elapsed = start.elapsed();

    print_with_duration("part_1", Some(&format!("{}", answer)), elapsed);

    let start = Instant::now();
    let answer = part_2(&input);
    let elapsed = start.elapsed();

    print_with_duration("part_2", Some(&format!("{}", answer)), elapsed);
}

#[allow(clippy::type_complexity)]
pub fn bench<ParserOutput, SolutionOutput: Display>(
    data: &str,
    parser: fn(input: &str) -> Vec<ParserOutput>,
    part_1: fn(&[ParserOutput]) -> SolutionOutput,
    part_2: fn(&[ParserOutput]) -> SolutionOutput,
) {
    let mut criterion = Criterion::default().with_output_color(true);

    criterion.bench_with_input(
        BenchmarkId::new("parser", ""),
        &data.to_string(),
        |b, _i| {
            b.iter_with_large_drop(|| {
                parser(data);
            });
        },
    );

    let input = parser(data);

    criterion.bench_with_input(BenchmarkId::new("part_1", ""), &input, |b, i| {
        b.iter_batched(|| i, part_1, BatchSize::SmallInput)
    });

    criterion.bench_with_input(BenchmarkId::new("part_2", ""), &input, |b, i| {
        b.iter_batched(|| i, part_2, BatchSize::SmallInput)
    });
}

/// Gets the input data from the filesystem.
/// If the input file doesn't already exist it will automatically download it.
pub fn get_input(year: u16, day: u8) -> Result<String> {
    let filename = format!("./inputs/{}/{:02}.txt", year, day);
    let file_path = Path::new(&filename);
    if !file_path.exists() {
        let input = download_input(year, day)?;
        fs::create_dir_all(format!("./inputs/{}/", year))?;
        fs::write(file_path, input)?;
        println!("Input downloaded to {}", file_path.display());
    }
    fs::read_to_string(file_path).context("Failed to read input file")
}

pub fn download_input(year: u16, day: u8) -> Result<String> {
    println!("Downloading inputs...");

    let session = std::env::var("COOKIE_SESSION")?;

    let response = ureq::get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .set("COOKIE", &format!("session={session}"))
    .set("User-Agent", "https://github.com/IceSentry/aoc_helper")
    .call();

    match response {
        Ok(response) => Ok(response.into_string()?),
        Err(ureq::Error::Status(code, _response)) => {
            bail!("Failed to download inputs. status_code={}", code)
        }
        Err(_) => bail!("Unknown error while downloading input"),
    }
}
