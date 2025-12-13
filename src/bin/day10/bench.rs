mod solution;
use solution::{Day10, Result, Solution};

type Answer = solution::Answer;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<Answer> {
    Day10::part1(divan::black_box(include_str!("../../../inputs/day10.txt")))
}

#[divan::bench]
fn bench_part2() -> Result<Answer> {
    Day10::part2(divan::black_box(include_str!("../../../inputs/day10.txt")))
}