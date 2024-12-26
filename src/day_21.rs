use std::collections::HashMap;

use advent_of_code::{
    grid::GridPoint,
    parse::{parsers, Parser},
};

fn parse(input: &str) -> impl Iterator<Item = String> {
    parsers::many_chars(|c| c.is_alphanumeric())
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

struct Keypad {
    buttons: HashMap<char, GridPoint<u32>>,
}

impl Keypad {
    fn new<I: Iterator<Item = (char, (u32, u32))>>(iter: I) -> Self {
        Keypad {
            buttons: iter
                .map(|(key, (row, col))| (key, GridPoint::new(row, col)))
                .collect(),
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|target| {
            println!("{}", target);
            0
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parse(input);
    0
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "";
    const DAY: Day = Day::Day21;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 0);
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
            0
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
            0
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
