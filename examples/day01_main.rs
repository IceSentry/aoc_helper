use itertools::Itertools;
use serde_scan::scan;

#[allow(clippy::type_complexity)]
fn main() {
    dotenvy::dotenv().expect("Failed to load .env");
    let input = aoc_helper::input::get_input(2020, 1).expect("Failed to get input");
    aoc_helper::run_single_day(&input, parse, part_1, part_2);
    aoc_helper::bench(&input, parse, part_1, part_2);
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
    input.lines().map(|l| scan!("{}" <- l).unwrap()).collect()
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
