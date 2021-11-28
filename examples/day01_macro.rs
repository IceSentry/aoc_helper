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
