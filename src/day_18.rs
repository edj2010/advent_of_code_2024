use std::collections::HashSet;

use advent_of_code::{
    grid::{GridDimensions, GridPoint, PLUS_ADJACENT},
    parse::{parsers, Parser},
    search::{WeightedGraph, WeightedGraphWithHeuristic},
};

fn parse(input: &str) -> Vec<GridPoint<u64>> {
    parsers::number()
        .pair(",", parsers::number())
        .map(|(col, row)| GridPoint::new(row, col))
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .collect()
}

struct Game(HashSet<GridPoint<u64>>, GridDimensions<u64>);

impl WeightedGraph for Game {
    type Key = GridPoint<u64>;
    type Cost = u64;
    fn adjacent(&self, k: &GridPoint<u64>) -> Option<impl Iterator<Item = GridPoint<u64>>> {
        Some(
            PLUS_ADJACENT
                .into_iter()
                .filter_map(|delta| k.add_checked(delta, &self.1))
                .filter(|p| !self.0.contains(p)),
        )
    }

    fn cost(&self, a: &GridPoint<u64>, b: &GridPoint<u64>) -> Option<u64> {
        if PLUS_ADJACENT.contains(&a.sub(*b).unwrap()) {
            Some(1)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &str, width: u64, height: u64, allowed_blocks: usize) -> u64 {
    let game = Game(
        parse(input).into_iter().take(allowed_blocks).collect(),
        GridDimensions::new(0, height, 0, width),
    );
    game.shortest_distance_with_condition(
        GridPoint::new(0, 0),
        |&k, _, _| k == GridPoint::new(height - 1, width - 1),
        0,
    )
    .unwrap()
    .1
}

#[allow(dead_code)]
pub fn part2(input: &str, width: u64, height: u64) -> String {
    let all_blocks = parse(input);
    let grid_dimensions = GridDimensions::new(0, height, 0, width);
    let mut min_idx = 0;
    let mut max_idx = all_blocks.len();
    while min_idx + 1 < max_idx {
        let candidate = (min_idx + max_idx) / 2;
        if let Some(_) = Game(
            all_blocks[..candidate].iter().cloned().collect(),
            grid_dimensions,
        )
        .shortest_distance_with_condition(
            GridPoint::new(0, 0),
            |&k, _, _| k == GridPoint::new(height - 1, width - 1),
            0,
        ) {
            min_idx = candidate;
        } else {
            max_idx = candidate;
        }
    }
    let terminal_block = all_blocks[min_idx];
    format!("{},{}", terminal_block.col, terminal_block.row)
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
    const DAY: Day = Day::Day18;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE, 7, 7, 12), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE, 7, 7), "6,1");
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                71,
                71,
                1024
            ),
            454
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            part2(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                71,
                71,
            ),
            "8,51"
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                71,
                71,
                1024,
            )
        });
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| {
            part2(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                71,
                71,
            )
        });
    }
}
