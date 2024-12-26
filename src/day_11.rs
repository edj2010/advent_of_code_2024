use std::collections::HashMap;

use advent_of_code::{
    itertools::Itertools,
    parse::{parsers, Parser},
};

fn parse(input: &str) -> HashMap<u64, u64> {
    parsers::number()
        .list(" ")
        .map(|l| {
            l.value_counts()
                .into_iter()
                .map(|(k, v)| (k, v as u64))
                .collect()
        })
        .skip_tag("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

fn simulate_one(n: u64) -> Vec<u64> {
    if n == 0 {
        vec![1]
    } else if n.ilog10() % 2 == 1 {
        let mask = 10_u64.pow(n.ilog10() / 2 + 1);
        vec![n / mask, n % mask]
    } else {
        vec![n * 2024]
    }
}

fn simulate_unordered(arrangement: HashMap<u64, u64>, depth: u64) -> HashMap<u64, u64> {
    if depth == 0 {
        arrangement
    } else {
        let mut next_arrangement: HashMap<u64, u64> = HashMap::new();
        for (value, count) in arrangement {
            for next_value in simulate_one(value) {
                (*next_arrangement.entry(next_value).or_default()) += count;
            }
        }
        simulate_unordered(next_arrangement, depth - 1)
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u64 {
    simulate_unordered(parse(input), 25)
        .into_iter()
        .map(|(_, v)| v)
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    simulate_unordered(parse(input), 75)
        .into_iter()
        .map(|(_, v)| v)
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "125 17
";
    const DAY: Day = Day::Day11;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 65601038650482);
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            224529
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY
            )),
            266820198587914
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| {
            part2(&load_question_input(
                crate::YEAR,
                crate::COOKIE_PATH,
                crate::INPUT_CACHE,
                DAY,
            ))
        });
    }
}
