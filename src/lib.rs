use std::{
    cmp::min,
    fmt::Display,
    time::{Duration, Instant},
};

use colored::*;

pub use anyhow;
use criterion::{BatchSize, Criterion};

pub mod input;

pub const TEMPLATE: &str = include_str!("./template.rs");

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
    let mut criterion = Criterion::default().with_output_color(true).without_plots();
    let mut group = criterion.benchmark_group("aoc");

    group.bench_with_input("parser", data, |b, _i| {
        b.iter_with_large_drop(|| {
            parser(data);
        });
    });

    let input = parser(data);

    group.bench_with_input("part_1", &input, |b, i| {
        b.iter_batched(|| i, part_1, BatchSize::SmallInput)
    });

    group.bench_with_input("part_2", &input, |b, i| {
        b.iter_batched(|| i, part_2, BatchSize::SmallInput)
    });
}
