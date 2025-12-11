mod solution;
use solution::Day9;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day9.txt");
    println!("Part 1: {:?}", Day9::part1(input));
    println!("Part 2: {:?}", Day9::part2(input));
}
