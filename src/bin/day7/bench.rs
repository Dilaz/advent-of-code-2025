mod solution;
use miette::Result;
use solution::Day7;
use solution::Solution;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<u64> {
    Day7::part1(divan::black_box(include_str!("../../../inputs/day7.txt",)))
}

#[divan::bench]
fn bench_part2() -> Result<u64> {
    Day7::part2(divan::black_box(include_str!("../../../inputs/day7.txt",)))
}

