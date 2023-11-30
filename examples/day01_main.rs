use itertools::Itertools;

fn main() {
    dotenvy::dotenv().expect("Failed to load .env");
    let session_token = std::env::var("AOC_COOKIE_SESSION").unwrap();
    let input = aoc_helper::get_input(&session_token, 2020, 1).expect("Failed to get input");
    let parsed_input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution("part_1", part_1, &parsed_input);
    aoc_helper::run_solution("part_2", part_2, &parsed_input);

    // aoc_helper::bench_parser("day01/parser", input, parse);
    // aoc_helper::bench_solution("day01/part_1", part_1, &parsed_input);
    // aoc_helper::bench_solution("day01/part_2", part_2, &parsed_input);
}

fn find_sum(input: &[i32], n: usize) -> Vec<i32> {
    input
        .iter()
        .copied()
        .combinations(n)
        .find(|x| x.iter().sum::<i32>() == 2020)
        .expect("There should be a valid combination")
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: &[i32]) -> i32 {
    find_sum(input, 2).iter().product::<i32>()
}

fn part_2(input: &[i32]) -> i32 {
    find_sum(input, 3).iter().product::<i32>()
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        1721
        979
        366
        299
        675
        1456
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 514579);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 241861950);
    }
}
