use advent_of_code::{
    grid::{Direction, Grid, GridDimensions, GridPoint},
    parse::{parsers, Parser},
};

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GuardState {
    position: GridPoint<usize>,
    direction: Direction,
}

impl GuardState {
    fn step(mut self, grid_dimensions: GridDimensions<usize>) -> Option<Self> {
        self.position
            .add_checked(self.direction.into(), &grid_dimensions)
            .map(|new_position| {
                self.position = new_position;
                self
            })
    }

    fn rotate(&mut self) {
        self.direction = self.direction.rotate_right();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Guard(Direction),
}

fn parse(input: &str) -> Grid<Cell> {
    parsers::tag_replace(".", Cell::Empty)
        .or(parsers::tag_replace("#", Cell::Wall))
        .or(parsers::tag_replace("^", Cell::Guard(Direction::North)))
        .or(parsers::tag_replace(">", Cell::Guard(Direction::East)))
        .or(parsers::tag_replace("<", Cell::Guard(Direction::West)))
        .or(parsers::tag_replace("v", Cell::Guard(Direction::South)))
        .many()
        .map(|i| i.collect::<Vec<Cell>>())
        .many_lines("\n")
        .map(|i| Grid::of_vec_of_vecs(i.collect::<Vec<Vec<Cell>>>()).unwrap())
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

// Returns Some(visited cells) if the guard exits, or None if it gets caught in a loop
fn simulate(grid: &Grid<Cell>, mut guard_state: GuardState) -> Option<HashSet<GuardState>> {
    let mut visited_positions: HashSet<GuardState> = HashSet::new();
    visited_positions.insert(guard_state);
    while let Some(next_state) = guard_state.step(grid.dimensions()) {
        if grid.get(next_state.position) == Ok(&Cell::Wall) {
            guard_state.rotate();
        } else {
            guard_state = next_state;
        }
        if visited_positions.contains(&guard_state) {
            return None;
        }
        visited_positions.insert(guard_state);
    }
    Some(visited_positions)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let grid = parse(input);
    let guard_state = grid
        .iter_points()
        .find_map(|position| match grid.get(position) {
            Ok(Cell::Guard(direction)) => Some(GuardState {
                position,
                direction: *direction,
            }),
            _ => None,
        })
        .unwrap();
    simulate(&grid, guard_state)
        .unwrap()
        .into_iter()
        .map(|guard_state| guard_state.position)
        .collect::<HashSet<GridPoint<usize>>>()
        .len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let grid = parse(input);
    let initial_guard_state = grid
        .iter_points()
        .find_map(|position| match grid.get(position) {
            Ok(Cell::Guard(direction)) => Some(GuardState {
                position,
                direction: *direction,
            }),
            _ => None,
        })
        .unwrap();
    simulate(&grid, initial_guard_state)
        .unwrap()
        .into_iter()
        .filter_map(|guard_state| {
            // consider adding a block right in front of the guard
            let next_state = guard_state.step(grid.dimensions())?;
            if grid.get(next_state.position) != Ok(&Cell::Wall) {
                let mut new_grid = grid.clone();
                new_grid.set(next_state.position, Cell::Wall).unwrap();
                if simulate(&new_grid, initial_guard_state).is_none() {
                    return Some(next_state.position);
                }
            }
            None
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

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    const DAY: Day = Day::Day06;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 6);
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
            5067
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
            1793
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
