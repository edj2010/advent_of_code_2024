use advent_of_code::{
    parse::{parsers, ParseState, Parser},
    search::{WeightedGraph, WeightedGraphWithHeuristic},
};

fn parse(input: &str) -> (Vec<String>, impl Iterator<Item = String>) {
    parsers::many_chars(|c| c.is_alphabetic())
        .list(", ")
        .map(|i| i.collect())
        .pair(
            "\n\n",
            parsers::many_chars(|c| c.is_alphabetic()).many_lines("\n"),
        )
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

struct Towels(Vec<String>);

impl WeightedGraph for Towels {
    type Key = String;
    type Cost = i32;

    fn adjacent(&self, k: &String) -> Option<impl Iterator<Item = String>> {
        Some(
            self.0
                .iter()
                .filter_map(|towel| match parsers::tag(towel).parse(k) {
                    ParseState::Ok { result: _, rest } => Some(rest.to_owned()),
                    _ => None,
                }),
        )
    }

    fn cost(&self, a: &String, b: &String) -> Option<i32> {
        <Self as WeightedGraph>::adjacent(self, a)?.find(|s| s == b)?;
        Some((a.len() as i32) - (b.len() as i32))
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (towels, targets) = parse(input);
    let towels = Towels(towels);
    targets
        .filter_map(|t| towels.shortest_distance_with_condition(t, |s, _, _| s == "", 0))
        .count()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u64 {
    let (towels, targets) = parse(input);
    let towels = Towels(towels);
    targets
        .filter_map(|t| Some(towels.shortest_path_count(t, "".to_owned(), 0)?))
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    const DAY: Day = Day::Day19;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 16);
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
            311
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
            616234236468263
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
