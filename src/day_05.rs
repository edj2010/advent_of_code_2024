use advent_of_code::parse::{parsers, Parser};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn parse(
    input: &str,
) -> (
    HashMap<u32, HashSet<u32>>,
    impl Iterator<Item = impl Iterator<Item = u32>>,
) {
    parsers::number()
        .pair("|", parsers::number())
        .many_lines("\n")
        .map(|rules| {
            let mut rule_map: HashMap<u32, HashSet<u32>> = HashMap::new();
            rules.for_each(|(before, after)| {
                rule_map.entry(before).or_default().insert(after);
            });
            rule_map
        })
        .pair("\n", parsers::number().list(",").many_lines("\n"))
        .parse(input)
        .finish()
        .expect("Failed to parse input")
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (rules, candidates) = parse(input);
    candidates
        .map(|p| p.collect::<Vec<u32>>())
        .filter(|pages| {
            let mut seen: HashSet<u32> = HashSet::new();
            for page in pages.iter() {
                if match rules.get(page) {
                    Some(rule) => rule.intersection(&seen).count() > 0,
                    None => false,
                } {
                    return false;
                }
                seen.insert(*page);
            }
            true
        })
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let (rules, candidates) = parse(input);
    candidates
        .map(|p| p.collect::<Vec<u32>>())
        .filter(|pages| {
            let mut seen: HashSet<u32> = HashSet::new();
            for page in pages.iter() {
                if match rules.get(page) {
                    Some(rule) => rule.intersection(&seen).count() > 0,
                    None => false,
                } {
                    return true;
                }
                seen.insert(*page);
            }
            false
        })
        .map(|mut pages| {
            pages.sort_by(|a, b| {
                if rules.get(a).map(|after| after.contains(b)).unwrap_or(false) {
                    Ordering::Less
                } else if rules.get(b).map(|after| after.contains(a)).unwrap_or(false) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            pages
        })
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use advent_of_code::{day::Day, web_api::load_question_input};
    use test::Bencher;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    const DAY: Day = Day::Day05;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 123);
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
            6051
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
            5093
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
