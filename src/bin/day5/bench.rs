mod solution;
use solution::Day5;
use solution::Solution;
use miette::Result;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_part1() -> Result<u64> {
   Day5::part1(divan::black_box(include_str!(
        "../../../inputs/day5.txt",
    )))
}

#[divan::bench]
fn bench_part2() -> Result<u64> {
    Day5::part2(divan::black_box(include_str!(
        "../../../inputs/day5.txt",
    )))
}