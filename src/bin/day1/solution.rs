#[path = "../../utils.rs"]
pub mod utils;

pub use utils::{Result, Solution};

pub type Answer = u32;

pub struct Day1;

const DIAL_START: i32 = 50;
const DIAL_MAX: i32 = 100;

impl Solution<Answer> for Day1 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let mut current_ticks = DIAL_START;
        let mut points_at_zero = 0;
        for line in input.lines() {
            let is_positive = matches!(line.chars().next(), Some('R'));
            let num = line.get(1..).unwrap().parse::<i32>().unwrap();
            current_ticks += num * if is_positive { 1 } else { -1 };

            if current_ticks < 0 {
                while current_ticks < 0 {
                    current_ticks += DIAL_MAX;
                }
            } else if current_ticks >= DIAL_MAX {
                while current_ticks >= DIAL_MAX {
                    current_ticks -= DIAL_MAX;
                }
            }

            if current_ticks == 0 {
                points_at_zero += 1;
            }
        }

        Ok(points_at_zero)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let mut current_ticks = DIAL_START;
        let mut points_at_zero: Answer = 0;
        for line in input.lines() {
            let is_positive = matches!(line.chars().next(), Some('R'));
            let num = line.get(1..).unwrap().parse::<i32>().unwrap();

            let crossings = if is_positive {
                (current_ticks + num) / DIAL_MAX
            } else if current_ticks == 0 {
                num / DIAL_MAX
            } else {
                (num - current_ticks + DIAL_MAX) / DIAL_MAX
            };
            points_at_zero += crossings as Answer;

            current_ticks += num * if is_positive { 1 } else { -1 };
            current_ticks %= DIAL_MAX;
        }

        Ok(points_at_zero)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day1, Solution};

    #[test]
    fn test_part1() {
        let test = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        let result = Day1::part1(test);
        assert_eq!(result.unwrap(), 3)
    }

    #[test]
    fn test_part2() {
        let test = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        let result = Day1::part2(test);
        assert_eq!(result.unwrap(), 6)
    }
}
