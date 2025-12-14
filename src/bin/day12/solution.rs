#[path = "../../utils.rs"]
pub mod utils;
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, space1},
    combinator::{map_res, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
pub use utils::{Result, Solution};

pub type Answer = u32;
type Region = ((u32, u32), Vec<u32>);
type Shape = (u32, Vec<Vec<bool>>);

pub struct Day12;

fn parse_shape(input: &str) -> IResult<&str, Shape> {
    (
        terminated(map_res(digit1, str::parse), (char(':'), line_ending)),
        separated_list1(
            line_ending,
            many1(alt((value(false, char('.')), value(true, char('#'))))),
        ),
    )
        .parse(input)
}

fn parse_shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    separated_list1((line_ending, line_ending), parse_shape).parse(input)
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    return separated_pair(
        separated_pair(
            map_res(digit1, str::parse),
            char('x'),
            map_res(digit1, str::parse),
        ),
        tag(": "),
        separated_list1(space1, map_res(digit1, str::parse)),
    )
    .parse(input);
}

fn parse_regions(input: &str) -> IResult<&str, Vec<Region>> {
    separated_list1(line_ending, parse_region).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    separated_pair(parse_shapes, (line_ending, line_ending), parse_regions).parse(input)
}

impl Solution<Answer> for Day12 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let (_, (shapes, regions)) = parse_input(input).unwrap();

        let shapes = shapes
            .into_iter()
            .map(|shape| {
                (
                    shape.0,
                    shape.1.into_iter().flatten().filter(|&x| x).count() as u32,
                )
            })
            .collect::<BTreeMap<u32, u32>>();

        let result = regions
            .into_iter()
            .filter(|region| {
                let (w, h) = region.0;
                let size = w * h;

                let required_space = region
                    .1
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| shapes.get(&(i as u32)).unwrap_or(&0) * x)
                    .sum::<u32>();

                required_space <= size
            })
            .count();

        Ok(result as Answer)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day12, Solution};

    #[test]
    fn test_part1() {
        let test = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;
        let result = Day12::part1(test);
        assert_eq!(result.unwrap(), 2)
    }

    #[test]
    fn test_part2() {
        let test = r#""#;
        let result = Day12::part2(test);
        assert_eq!(result.unwrap(), 0)
    }
}
