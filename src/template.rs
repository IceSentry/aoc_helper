type Data = usize;

pub fn parse(input: &str) -> Vec<Data> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_1(input: &[Data]) -> usize {
    0
}

pub fn part_2(input: &[Data]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 0);
    }
}
