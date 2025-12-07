#[path = "../../utils.rs"]
pub mod utils;

pub use utils::{Result, Solution};

pub type Answer = u64;

pub struct Day3;

const PART_2_NUMS: usize = 12;

impl Solution<Answer> for Day3 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let batteries = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|num| num.to_digit(10).unwrap() as u64)
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        let sum = batteries
            .iter()
            .map(|battery| {
                let mut max_jolts = 0;
                let mut start_iter = battery.iter();
                for first in start_iter.clone() {
                    let second_iter = start_iter.clone().skip(1);

                    for second in second_iter {
                        let jolts = first * 10 + second;
                        if jolts > max_jolts {
                            max_jolts = jolts;
                        }
                    }

                    start_iter.next();
                }

                max_jolts
            })
            .sum::<Answer>();

        Ok(sum)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let batteries = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|num| num.to_digit(10).unwrap() as u64)
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();
        let sum = batteries
            .iter()
            .map(|battery| {
                let mut stack = Vec::with_capacity(PART_2_NUMS);
                let n = battery.len();
                for (i, &digit) in battery.iter().enumerate() {
                    while let Some(&top) = stack.last() {
                        if top < digit && stack.len() + (n - 1 - i) >= PART_2_NUMS {
                            stack.pop();
                        } else {
                            break;
                        }
                    }
                    if stack.len() < PART_2_NUMS {
                        stack.push(digit);
                    }
                }
                stack.iter().fold(0, |acc, &d| acc * 10 + d)
            })
            .sum::<Answer>();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day3, Solution};

    #[test]
    fn test_part1() {
        let test = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        let result = Day3::part1(test);
        assert_eq!(result.unwrap(), 357)
    }

    #[test]
    fn test_part2() {
        let test = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        let result = Day3::part2(test);
        assert_eq!(result.unwrap(), 3121910778619)
    }
}
