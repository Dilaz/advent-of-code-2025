#[path = "../../utils.rs"]
pub mod utils;

use itertools::Itertools;

pub use utils::{Length, Result, Solution};

pub type Answer = u64;

pub struct Day6;

impl Solution<Answer> for Day6 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let mut values = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<Vec<&str>>>();

        let operators = values.pop().unwrap();
        let number_of_calculations = operators.len();
        let mut total = 0;
        for col in 0..number_of_calculations {
            let operator = operators.get(col).unwrap();
            match *operator {
                "+" => {
                    total += (0usize..values.len())
                        .map(|row| {
                            values
                                .get(row)
                                .unwrap()
                                .get(col)
                                .unwrap()
                                .parse::<u64>()
                                .unwrap()
                        })
                        .sum::<u64>();
                }
                "*" => {
                    total += (0usize..values.len())
                        .map(|row| {
                            values
                                .get(row)
                                .unwrap()
                                .get(col)
                                .unwrap()
                                .parse::<u64>()
                                .unwrap()
                        })
                        .product::<u64>();
                }
                _ => unreachable!(),
            }
        }

        Ok(total)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let mut values = input.lines().collect::<Vec<&str>>();

        let operators = values
            .pop()
            .unwrap()
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect_vec();

        let mut current_col = 0usize;
        let mut total = 0u64;
        for operator in operators.iter() {
            let mut numbers = vec![];
            loop {
                let mut current_num = 0u64;
                let mut all_empty = true;
                for row in values.iter() {
                    if let Some(ch) = row.chars().nth(current_col) {
                        match ch {
                            ' ' => (),
                            n if n.is_ascii_digit() => {
                                all_empty = false;
                                current_num = current_num * 10 + n.to_digit(10).unwrap() as u64;
                            }
                            _ => (),
                        }
                    }
                }
                if !all_empty {
                    numbers.push(current_num);
                } else {
                    match *operator {
                        "+" => total += numbers.iter().sum::<u64>(),
                        "*" => total += numbers.iter().product::<u64>(),
                        _ => unreachable!(),
                    };

                    // Seek the next non-empty column
                    current_col += 1;
                    break;
                }
                current_col += 1;
            }
        }

        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day6, Solution};

    #[test]
    fn test_part1() {
        let test = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        let result = Day6::part1(test);
        assert_eq!(result.unwrap(), 4277556)
    }

    #[test]
    fn test_part2() {
        let test = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        let result = Day6::part2(test);
        assert_eq!(result.unwrap(), 3263827)
    }
}
