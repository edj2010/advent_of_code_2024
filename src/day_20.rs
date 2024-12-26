use std::collections::HashMap;

use advent_of_code::{
    grid::{Grid, GridPoint, GridPointDelta, Maze, MazeCell},
    parse::{parsers, Parser},
    search::WeightedGraphWithHeuristic,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// struct Race(Grid<MazeCell>);

// impl WeightedGraph for Race {
//     type Key = GridPoint<usize>;
//     type Cost = usize;

//     fn adjacent(&self, k: &Self::Key) -> Option<impl Iterator<Item = Self::Key>> {
//         Some(
//             PLUS_ADJACENT
//                 .into_iter()
//                 .filter_map(|d| k.add_checked(d, &self.0.dimensions()))
//                 .filter(|k| self.0.get(*k) != Ok(&Cell::Wall)),
//         )
//     }

//     fn cost(&self, a: &Self::Key, b: &Self::Key) -> Option<Self::Cost> {
//         a.sub::<isize>(*b).map(|d| d.l1_norm() as usize)
//     }
// }

fn nearby_cells(distance: isize) -> impl Iterator<Item = GridPointDelta<isize>> {
    (-distance..=distance).flat_map(move |row_delta| {
        (-distance..=distance).filter_map(move |col_delta| {
            if (row_delta.abs() + col_delta.abs()) <= distance {
                Some(GridPointDelta::new(row_delta, col_delta))
            } else {
                None
            }
        })
    })
}

#[allow(dead_code)]
pub fn part1(input: &str, save_count: u64) -> usize {
    let (grid, special) = parse(input);
    let start_position = special.get(&Special::Start).unwrap()[0];
    let end_position = special.get(&Special::End).unwrap()[0];
    let maze = Maze::of_grid_with_end(
        grid,
        |from_loc, to_loc| (to_loc.sub::<i64>(from_loc).unwrap().l1_norm() as u64),
        end_position,
    );

    let shortest_paths = maze
        .shortest_paths_to_many(start_position, |_, _, _| false, 0)
        .0;
    shortest_paths
        .shortest_path(&end_position)
        .into_iter()
        .filter_map(|current| {
            let this_cost = shortest_paths.shortest_cost(&current)?;
            Some(
                nearby_cells(2)
                    .filter(|delta| {
                        current
                            .add_checked(*delta, &maze.dimensions())
                            .and_then(|point| shortest_paths.shortest_cost(&point))
                            .map(|new_cost| {
                                *new_cost >= this_cost + (delta.l1_norm() as u64) + save_count
                            })
                            .unwrap_or(false)
                    })
                    .count(),
            )
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str, save_count: u64) -> usize {
    let (grid, special) = parse(input);
    let start_position = special.get(&Special::Start).unwrap()[0];
    let end_position = special.get(&Special::End).unwrap()[0];
    let maze = Maze::of_grid_with_end(
        grid,
        |from_loc, to_loc| (to_loc.sub::<i64>(from_loc).unwrap().l1_norm() as u64),
        end_position,
    );

    let shortest_paths = maze
        .shortest_paths_to_many(start_position, |_, _, _| false, 0)
        .0;
    shortest_paths
        .shortest_path(&end_position)
        .into_iter()
        .filter_map(|current| {
            let this_cost = shortest_paths.shortest_cost(&current)?;
            Some(
                nearby_cells(20)
                    .filter(|delta| {
                        current
                            .add_checked(*delta, &maze.dimensions())
                            .and_then(|point| shortest_paths.shortest_cost(&point))
                            .map(|new_cost| {
                                *new_cost >= this_cost + (delta.l1_norm() as u64) + save_count
                            })
                            .unwrap_or(false)
                    })
                    .count(),
            )
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

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    const DAY: Day = Day::Day20;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE, 20), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE, 50), 285);
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                100
            ),
            1422
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                100
            ),
            1009299
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                20,
            )
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| {
            part2(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                100,
            )
        });
    }
}
