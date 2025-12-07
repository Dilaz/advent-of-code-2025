mod solution;
use solution::{DayX, Result, Solution};

type Answer = solution::Answer;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<Answer> {
    DayX::part1(divan::black_box(include_str!("../../../inputs/dayX.txt")))
}

#[divan::bench]
fn bench_part2() -> Result<Answer> {
    DayX::part2(divan::black_box(include_str!("../../../inputs/dayX.txt")))
}