mod solution;
use solution::{Day8, Result, Solution};

type Answer = solution::Answer;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<Answer> {
    Day8::part1(divan::black_box(include_str!("../../../inputs/day8.txt")))
}

#[divan::bench]
fn bench_part2() -> Result<Answer> {
    Day8::part2(divan::black_box(include_str!("../../../inputs/day8.txt")))
}