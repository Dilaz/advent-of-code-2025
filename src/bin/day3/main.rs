mod solution;
use solution::Day3;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day3.txt");
    println!("Part 1: {:?}", Day3::part1(input));
    println!("Part 2: {:?}", Day3::part2(input));
}
