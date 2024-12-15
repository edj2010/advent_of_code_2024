use advent_of_code::parse::{parsers, Parser};

#[derive(Clone, Copy)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
    Noop,
}

fn parse(input: &str) -> impl Iterator<Item = Instruction> {
    parsers::tag("mul(")
        .ignore_and_then(parsers::number())
        .pair(",", parsers::number())
        .skip_tag(")")
        .map(|(a, b)| Instruction::Mul(a, b))
        .or(parsers::tag_replace("do()", Instruction::Do))
        .or(parsers::tag_replace("don't()", Instruction::Dont))
        .or(parsers::char_any().map(|_| Instruction::Noop))
        .many()
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parse(input)
        .map(|pair| match pair {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parse(input)
        .fold((true, 0), |(enabled, acc), pair| match pair {
            Instruction::Mul(a, b) => {
                if enabled {
                    (enabled, acc + a * b)
                } else {
                    (enabled, acc)
                }
            }
            Instruction::Do => (true, acc),
            Instruction::Dont => (false, acc),
            _ => (enabled, acc),
        })
        .1
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const DAY: Day = Day::Day03;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
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
            175700056
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
            71668682
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
