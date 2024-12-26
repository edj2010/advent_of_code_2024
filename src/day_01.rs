use advent_of_code::{itertools::Itertools, parse::{parsers, Parser}};

fn parse(input: &str) -> Vec<(u32,u32)> {
    parsers::number()
    .pair("   ",parsers::number())
    .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .collect()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (mut list_a, mut list_b): (Vec<u32>, Vec<u32>) = parse(input).into_iter().unzip();
    list_a.sort();
    list_b.sort();
    list_a.into_iter().zip(list_b.into_iter()).map(|(a, b)| a.abs_diff(b)).sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let (list_a, list_b): (Vec<u32>, Vec<u32>) = parse(input).into_iter().unzip();
    let count_b = list_b.into_iter().value_counts();
    list_a.into_iter().fold(0, |acc, n| acc + n * count_b.get(&n).unwrap_or(&0))
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";
    const DAY: Day = Day::Day01;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 31);
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
            2344935
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
            27647262
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
