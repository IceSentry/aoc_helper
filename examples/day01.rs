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
