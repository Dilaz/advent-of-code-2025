mod solution;
use solution::Day6;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day6.txt");
    println!("Part 1: {:?}", Day6::part1(input));
    println!("Part 2: {:?}", Day6::part2(input));
}
