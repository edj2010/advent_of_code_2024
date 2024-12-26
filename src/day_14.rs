use advent_of_code::{
    grid::{Grid, GridPoint, GridPointDelta},
    parse::{parsers, Parser},
};
use std::io::stdin;
use std::{cmp::Ordering, io::BufRead};

struct Robot {
    position: GridPoint<i32>,
    delta: GridPointDelta<i32>,
}

impl Robot {
    fn future_position(&self, time: i32, width: i32, height: i32) -> GridPoint<i32> {
        let mut unwrapped_position = self.position + (self.delta * time);
        unwrapped_position.row = unwrapped_position.row.rem_euclid(height);
        unwrapped_position.col = unwrapped_position.col.rem_euclid(width);
        unwrapped_position
    }
}

fn parse(input: &str) -> Vec<Robot> {
    parsers::tag("p=")
        .ignore_and_then(
            parsers::number()
                .pair(",", parsers::number())
                .map(|(a, b)| GridPoint::new(b, a)),
        )
        .pair(
            " v=",
            parsers::signed_number()
                .pair(",", parsers::signed_number())
                .map(|(a, b)| GridPointDelta::new(b, a)),
        )
        .map(|(position, delta)| Robot { position, delta })
        .many_lines("\n")
        .parse(input)
        .finish()
        .expect("Failed to parse input")
        .collect()
}

#[allow(dead_code)]
pub fn part1(input: &str, width: i32, height: i32) -> i32 {
    let col_mid = width / 2;
    let row_mid = height / 2;
    let (tl, tr, bl, br) = parse(input)
        .into_iter()
        .map(|robot| robot.future_position(100, width, height))
        .fold((0, 0, 0, 0), |(tl, tr, bl, br), position| {
            match (position.row.cmp(&row_mid), position.col.cmp(&col_mid)) {
                (Ordering::Less, Ordering::Less) => (tl + 1, tr, bl, br),
                (Ordering::Greater, Ordering::Less) => (tl, tr, bl + 1, br),
                (Ordering::Less, Ordering::Greater) => (tl, tr + 1, bl, br),
                (Ordering::Greater, Ordering::Greater) => (tl, tr, bl, br + 1),
                _ => (tl, tr, bl, br),
            }
        });
    tl * tr * bl * br
}

#[allow(dead_code)]
pub fn part2(input: &str, width: i32, height: i32) -> i32 {
    let robots = parse(input);
    let mut to_step = String::new();
    let mut stdin_lock = stdin().lock();
    let mut time = 0;
    let mut step = 1;
    while let Ok(_) = stdin_lock.read_line(&mut to_step) {
        step = to_step.trim().parse::<i32>().unwrap_or(step);
        time += step;
        to_step = String::new();
        let positions: Vec<GridPoint<i32>> = robots
            .iter()
            .map(|robot| robot.future_position(time, width, height))
            .collect();
        let mut grid = Grid::init(" ", height as usize, width as usize);
        for position in positions {
            grid.set(position.try_as_type().unwrap(), "#").unwrap();
        }
        println!("{}", grid);
        println!("{}================================================================================================================================================================================================", time);
    }
    time
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    const DAY: Day = Day::Day14;

    // Part 2 is not trivially evaluatable. Answer is 7672

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE, 11, 7), 12);
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            part1(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                101,
                103
            ),
            230686500
        );
    }

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| {
            part1(
                &load_question_input(crate::YEAR, crate::COOKIE_PATH, crate::INPUT_CACHE, DAY),
                101,
                103,
            )
        });
    }
}
