#[path = "../../utils.rs"]
pub mod utils;
use std::collections::VecDeque;
use std::ops::BitXor;

use good_lp::{constraint, microlp, variable, Expression, Solution as LpSolution, SolverModel};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::{IResult, Parser};
use rayon::prelude::*;
pub use utils::{Result, Solution};

pub type Answer = u32;

pub struct Day10;

fn int_vec_to_u32(vec: Vec<u32>) -> u32 {
    vec.into_iter()
        .fold(0, |acc, num| acc + 2u32.pow(num as u32))
}

#[derive(Debug, Default)]
struct Machine {
    lights: u32,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<u32>,
}

impl Machine {
    fn new(lights: u32, buttons: Vec<Vec<u32>>, joltage: Vec<u32>) -> Self {
        Self {
            lights,
            buttons,
            joltage,
        }
    }
}

fn parse_lights(input: &str) -> IResult<&str, u32> {
    let (input, result): (_, Vec<char>) =
        delimited(char('['), many1(alt((char('.'), char('#')))), char(']')).parse(input)?;

    let result_num = result.into_iter().enumerate().fold(0, |acc, (i, chr)| {
        if chr == '#' {
            acc + 2u32.pow(i as u32)
        } else {
            acc
        }
    });

    Ok((input, result_num))
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, result): (_, Vec<Vec<u32>>) = separated_list1(
        char(' '),
        delimited(
            char('('),
            separated_list1(char(','), map_res(digit1, str::parse)),
            char(')'),
        ),
    )
    .parse(input)?;

    Ok((input, result))
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, result): (_, Vec<u32>) = delimited(
        char('{'),
        separated_list1(char(','), map_res(digit1, str::parse)),
        char('}'),
    )
    .parse(input)?;

    Ok((input, result))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (lights, _, buttons, _, joltage)): (_, (u32, _, Vec<Vec<u32>>, _, Vec<u32>)) = (
        parse_lights,
        char(' '),
        parse_buttons,
        char(' '),
        parse_joltage,
    )
        .parse(input)?;

    Ok((input, Machine::new(lights, buttons, joltage)))
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(parse_machine)
        .map(|machine_result| machine_result.unwrap().1)
        .collect_vec()
}

fn solve_part_1(machine: Machine) -> Answer {
    let mut queue = VecDeque::new();
    let start = (0u32, 0u32);
    let buttons = machine
        .buttons
        .into_iter()
        .map(int_vec_to_u32)
        .collect_vec();

    queue.push_back(start);
    let mut min = None;
    while let Some((current, clicks)) = queue.pop_front() {
        if min.is_some() && clicks > min.unwrap() {
            break;
        }
        if current == machine.lights {
            if min.is_none() || (min.is_some() && min.unwrap() > clicks) {
                min = Some(clicks);
            }
        }
        for button in &buttons {
            let next = current.bitxor(button);
            queue.push_back((next, clicks + 1));
        }
    }

    min.unwrap_or(0)
}

fn solve_part_2(machine: Machine) -> Answer {
    let target = &machine.joltage;
    let buttons = &machine.buttons;
    let n_counters = target.len();
    let n_buttons = buttons.len();

    // Create variables for each button press count
    let mut vars = good_lp::ProblemVariables::new();
    let button_vars: Vec<_> = (0..n_buttons)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    // Minimize total button presses
    let objective: Expression = button_vars.iter().copied().sum();

    let mut problem = vars.minimise(objective).using(microlp);

    // Add constraints
    for counter_idx in 0..n_counters {
        let mut expr: Expression = 0.into();
        for (button_idx, button) in buttons.iter().enumerate() {
            if button.contains(&(counter_idx as u32)) {
                expr += button_vars[button_idx];
            }
        }
        problem = problem.with(constraint!(expr == target[counter_idx] as i32));
    }

    // Solve
    match problem.solve() {
        Ok(solution) => {
            let total: f64 = button_vars.iter().map(|&v| solution.value(v)).sum();
            total.round() as Answer
        }
        Err(_) => 0,
    }
}

impl Solution<Answer> for Day10 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let machines = parse(input);

        Ok(machines.into_par_iter().map(solve_part_1).sum::<Answer>())
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let machines = parse(input);

        Ok(machines.into_par_iter().map(solve_part_2).sum::<Answer>())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day10, Solution};

    #[test]
    fn test_part1() {
        let test = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        let result = Day10::part1(test);
        assert_eq!(result.unwrap(), 7)
    }

    #[test]
    fn test_part2() {
        let test = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        let result = Day10::part2(test);
        assert_eq!(result.unwrap(), 33)
    }
}
