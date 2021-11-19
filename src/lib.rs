use std::fmt::Display;
use std::path::Path;
use std::time::Duration;
use std::{cmp::min, time::Instant};

use colored::*;

pub use colored;
pub use criterion;
use criterion::{BatchSize, Criterion};
pub use dotenv;
use structopt::StructOpt;

pub mod input;

pub const TEMPLATE: &str = include_str!("./template.rs");

const DISPLAY_WIDTH: usize = 40;

type ParserFn<T> = fn(input: &str) -> T;
type SolutionFn<Input, Output> = fn(input: &Input) -> Output;

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
        println!()
    }
}

pub fn run<ParserOutput, SolutionOutput: Display>(
    input: &str,
    parser: ParserFn<ParserOutput>,
    solutions: Vec<(&str, SolutionFn<ParserOutput, SolutionOutput>)>,
) {
    let start = Instant::now();
    let input = parser(input);
    let elapsed = start.elapsed();

    print_with_duration("parser", None, elapsed);

    for (id, solution) in solutions {
        let start = Instant::now();
        let answer = solution(&input);
        let elapsed = start.elapsed();

        print_with_duration(id, Some(&format!("{}", answer)), elapsed);
    }
}

pub fn bench<ParserOutput, SolutionOutput: Display>(
    year: &str,
    day: &str,
    data: &str,
    parser: ParserFn<ParserOutput>,
    solutions: Vec<(&str, SolutionFn<ParserOutput, SolutionOutput>)>,
) {
    let mut criterion = Criterion::default().without_plots().with_output_color(true);
    let mut group = criterion.benchmark_group(format!("{}-{:0>2}", year, day));

    group.bench_with_input("parser", data, |b, _i| {
        b.iter_with_large_drop(|| {
            parser(data);
        });
    });

    let input = parser(data);

    for (id, solution) in solutions {
        group.bench_with_input(id, &input, |b, i| {
            b.iter_batched(|| i, solution, BatchSize::SmallInput)
        });
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc_helper")]
pub struct Opt {
    pub day: String,
    #[structopt(short, long)]
    pub bench: bool,
    #[structopt(short, long)]
    pub download: bool,
    #[structopt(short, long)]
    pub init: bool,
}

pub fn main_setup(year: u16, days: &[&str]) -> Option<(String, String, Opt)> {
    dotenv::dotenv().expect("Failed to load .env");
    let opt = Opt::from_args();
    let module_name = format!("day{:0>2}", opt.day);
    let day: u8 = opt.day.parse().expect("Day is not a number");
    let data = input::get_input(year, day).expect("Failed to get input data");

    if opt.download {
        return None;
    }

    if opt.init {
        let filename = format!("./src/day{}.rs", opt.day);
        let file_path = Path::new(&filename);
        std::fs::write(file_path, TEMPLATE).expect("Failed to write file");
        println!("new file created at {}", file_path.display());
    }

    if !days.contains(&module_name.as_str()) {
        eprintln!(
            "Module `{}` was not registered, modules available are: {}",
            module_name,
            days.join(", "),
        );
    }

    println!("Day {:0>2}", day);

    Some((data, module_name, opt))
}

#[macro_export]
macro_rules! main {
    (
        year : $year: expr;
        $( $day: ident $( : $parser: ident )? => $( $solution: ident ),+ );+
        $( ; )?
    ) => {
        fn main() {
            let setup = $crate::main_setup($year, &[$(stringify!($day)),*]);
            if let Some((data, module_name, opt)) = setup {
                let input = data.as_str();
                $(
                    if module_name == stringify!($day) {
                        if opt.bench {
                            $crate::bench(
                                stringify!($year),
                                stringify!($day),
                                input,
                                $( crate::$day::$parser )?,
                                vec![$((stringify!($solution), $day::$solution),)*]
                            );
                        } else {
                            $crate::run(
                                input,
                                $( crate::$day::$parser )?,
                                vec![$((stringify!($solution), $day::$solution),)*]
                            );
                        }
                    }
                )+
            }
        }
    };
}
