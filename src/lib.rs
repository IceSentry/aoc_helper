use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use anyhow::{self, bail, Context, Result};
use colored::*;
use criterion::{BatchSize, BenchmarkId, Criterion};
use std::{fs, path::Path};

const DISPLAY_WIDTH: usize = 32;

pub fn print_with_duration(label: &str, result: &str, duration: Duration) {
    let duration = format!("({duration:.2?})");
    let label_duration = format!("{} {}", label, duration.bright_black());
    print!("{}", label_duration);

    if result.is_empty() {
        println!();
        return;
    }

    let dots = ".".repeat(DISPLAY_WIDTH - label_duration.chars().count());
    println!(" {} {}", dots.bright_black(), result.bold());
}

#[allow(clippy::type_complexity)]
pub fn run_day<ParserOutput, SolutionOutput: Display>(
    input: &str,
    parser: fn(&str) -> Vec<ParserOutput>,
    part_1: fn(&[ParserOutput]) -> SolutionOutput,
    part_2: fn(&[ParserOutput]) -> SolutionOutput,
) {
    let parsed_input = run_parser(input, parser);
    run_solution("part_1", &parsed_input, part_1);
    run_solution("part_2", &parsed_input, part_2);
}

pub fn run_parser<ParserOutput>(
    input: &str,
    parser: fn(&str) -> Vec<ParserOutput>,
) -> Vec<ParserOutput> {
    let start = Instant::now();
    let parsed_input = parser(input);
    let elapsed = start.elapsed();

    print_with_duration("parser", "", elapsed);
    parsed_input
}

pub fn run_solution<ParserOutput, SolutionOutput: Display>(
    label: &str,
    parsed_input: &[ParserOutput],
    solution: fn(&[ParserOutput]) -> SolutionOutput,
) {
    let start = Instant::now();
    let answer = solution(parsed_input);
    let elapsed = start.elapsed();

    print_with_duration(label, &format!("{answer}"), elapsed);
}

pub fn run<F: Fn() -> O, O: Display>(label: &str, f: F) -> O {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    print_with_duration(label, &format!("{result}"), elapsed);
    result
}

/// Runs benchmarks for parsers and solutions idependently.
/// Parser time is not counted in solution time.
#[allow(clippy::type_complexity)]
pub fn bench_day<ParserOutput, SolutionOutput>(
    label: &str,
    data: &str,
    parser: fn(input: &str) -> Vec<ParserOutput>,
    part_1: fn(&[ParserOutput]) -> SolutionOutput,
    part_2: fn(&[ParserOutput]) -> SolutionOutput,
) {
    let mut criterion = Criterion::default().with_output_color(true);
    let mut group = criterion.benchmark_group(label);

    group.bench_with_input(
        BenchmarkId::new("parser", ""),
        &data.to_string(),
        |b, _i| {
            b.iter_with_large_drop(|| {
                parser(data);
            });
        },
    );

    let input = parser(data);

    group.bench_with_input(BenchmarkId::new("part_1", ""), &input, |b, i| {
        b.iter_batched(|| i, part_1, BatchSize::SmallInput)
    });

    group.bench_with_input(BenchmarkId::new("part_2", ""), &input, |b, i| {
        b.iter_batched(|| i, part_2, BatchSize::SmallInput)
    });
}

pub fn bench_solution<ParserOutput, SolutionOutput>(
    parsed_input: Vec<ParserOutput>,
    solution: fn(&[ParserOutput]) -> SolutionOutput,
) {
    let mut criterion = Criterion::default().with_output_color(true);
    criterion.bench_with_input(BenchmarkId::new("solution", ""), &parsed_input, |b, i| {
        b.iter_batched(|| i, solution, BatchSize::SmallInput)
    });
}

/// Gets the input data from the filesystem.
/// If the input file doesn't already exist it will automatically download it.
pub fn get_input(session_token: &str, year: u16, day: u8) -> Result<String> {
    let filename = format!("./inputs/{:02}.txt", day);
    let file_path = Path::new(&filename);
    if !file_path.exists() {
        println!("Downloading inputs for year: {year} day: {day}");
        let input = download_input(session_token, year, day)?;
        fs::create_dir_all("./inputs/")?;
        fs::write(file_path, input)?;
        println!("Input downloaded to {}", file_path.display());
    }
    fs::read_to_string(file_path).context("Failed to read input file")
}

pub fn download_input(session_token: &str, year: u16, day: u8) -> Result<String> {
    let response = ureq::get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .set("COOKIE", &format!("session={session_token}"))
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
