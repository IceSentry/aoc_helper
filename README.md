# aoc_helper

This is a small crate that has a few helper utilities to simplify solving advent of code puzzles.

Highly inspired by <https://github.com/remi-dupre/aoc>

## Usage Guide

### Macro

The intended usage is to use the provided `aoc_helper::main!` macro. This will setup the cli and call the helpers correctly.

```rust
// Example main declaration
aoc_helper::main! {
    // The year needs to be a number
    year: 2020;
    // Each line is a day, the name of the day is also the name of
    // the module containing the related functions.
    //
    // The function before the => is the function that will receive the raw string input and parse it to something easier to use
    //
    // The functions after the => are the possible solutions.
    //   It is a comma separated list of function and can contain as many solution as you want.
    //   They will all receive the same output of the parser
    day01: parse => part_1, part_2;
    // day02: parse => part_1, part_2;
}
```

#### Macro generated CLI

```txt
USAGE:
    day01_macro.exe [FLAGS] <day>

FLAGS:
    -b, --bench       Run criterion benchmark on each solutions for the selected day
    -d, --download    Download the input of the selected day to ./inputs/dayXX
    -h, --help        Prints help information
    -i, --init        Initialize a day by downloading the inputs and creating a template file
    -s, --submit      WARN This is an experimental flag, it will send an answer, but it makes no validation so if you
                      send the wrong answer you won't know until you check the website
    -V, --version     Prints version information

ARGS:
    <day>    The selected day to run
```

### Helpers

The crates also exposes a few helpers if you prefer doing the cli part yourself.
