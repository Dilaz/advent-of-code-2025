mod solution;
use solution::Day5;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day5.txt");
    println!("Part 1: {:?}", Day5::part1(input));
    println!("Part 2: {:?}", Day5::part2(input));
}
