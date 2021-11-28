mod day01 {
    use itertools::Itertools;
    use serde_scan::scan;

    type Data = Vec<i32>;

    fn find_sum(input: &[i32], n: usize) -> Vec<i32> {
        input
            .iter()
            .copied()
            .combinations(n)
            .find(|x| x.iter().sum::<i32>() == 2020)
            .expect("There should be a valid combination")
    }

    pub fn parse(input: &str) -> Data {
        input.lines().map(|l| scan!("{}" <- l).unwrap()).collect()
    }

    #[allow(clippy::ptr_arg)]
    pub fn part_1(input: &Data) -> i32 {
        find_sum(input, 2).iter().product::<i32>()
    }

    #[allow(clippy::ptr_arg)]
    pub fn part_2(input: &Data) -> i32 {
        find_sum(input, 3).iter().product::<i32>()
    }
}

fn main() {
    dotenv::dotenv().expect("Failed to load .env");
    let input = aoc_helper::input::get_input(2020, 1).expect("Failed to get input");
    let answer = aoc_helper::run(
        &input,
        day01::parse,
        vec![("part_1", day01::part_1), ("part_2", day01::part_2)],
    );
    // aoc_helper::input::submit(2020, 1, 1, &answer.unwrap().to_string());
}
