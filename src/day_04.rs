use advent_of_code::{
    grid::{Grid, ADJACENT, NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST},
    parse::{parsers, Parser},
};

fn parse(input: &str) -> Grid<char> {
    parsers::chars(|c| c.is_alphabetic())
        .grid("", "\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    grid.iter_points()
        .map(|point| {
            ADJACENT
                .into_iter()
                .filter(|direction| {
                    point
                        .traverse_by(*direction, grid.dimensions())
                        .take(4)
                        .filter_map(|p| grid.get(p).ok())
                        .collect::<String>()
                        == "XMAS"
                })
                .count()
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let grid = parse(input);
    grid.iter_points()
        .filter(|point| {
            let myself = grid.get(*point).ok().unwrap();
            let corners: String = [
                point.add_checked(NORTHWEST, &grid.dimensions()),
                point.add_checked(NORTHEAST, &grid.dimensions()),
                point.add_checked(SOUTHEAST, &grid.dimensions()),
                point.add_checked(SOUTHWEST, &grid.dimensions()),
            ]
            .into_iter()
            .filter_map(|p| p.and_then(|p| grid.get(p).ok()))
            .collect();
            *myself == 'A'
                && (corners == "MMSS"
                    || corners == "SMMS"
                    || corners == "SSMM"
                    || corners == "MSSM")
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

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    const DAY: Day = Day::Day04;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 9);
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
            2483
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
            1925
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
