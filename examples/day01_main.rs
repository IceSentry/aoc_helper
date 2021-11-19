mod day01 {
    use itertools::Itertools;
    use serde_scan::scan;

    fn find_sum(input: &[i32], n: usize) -> Vec<i32> {
        input
            .iter()
            .copied()
            .combinations(n)
            .find(|x| x.iter().sum::<i32>() == 2020)
            .expect("There should be a valid combination")
    }

    pub fn parse(input: &str) -> Vec<i32> {
        input.lines().map(|l| scan!("{}" <- l).unwrap()).collect()
    }

    pub fn part_1(input: &Vec<i32>) -> i32 {
        find_sum(input, 2).iter().product::<i32>()
    }

    pub fn part_2(input: &Vec<i32>) -> i32 {
        find_sum(input, 3).iter().product::<i32>()
    }
}

fn main() {
    dotenv::dotenv().expect("Failed to load .env");
    let input = aoc_helper::input::get_input(2020, 1).expect("Failed to get input");
    aoc_helper::run(
        &input,
        day01::parse,
        vec![("part_1", day01::part_1), ("part_2", day01::part_2)],
    )
}