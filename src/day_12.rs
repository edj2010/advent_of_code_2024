use advent_of_code::{
    grid::{Direction, Grid, GridPoint},
    parse::{parsers, Parser},
};

use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> Grid<char> {
    parsers::chars(|c| c.is_alphabetic())
        .grid("", "\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let grid = parse(input);
    let mut seen = HashSet::new();
    let mut to_explore = VecDeque::from([GridPoint::new(0, 0)]);
    let mut total = 0;
    while let Some(seed) = to_explore.pop_front() {
        if seen.contains(&seed) {
            continue;
        }
        seen.insert(seed);
        let mut within_region = VecDeque::from([seed]);
        let mut seen_within_region = HashSet::new();
        let mut perimeter = 0_u32;
        let mut area = 0_u32;
        while let Some(current) = within_region.pop_front() {
            if seen_within_region.contains(&current) {
                continue;
            }
            seen_within_region.insert(current);
            area += 1;
            for delta in Direction::all() {
                if let Some(next) = current.add_checked(delta.into(), &grid.dimensions()) {
                    if grid.get(next) == grid.get(current) {
                        within_region.push_back(next);
                    } else {
                        perimeter += 1;
                        to_explore.push_back(next);
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        seen.extend(seen_within_region);
        total += perimeter * area;
    }
    total
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let grid = parse(input);
    let mut seen = HashSet::new();
    let mut to_explore = VecDeque::from([GridPoint::new(0, 0)]);
    let mut total = 0;
    while let Some(seed) = to_explore.pop_front() {
        if seen.contains(&seed) {
            continue;
        }
        seen.insert(seed);
        let mut within_region = VecDeque::from([seed]);
        let mut seen_within_region = HashSet::new();
        let mut perimeter = 0_u32;
        let mut area = 0_u32;
        while let Some(current) = within_region.pop_front() {
            if seen_within_region.contains(&current) {
                continue;
            }
            seen_within_region.insert(current);
            area += 1;
            for delta in Direction::all() {
                if let Some(next) = current.add_checked(delta.into(), &grid.dimensions()) {
                    if grid.get(next) == grid.get(current) {
                        within_region.push_back(next);
                    } else {
                        let right = delta.rotate_right();
                        let right_cell = current.add_checked(right.into(), &grid.dimensions());
                        let right_next_cell = next.add_checked(right.into(), &grid.dimensions());
                        if !(right_cell.and_then(|c| grid.get(c).ok()) == grid.get(current).ok()
                            && right_next_cell.and_then(|c| grid.get(c).ok())
                                != grid.get(current).ok())
                        {
                            perimeter += 1;
                        }
                        to_explore.push_back(next);
                    }
                } else {
                    if current
                        .add_checked(delta.rotate_right().into(), &grid.dimensions())
                        .and_then(|c| grid.get(c).ok())
                        != grid.get(current).ok()
                    {
                        perimeter += 1;
                    }
                }
            }
        }
        seen.extend(seen_within_region);
        total += perimeter * area;
    }
    total
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    const DAY: Day = Day::Day12;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 1206);
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
            1449902
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
            908042
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
