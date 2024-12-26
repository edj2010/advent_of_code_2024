use advent_of_code::{
    grid::{GridDimensions, GridPoint, GridPointDelta},
    itertools::Itertools,
    parse::{parsers, Parser},
};

use std::collections::{HashMap, HashSet};

fn parse(
    input: &str,
) -> (
    GridDimensions<usize>,
    HashMap<char, HashSet<GridPoint<usize>>>,
) {
    parsers::tag_replace(".", None)
        .or(parsers::chars(|c| c.is_alphanumeric()).map(|c: char| Some(c)))
        .grid("", "\n")
        .map(|grid| {
            (
                grid.dimensions(),
                grid.iter_points().fold(
                    HashMap::new(),
                    |mut antenna: HashMap<char, HashSet<GridPoint<usize>>>, point| {
                        if let Ok(Some(frequency)) = grid.get(point) {
                            antenna.entry(*frequency).or_default().insert(point);
                        }
                        antenna
                    },
                ),
            )
        })
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (grid_dimensions, antenna) = parse(input);
    antenna
        .into_iter()
        .flat_map(|(_, locations)| {
            locations
                .iter()
                .ordered_pairs()
                .filter_map(|(&a, &b)| {
                    if a == b {
                        return None;
                    }
                    let delta: GridPointDelta<isize> = b.sub(a)?;
                    b.add_checked(delta, &grid_dimensions)
                })
                .collect::<Vec<GridPoint<usize>>>()
        })
        .collect::<HashSet<GridPoint<usize>>>()
        .len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let (grid_dimensions, antenna) = parse(input);
    antenna
        .into_iter()
        .flat_map(|(_, locations)| {
            locations
                .iter()
                .ordered_pairs()
                .filter_map(|(&a, &b)| {
                    if a == b {
                        return None;
                    }
                    let delta: GridPointDelta<isize> = b.sub(a)?.min_step();
                    Some(b.traverse_by(delta, grid_dimensions))
                })
                .flatten()
                .collect::<Vec<GridPoint<usize>>>()
        })
        .collect::<HashSet<GridPoint<usize>>>()
        .len()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    const DAY: Day = Day::Day08;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 34);
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
            273
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
            1017
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
