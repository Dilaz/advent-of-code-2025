mod solution;
use solution::Day7;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day7.txt");
    println!("Part 1: {:?}", Day7::part1(input));
    println!("Part 2: {:?}", Day7::part2(input));
}
