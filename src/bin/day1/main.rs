mod solution;
use solution::Day1;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day1.txt");
    println!("Part 1: {:?}", Day1::part1(input));
    println!("Part 2: {:?}", Day1::part2(input));
}
