use advent_of_code::parse::{parsers, Parser};

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32>> {
    parsers::number()
        .list(" ")
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

fn is_good_ascending(input: &[u32]) -> bool {
    input.iter().is_sorted()
        && input
            .iter()
            .zip(input.iter().skip(1))
            .all(|(a, b)| 1 <= a.abs_diff(*b) && a.abs_diff(*b) <= 3)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    parse(input)
        .filter_map(|report| {
            let report: Vec<u32> = report.collect();
            let reverse_report: Vec<u32> = report.iter().rev().copied().collect();
            if is_good_ascending(&report) || is_good_ascending(&reverse_report) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    parse(input)
        .filter_map(|report| {
            let report: Vec<u32> = report.collect();
            let reverse_report: Vec<u32> = report.iter().rev().copied().collect();
            if is_good_ascending(&report)
                || is_good_ascending(&reverse_report)
                || (0..report.len()).any(|idx| {
                    is_good_ascending(&[&report[..idx], &report[idx + 1..]].concat())
                        || is_good_ascending(
                            &[&reverse_report[..idx], &reverse_report[idx + 1..]].concat(),
                        )
                })
            {
                Some(())
            } else {
                None
            }
        })
        .count()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    const DAY: Day = Day::Day02;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 4);
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
            356
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
