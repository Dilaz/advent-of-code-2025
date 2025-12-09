mod solution;
use solution::Day8;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day8.txt");
    println!("Part 1: {:?}", Day8::part1(input));
    println!("Part 2: {:?}", Day8::part2(input));
}
