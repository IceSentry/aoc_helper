mod day01;

#[allow(clippy::type_complexity)]
fn main() {
    dotenvy::dotenv().expect("Failed to load .env");
    let input = aoc_helper::input::get_input(2020, 1).expect("Failed to get input");
    let solutions: Vec<(&str, fn(&_) -> _)> =
        vec![("part_1", day01::part_1), ("part_2", day01::part_2)];
    let _answer = aoc_helper::run_single_day(&input, day01::parse, &solutions);
}
