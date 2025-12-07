#[path = "../../utils.rs"]
pub mod utils;

use itertools::Itertools;

pub use utils::{Result, Solution};

pub type Answer = u64;

pub struct Day5;

impl Solution<Answer> for Day5 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let mut fresh_ids = Vec::with_capacity(200);
        let mut line_iter = input.lines();

        for line in line_iter.by_ref() {
            if line.is_empty() {
                break;
            }

            let (start, end) = line
                .split('-')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            fresh_ids.push((start, end));
        }

        let mut count = 0;
        for line in line_iter {
            let num = line.parse::<u64>().unwrap();
            for (start, end) in fresh_ids.iter() {
                if start < &num && &num <= end {
                    count += 1;
                    break;
                }
            }
        }

        Ok(count)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let mut line_iter = input.lines();

        let mut fresh_ids = Vec::with_capacity(200);

        for line in line_iter.by_ref() {
            if line.is_empty() {
                break;
            }

            let (start, end) = line
                .split('-')
                .map(|n| n.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            fresh_ids.push((start, end));
        }

        fresh_ids.sort_by_key(|(start, _)| *start);
        let mut final_ranges = Vec::with_capacity(100);

        let mut prev_start: Option<u64> = None;
        let mut prev_end: Option<u64> = None;
        for (start, end) in fresh_ids {
            if let Some(p_end) = prev_end {
                if start <= p_end {
                    prev_end = Some(p_end.max(end));
                    continue;
                } else {
                    final_ranges.push((prev_start.unwrap(), prev_end.unwrap()));
                }
            }

            prev_start = Some(start);
            prev_end = Some(end);
        }
        if let (Some(prev_start), Some(prev_end)) = (prev_start, prev_end) {
            final_ranges.push((prev_start, prev_end));
        }

        let mut total_fresh = 0;
        for (start, end) in final_ranges {
            total_fresh += end - start + 1;
        }

        Ok(total_fresh)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day5, Solution};

    #[test]
    fn test_part1() {
        let test = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        let result = Day5::part1(test);
        assert_eq!(result.unwrap(), 3)
    }

    #[test]
    fn test_part2() {
        let test = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        let result = Day5::part2(test);
        assert_eq!(result.unwrap(), 14)
    }
}
