use advent_of_code::{
    grid::{Grid, GridPoint, PLUS_ADJACENT},
    parse::{parsers, Parser},
};

use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Grid<u32> {
    parsers::chars(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .grid("", "\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut trail_locations: HashSet<GridPoint<usize>> = map
        .iter_points()
        .filter_map(|point| if map[point] == 0 { Some(point) } else { None })
        .collect();
    let mut starting_points: HashMap<GridPoint<usize>, HashSet<GridPoint<usize>>> = HashMap::new();
    for start in trail_locations.iter() {
        starting_points.entry(*start).or_default().insert(*start);
    }
    for level in 0..9 {
        let mut next_trail_locations: HashSet<GridPoint<usize>> = HashSet::new();
        for trail_location in trail_locations {
            let current_starting_points = starting_points.get(&trail_location).unwrap().clone();
            for direction in PLUS_ADJACENT {
                if let Some(candidate_location) =
                    trail_location.add_checked(direction, &map.dimensions())
                    && map[candidate_location] == level + 1
                {
                    next_trail_locations.insert(candidate_location);
                    starting_points
                        .entry(candidate_location)
                        .or_default()
                        .extend(current_starting_points.clone())
                }
            }
        }
        trail_locations = next_trail_locations;
    }
    trail_locations
        .into_iter()
        .filter_map(|loc| starting_points.get(&loc).map(|s| s.len()))
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    let map = parse(input);
    let mut trail_locations: HashSet<GridPoint<usize>> = map
        .iter_points()
        .filter_map(|point| if map[point] == 0 { Some(point) } else { None })
        .collect();
    let mut trail_counts: HashMap<GridPoint<usize>, u64> = HashMap::new();
    for start in trail_locations.iter() {
        (*trail_counts.entry(*start).or_default()) += 1;
    }
    for level in 0..9 {
        let mut next_trail_locations: HashSet<GridPoint<usize>> = HashSet::new();
        for trail_location in trail_locations {
            for direction in PLUS_ADJACENT {
                if let Some(candidate_location) =
                    trail_location.add_checked(direction, &map.dimensions())
                    && map[candidate_location] == level + 1
                {
                    next_trail_locations.insert(candidate_location);
                    (*trail_counts.entry(candidate_location).or_default()) +=
                        trail_counts[&trail_location];
                }
            }
        }
        trail_locations = next_trail_locations;
    }
    trail_locations
        .into_iter()
        .map(|loc| trail_counts[&loc])
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    const DAY: Day = Day::Day10;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 81);
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
            786
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
            1722
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
