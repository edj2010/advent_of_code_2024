use std::collections::HashSet;

use advent_of_code::parse::{parsers, Parser};

fn parse(input: &str) -> impl Iterator<Item = (u64, impl Iterator<Item = u64>)> {
    parsers::number()
        .skip_tag(": ")
        .and_then(parsers::number().list(" "))
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u64 {
    parse(input)
        .filter_map(|(target, mut components)| {
            let first = components.next().unwrap();
            if components
                .fold(HashSet::from([first]), |seen, next| {
                    seen.into_iter()
                        .flat_map(|prev| [prev + next, prev * next])
                        .collect()
                })
                .contains(&target)
            {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    parse(input)
        .filter_map(|(target, mut components)| {
            let first = components.next().unwrap();
            if components
                .fold(HashSet::from([first]), |seen, next| {
                    seen.into_iter()
                        .flat_map(|prev| {
                            let concat_scalar = 10_u64.pow(next.ilog10() + 1);
                            [prev + next, prev * next, prev * concat_scalar + next]
                        })
                        .collect()
                })
                .contains(&target)
            {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    const DAY: Day = Day::Day07;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 11387);
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
            538191549061
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
            34612812972206
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
