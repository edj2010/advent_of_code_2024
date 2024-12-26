#![feature(test)]
#![feature(let_chains)]

#[allow(unused_imports)]
use advent_of_code::{day::Day, web_api::load_question_input};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;

#[allow(dead_code)]
const YEAR: &str = "2024";

#[allow(dead_code)]
const COOKIE_PATH: &str = "../session.cookie";

#[allow(dead_code)]
const INPUT_CACHE: &str = "input";

fn main() {
    day_20::part1(
        &load_question_input(
            crate::YEAR,
            crate::COOKIE_PATH,
            crate::INPUT_CACHE,
            Day::Day20,
        ),
        100,
    );
}
