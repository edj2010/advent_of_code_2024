use advent_of_code::{
    grid::{GridPoint, GridPointDelta},
    parse::{parsers, Parser},
};

struct Game {
    button_a: GridPointDelta<i64>,
    button_b: GridPointDelta<i64>,
    target: GridPoint<i64>,
}

impl Game {
    fn solution(&self) -> Option<(i64, i64)> {
        let determinant = self.button_a.row_delta * self.button_b.col_delta
            - self.button_a.col_delta * self.button_b.row_delta;
        let a_count =
            self.button_b.col_delta * self.target.row - self.button_b.row_delta * self.target.col;
        let b_count =
            self.button_a.row_delta * self.target.col - self.button_a.col_delta * self.target.row;
        if a_count % determinant == 0 && b_count % determinant == 0 {
            Some((a_count / determinant, b_count / determinant))
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Game> {
    parsers::tag("Button A: X")
        .ignore_and_then(
            parsers::signed_number()
                .pair(", Y", parsers::signed_number())
                .map(|(row_delta, col_delta)| GridPointDelta::new(row_delta, col_delta)),
        )
        .skip_tag("\nButton B: X")
        .and_then(
            (parsers::signed_number())
                .pair(", Y", parsers::signed_number())
                .map(|(row_delta, col_delta)| GridPointDelta::new(row_delta, col_delta)),
        )
        .skip_tag("\nPrize: X=")
        .and_then(
            parsers::signed_number()
                .skip_tag(", Y=")
                .and_then(parsers::signed_number())
                .skip_tag("\n")
                .map(|(row, col)| GridPoint::new(row, col)),
        )
        .map(|((button_a, button_b), target)| Game {
            button_a,
            button_b,
            target,
        })
        .list("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .collect()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|game| game.solution().map(|(a, b)| 3 * a + b).unwrap_or(0))
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|mut game| {
            game.target += GridPointDelta::new(10000000000000_i64, 10000000000000_i64);
            game.solution().map(|(a, b)| 3 * a + b).unwrap_or(0)
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

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    const DAY: Day = Day::Day13;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 875318608908);
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
            32041
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
            95843948914827
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
