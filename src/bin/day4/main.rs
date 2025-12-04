mod solution;
use solution::Day4;
use solution::Solution;

fn main() {
    let input = include_str!("../../../inputs/day4.txt");
    println!("Part 1: {:?}", Day4::part1(input));
    println!("Part 2: {:?}", Day4::part2(input));
}
