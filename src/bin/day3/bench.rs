mod solution;
use solution::{Day3, Result, Solution};

type Answer = solution::Answer;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<Answer> {
    Day3::part1(divan::black_box(include_str!("../../../inputs/day3.txt")))
}

#[divan::bench]
fn bench_part2() -> Result<Answer> {
    Day3::part2(divan::black_box(include_str!("../../../inputs/day3.txt")))
}
