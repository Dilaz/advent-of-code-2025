mod solution;
use solution::Day2;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<u64> {
   Day2::part1(divan::black_box(include_str!(
        "../../../inputs/day2.txt"
    )))
}

#[divan::bench]
fn bench_part2() -> Result<u64> {
    Day2::part2(divan::black_box(include_str!(
        "../../../inputs/day2.txt"
    )))
}