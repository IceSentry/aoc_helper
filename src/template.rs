type Data = Vec<i32>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[allow(clippy::ptr_arg)]
pub fn part_1(input: &Data) -> usize {
    0
}

#[allow(clippy::ptr_arg)]
pub fn part_2(input: &Data) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 1);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 2);
    }
}
