mod solution;
use solution::Day2;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day2.txt");
    println!("Part 1: {:?}", Day2::part1(input));
    println!("Part 2: {:?}", Day2::part2(input));
}
