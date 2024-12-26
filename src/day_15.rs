use advent_of_code::{
    grid::{Direction, Grid, GridPoint},
    parse::{parsers, Parser},
};

use std::{
    collections::{HashSet, VecDeque},
    vec,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Wall,
    Box,
    Robot,
}

fn parse1(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    parsers::char_map(|c| match c {
        '.' => Some(Cell::Empty),
        'O' => Some(Cell::Box),
        '#' => Some(Cell::Wall),
        '@' => Some(Cell::Robot),
        _ => None,
    })
    .grid("", "\n")
    .pair(
        "\n",
        parsers::char_map(|c| match c {
            '^' => Some(Direction::North),
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            _ => None,
        })
        .many()
        .many_lines("\n")
        .map(|i| i.flatten().collect()),
    )
    .parse(input)
    .finish()
    .expect("Failed to parse input")
}

fn parse2(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    parsers::char_map(|c| match c {
        '.' => Some([Cell::Empty, Cell::Empty]),
        'O' => Some([Cell::Box, Cell::Empty]),
        '#' => Some([Cell::Wall, Cell::Wall]),
        '@' => Some([Cell::Robot, Cell::Empty]),
        _ => None,
    })
    .many_at_least_one()
    .map(|i| i.flatten().collect::<Vec<Cell>>())
    .many_lines("\n")
    .map(|i| Grid::of_vec_of_vecs(i.collect()).unwrap())
    .pair(
        "\n",
        parsers::char_map(|c| match c {
            '^' => Some(Direction::North),
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            _ => None,
        })
        .many()
        .many_lines("\n")
        .map(|i| i.flatten().collect()),
    )
    .parse(input)
    .finish()
    .expect("Failed to parse input")
}

fn attempt_move(
    robot: GridPoint<usize>,
    direction: Direction,
    grid: &mut Grid<Cell>,
) -> GridPoint<usize> {
    if robot
        .traverse_by(direction.into(), grid.dimensions())
        .skip(1)
        .take_while(|p| grid.get(*p) != Ok(&Cell::Empty))
        .all(|p| grid.get(p) != Ok(&Cell::Wall))
    {
        let first_empty = robot
            .traverse_by(direction.into(), grid.dimensions())
            .skip(1)
            .find(|p| grid.get(*p) == Ok(&Cell::Empty))
            .unwrap();
        let new_location = robot
            .add_checked(direction.into(), &grid.dimensions())
            .unwrap();
        grid.set(first_empty, Cell::Box).unwrap();
        grid.set(new_location, Cell::Empty).unwrap();
        new_location
    } else {
        robot
    }
}

fn attempt_move2(
    robot: GridPoint<usize>,
    direction: Direction,
    grid: &mut Grid<Cell>,
) -> GridPoint<usize> {
    let new_robot = robot + direction.into();
    if grid.get(new_robot) == Ok(&Cell::Wall) {
        return robot;
    }
    let impacted_boxes: Vec<GridPoint<usize>> = match direction {
        Direction::East => new_robot
            .traverse_by(direction.into(), grid.dimensions())
            .step_by(2)
            .take_while(|p| grid.get(*p) == Ok(&Cell::Box))
            .collect(),
        Direction::West => new_robot
            .traverse_by(direction.into(), grid.dimensions())
            .skip(1)
            .step_by(2)
            .take_while(|p| grid.get(*p) == Ok(&Cell::Box))
            .collect(),
        Direction::North | Direction::South => {
            let mut candidate_spots =
                VecDeque::from([new_robot, new_robot + Direction::West.into()]);
            let mut seen = HashSet::new();
            let mut boxes = HashSet::new();
            while let Some(spot) = candidate_spots.pop_front() {
                if seen.contains(&spot) {
                    continue;
                }
                seen.insert(spot);

                if grid.get(spot) == Ok(&Cell::Box) {
                    boxes.insert(spot);
                    candidate_spots.push_back(spot + direction.into());
                    candidate_spots.push_back(spot + direction.into() + Direction::East.into());
                    candidate_spots.push_back(spot + direction.into() + Direction::West.into());
                }
            }
            boxes.into_iter().collect()
        }
    };
    if impacted_boxes.iter().any(|&b| {
        (match direction {
            Direction::West => vec![b + direction.into()],
            Direction::East => vec![b + direction.into() + direction.into()],
            _ => {
                vec![
                    b + direction.into(),
                    b + direction.into() + Direction::East.into(),
                ]
            }
        })
        .into_iter()
        .any(|b| grid.get(b) == Ok(&Cell::Wall))
    }) {
        robot
    } else {
        let new_boxes: Vec<GridPoint<usize>> = impacted_boxes
            .iter()
            .map(|&b| b + direction.into())
            .collect();
        for old_box in impacted_boxes {
            grid.set(old_box, Cell::Empty).unwrap();
        }
        for new_box in new_boxes {
            grid.set(new_box, Cell::Box).unwrap();
        }
        new_robot
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (mut grid, instructions) = parse1(input);
    let mut robot = grid
        .iter_points()
        .find(|p| grid.get(*p) == Ok(&Cell::Robot))
        .unwrap();
    grid.set(robot, Cell::Empty).unwrap();
    for direction in instructions {
        robot = attempt_move(robot, direction, &mut grid);
    }
    grid.iter_points()
        .filter_map(|p| match grid.get(p) {
            Ok(Cell::Box) => Some(p.row * 100 + p.col),
            _ => None,
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let (mut grid, instructions) = parse2(input);
    let mut robot = grid
        .iter_points()
        .find(|p| grid.get(*p) == Ok(&Cell::Robot))
        .unwrap();
    grid.set(robot, Cell::Empty).unwrap();
    for direction in instructions {
        robot = attempt_move2(robot, direction, &mut grid);
    }
    grid.iter_points()
        .filter_map(|p| match grid.get(p) {
            Ok(Cell::Box) => Some(p.row * 100 + p.col),
            _ => None,
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

    const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    const DAY: Day = Day::Day15;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 9021);
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
            1563092
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
            1582688
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
