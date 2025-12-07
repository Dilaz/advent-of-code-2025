#[path = "../../utils.rs"]
pub mod utils;
use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use miette::Result;
pub use utils::Solution;
pub struct Day7;

impl Solution<u64> for Day7 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u64> {
        let lines = input.lines().collect_vec();
        let start = lines.first().unwrap().find('S').unwrap();
        let mut beams = BTreeSet::new();
        beams.insert(start);

        let mut split_counter = 0;
        for line in lines.into_iter().skip(1) {
            let mut new_beams = BTreeSet::new();
            for (i, c) in line.chars().enumerate() {
                if c == '^' && beams.contains(&i) {
                    split_counter += 1;
                    beams.remove(&i);
                    new_beams.insert(i - 1);
                    new_beams.insert(i + 1);
                }
            }

            new_beams.into_iter().for_each(|n| {
                beams.insert(n);
            });
        }

        Ok(split_counter)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<u64> {
        let lines = input.lines().collect_vec();
        let start = lines.first().unwrap().find('S').unwrap();
        let mut beams: Vec<BTreeMap<usize, u64>> = vec![BTreeMap::from([(start, 1)])];

        for line in lines.into_iter().skip(1) {
            let prev_beams = beams.last().unwrap();
            let mut new_beams = BTreeMap::<usize, u64>::new();
            for (i, c) in line.chars().enumerate() {
                if let Some(num_beams) = prev_beams.get(&i) {
                    if c == '^' {
                        new_beams
                            .entry(i - 1)
                            .and_modify(|n| *n += num_beams)
                            .or_insert(*num_beams);
                        new_beams
                            .entry(i + 1)
                            .and_modify(|n| *n += num_beams)
                            .or_insert(*num_beams);
                    } else {
                        new_beams
                            .entry(i)
                            .and_modify(|n| *n += num_beams)
                            .or_insert(*num_beams);
                    }
                }
            }
            beams.push(new_beams);
        }

        let last_row = beams.last().unwrap().clone();

        Ok(last_row
            .into_iter()
            .fold(0, |total, (_, count)| total + count))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day7, Solution};

    #[test]
    fn test_part1() {
        let test = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        let result = Day7::part1(test);
        assert_eq!(result.unwrap(), 21)
    }

    #[test]
    fn test_part2() {
        let test = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        let result = Day7::part2(test);
        assert_eq!(result.unwrap(), 40)
    }
}
