use std::collections::{HashMap, HashSet};

use advent_of_code::{
    grid::{Direction, Grid, GridPoint, MazeCell, MazeWithTurningCost},
    parse::{parsers, Parser},
    search::WeightedGraphWithHeuristic,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Special {
    Start,
    End,
}

fn maybe_from_char(c: char) -> Option<(MazeCell, Option<Special>)> {
    match c {
        'S' => Some((MazeCell::Empty, Some(Special::Start))),
        'E' => Some((MazeCell::Empty, Some(Special::End))),
        '.' => Some((MazeCell::Empty, None)),
        '#' => Some((MazeCell::Wall, None)),
        _ => None,
    }
}

fn parse(input: &str) -> (Grid<MazeCell>, HashMap<Special, Vec<GridPoint<usize>>>) {
    parsers::char_map(maybe_from_char)
        .grid_with_special_cells("", "\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u64 {
    let (grid, special) = parse(input);
    let start_position = special.get(&Special::Start).unwrap()[0];
    let end_position = special.get(&Special::End).unwrap()[0];
    let start_direction = Direction::East;
    let maze = MazeWithTurningCost::of_grid_with_end(
        grid,
        |(from_loc, from_dir), (to_loc, to_dir)| {
            (to_loc.sub::<i64>(from_loc).unwrap().l1_norm() as u64)
                + (if from_dir == to_dir {
                    0
                } else if from_dir.rotate_right() == to_dir || from_dir.rotate_left() == to_dir {
                    1000
                } else {
                    2000
                })
        },
        end_position,
    );

    maze.shortest_distance_with_condition(
        (start_position, start_direction),
        |(position, _), _, _| *position == end_position,
        0,
    )
    .unwrap()
    .1
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let (grid, special) = parse(input);
    let start_position = special.get(&Special::Start).unwrap()[0];
    let end_position = special.get(&Special::End).unwrap()[0];
    let start_direction = Direction::East;
    let maze = MazeWithTurningCost::of_grid_with_end(
        grid,
        |(from_loc, from_dir), (to_loc, to_dir)| {
            (to_loc.sub::<i64>(from_loc).unwrap().l1_norm() as u64)
                + (if from_dir == to_dir {
                    0
                } else if from_dir.rotate_right() == to_dir || from_dir.rotate_left() == to_dir {
                    1000
                } else {
                    2000
                })
        },
        end_position,
    );

    let shortest_paths_precedent_map = maze
        .shortest_paths_to_many((start_position, start_direction), |_, _, _| false, 0)
        .0;

    let min_weight = Direction::all()
        .into_iter()
        .filter_map(|direction| {
            shortest_paths_precedent_map.shortest_cost(&(end_position, direction))
        })
        .min()
        .unwrap();

    Direction::all()
        .into_iter()
        .filter_map(|direction| {
            let key = (end_position, direction);
            if min_weight == shortest_paths_precedent_map.shortest_cost(&key)? {
                Some(
                    shortest_paths_precedent_map
                        .all_precedents(&key)
                        .into_iter(),
                )
            } else {
                None
            }
        })
        .flatten()
        .map(|(position, _)| position)
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

    const EXAMPLE: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
    const DAY: Day = Day::Day16;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 64);
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
            143580
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
            645
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
