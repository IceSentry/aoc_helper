use std::fmt::Display;
use std::path::Path;
use std::time::Duration;
use std::{cmp::min, time::Instant};

use colored::*;

pub use anyhow;
use colored;
use criterion;
use criterion::{BatchSize, Criterion};
use dotenv;
use structopt::StructOpt;

pub mod input;

pub const TEMPLATE: &str = include_str!("./template.rs");

const DISPLAY_WIDTH: usize = 40;

type ParserFn<T> = fn(input: &str) -> T;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc_helper")]
pub struct Opt {
    /// The selected day to run
    pub day: String,
    /// Run criterion benchmark on each solutions for the selected day
    #[structopt(short, long)]
    pub bench: bool,
    /// Download the input of the selected day to ./inputs/dayXX
    #[structopt(short, long)]
    pub download: bool,
    /// Initialize a day by downloading the inputs and creating a template file
    #[structopt(short, long)]
    pub init: bool,
    /// WARN
    /// This is an experimental flag, it will send an answer,
    /// but it makes no validation so if you send the wrong answer you won't know until you check the website
    #[structopt(short, long)]
    pub submit: bool,
}

pub fn main_setup(year: u16, days: &[&str]) -> Option<(String, String, Opt)> {
    dotenv::dotenv().expect("Failed to load .env");
    let opt = Opt::from_args();
    let module_name = format!("day{:0>2}", opt.day);
    let day: u8 = opt.day.parse().expect("Day is not a number");

    if opt.init {
        let filename = format!("./src/day{:0>2}.rs", opt.day);
        let file_path = Path::new(&filename);
        std::fs::write(file_path, TEMPLATE).expect("Failed to write file");
        println!("new file created at {}", file_path.display());
    }

    let data = input::get_input(year, day).expect("Failed to get input data");
    if opt.download {
        return None;
    }

    if !days.contains(&module_name.as_str()) {
        eprintln!(
            "Module `{}` was not registered, modules available are: {}",
            module_name,
            days.join(", "),
        );
        None
    } else {
        println!("Day {:0>2}", day);
        Some((data, module_name, opt))
    }
}

#[macro_export]
macro_rules! main {
    (year : $year: expr;) => {
        use $crate::anyhow::Result;

        fn main() -> Result<()> {
            $crate::main_setup($year, &[]);
            Ok(())
        }
    };
    (
        year : $year: expr;
        $( $day: ident $( : $parser: ident )? => $( $solution: ident ),+ );+
        $( ; )?
    ) => {
        use $crate::anyhow::{Result, bail};

        $( mod $day; )+

        fn main() -> Result<()> {
            if let Some((data, module_name, opt)) = $crate::main_setup($year, &[$(stringify!($day)),*]) {
                let input = data.as_str();
                let year = stringify!($year);
                $(
                    if module_name == stringify!($day) {
                        let day = stringify!($day);
                        let parser = $( $day::$parser )?;
                        let solutions: Vec<(&str, Box<dyn Fn(&_) -> _>)> = vec![$((stringify!($solution), Box::new($day::$solution)),)*];

                        if opt.bench {
                            $crate::bench(year, day, input, parser, &solutions);
                        } else {
                            let answer = $crate::run(input, parser, &solutions);
                            if opt.submit {
                                $crate::input::submit(
                                    year.parse()?,
                                    opt.day.parse()?,
                                    1,
                                    &answer.expect("You need a valid solution to submit an answer").to_string())
                                .expect("Failed to submit answer!");
                            }
                        }
                    }
                )+
            }
            Ok(())
        }
    };
}

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

pub fn run<ParserOutput, SolutionOutput>(
    input: &str,
    parser: fn(&str) -> ParserOutput,
    solutions: &[(&str, Box<dyn Fn(&ParserOutput) -> SolutionOutput>)],
) -> Option<SolutionOutput>
where
    SolutionOutput: Display,
{
    let start = Instant::now();
    let input = parser(input);
    let elapsed = start.elapsed();

    print_with_duration("parser", None, elapsed);

    let mut output = None;
    for (id, solution) in solutions {
        let start = Instant::now();
        let answer = solution(&input);
        let elapsed = start.elapsed();

        print_with_duration(id, Some(&format!("{}", answer)), elapsed);

        output = Some(answer);
    }
    output
}

pub fn bench<ParserOutput, SolutionOutput: Display>(
    year: &str,
    day: &str,
    data: &str,
    parser: ParserFn<ParserOutput>,
    solutions: &[(&str, Box<dyn Fn(&ParserOutput) -> SolutionOutput>)],
) {
    let mut criterion = Criterion::default().with_output_color(true).without_plots();
    let mut group = criterion.benchmark_group(format!("{}-{:0>2}", year, day));

    group.bench_with_input("parser", data, |b, _i| {
        b.iter_with_large_drop(|| {
            parser(data);
        });
    });

    let input = parser(data);

    for (id, solution) in solutions {
        group.bench_with_input(*id, &input, |b, i| {
            b.iter_batched(|| i, solution, BatchSize::SmallInput)
        });
    }
}
