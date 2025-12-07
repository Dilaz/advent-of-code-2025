#[path = "../../utils.rs"]
pub mod utils;
pub use utils::{Result, Solution};

pub type Answer = u32;

pub struct DayX;

impl Solution<Answer> for DayX {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        Ok(0)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{DayX, Solution};

    #[test]
    fn test_part1() {
        let test = r#""#;
        let result = DayX::part1(test);
        assert_eq!(result.unwrap(), 0)
    }

    #[test]
    fn test_part2() {
        let test = r#""#;
        let result = DayX::part2(test);
        assert_eq!(result.unwrap(), 0)
    }
}
