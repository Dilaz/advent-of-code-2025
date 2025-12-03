#[path = "../../utils.rs"]
pub mod utils;
use itertools::Itertools;
pub use utils::Solution;
use miette::Result;
use rayon::prelude::*;
pub struct Day2;

trait Length {
    fn len(&self) -> usize;
}

impl Length for u64 {
    fn len(&self) -> usize {
        let mut num = *self;
        let mut size = 0;
        while num > 0 {
            num /= 10;
            size += 1;
        }
        size
    }
}

fn is_invalid_part1(num: u64) -> bool {
    let num_len = num.len() as u32;
    if !num_len.is_multiple_of(2) {
        return false;
    }

    num / 10_u64.pow(num_len / 2) == num % 10_u64.pow(num_len / 2)
}

fn is_invalid_part2(num: u64) -> bool {
    let num_len = num.len() as u32;
    for len in 1..=num_len / 2 {
        if !num_len.is_multiple_of(len) {
            continue;
        }

        let multiplier = 10_u64.pow(len);
        let mut current = num;
        let first = current % multiplier;
        current /= multiplier;
        
        let mut ok = true;
        while current > 0 {
            let next = current % multiplier;
            current /= multiplier;
            if first != next {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }

    false
}

impl Solution<u64> for Day2 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u64> {
        let pairs = input.split(',').collect::<Vec<&str>>();
        let sum =pairs.par_iter().map(|pair| {
            let (start, end) = pair.split('-').map(|num| num.parse::<u64>().unwrap()).collect_tuple().unwrap();
            (start..=end).filter(|num| is_invalid_part1(*num)).sum::<u64>()
        }).sum::<u64>();

        Ok(sum)
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u64> {
        let pairs = input.split(',').collect::<Vec<&str>>();
        let sum =pairs.iter().map(|pair| {
            let (start, end) = pair.split('-').map(|num| num.parse::<u64>().unwrap()).collect_tuple().unwrap();
            (start..=end).filter(|num| is_invalid_part2(*num)).sum::<u64>()
        }).sum::<u64>();

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day2, Solution};

    #[test]
    fn test_part1() {
        let test = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
        let result = Day2::part1(test);
        assert_eq!(result.unwrap(), 1227775554)
    }

    #[test]
    fn test_part2() {
        let test = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
        let result = Day2::part2(test);
        assert_eq!(result.unwrap(), 4174379265)
    }

    #[test]
    fn test_is_invalid_part2_cases() {
        use super::is_invalid_part2;
        assert!(is_invalid_part2(11));
        assert!(is_invalid_part2(22));
        assert!(is_invalid_part2(1212));
        assert!(is_invalid_part2(123123));
        assert!(is_invalid_part2(121212));
        assert!(is_invalid_part2(101010));
        
        assert!(!is_invalid_part2(10));
        assert!(!is_invalid_part2(12));
        assert!(!is_invalid_part2(123));
        assert!(!is_invalid_part2(12121));
    }
}
