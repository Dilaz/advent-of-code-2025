#[path = "../../utils.rs"]
pub mod utils;
use std::{collections::BTreeMap, sync::RwLock};

use itertools::Itertools;
use lazy_static::lazy_static;

pub use utils::{Result, Solution};

pub type Answer = u64;

const PART1_START_STR: &str = "you";
const PART2_START_STR: &str = "svr";
const END_STR: &str = "out";
const PART2_DAC_STR: &str = "dac";
const PART2_FFT_STR: &str = "fft";

lazy_static! {
    static ref PART2_CACHE: RwLock<BTreeMap<(String, bool, bool), Answer>> =
        RwLock::new(BTreeMap::new());
}

pub struct Day11;

fn find_routes_part1(routes: &BTreeMap<String, Vec<String>>, start: &str) -> Answer {
    let current = routes.get(start).unwrap();
    return current
        .iter()
        .filter(|s| *s != END_STR)
        .map(|s| find_routes_part1(routes, s.as_str()))
        .sum::<Answer>()
        + if current.contains(&END_STR.to_string()) {
            1
        } else {
            0
        };
}

fn find_routes_part2(
    routes: &BTreeMap<String, Vec<String>>,
    start: &str,
    dac_visited: bool,
    fft_visited: bool,
) -> Answer {
    let dac_now = dac_visited || start == PART2_DAC_STR;
    let fft_now = fft_visited || start == PART2_FFT_STR;

    let cache_key = (start.to_string(), dac_now, fft_now);

    // Check cache
    if let Some(&cached) = PART2_CACHE.read().unwrap().get(&cache_key) {
        return cached;
    }

    let current = routes.get(start).unwrap();
    let mut count: Answer = 0;

    if current.contains(&END_STR.to_string()) && dac_now && fft_now {
        count += 1;
    }

    for next in current.iter().filter(|s| *s != END_STR) {
        count += find_routes_part2(routes, next.as_str(), dac_now, fft_now);
    }

    // Store in cache
    PART2_CACHE.write().unwrap().insert(cache_key, count);
    count
}

fn parse_input(input: &str) -> BTreeMap<String, Vec<String>> {
    let mut routes = BTreeMap::new();

    input
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(from, to)| {
            (
                from.to_string(),
                to.split_whitespace().map(String::from).collect_vec(),
            )
        })
        .for_each(|(from, to)| {
            routes.entry(from).or_insert_with(Vec::new).extend(to);
        });

    routes
}

impl Solution<Answer> for Day11 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let routes = parse_input(input);

        Ok(find_routes_part1(&routes, PART1_START_STR))
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let routes = parse_input(input);

        Ok(find_routes_part2(&routes, PART2_START_STR, false, false))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day11, Solution};

    #[test]
    fn test_part1() {
        let test = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;
        let result = Day11::part1(test);
        assert_eq!(result.unwrap(), 5)
    }

    #[test]
    fn test_part2() {
        let test = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;
        let result = Day11::part2(test);
        assert_eq!(result.unwrap(), 2)
    }
}
