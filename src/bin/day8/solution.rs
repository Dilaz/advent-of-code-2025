#[path = "../../utils.rs"]
pub mod utils;
use std::collections::HashMap;

use glam::IVec3;
use itertools::Itertools;
pub use utils::{Result, Solution};

pub type Answer = u64;

pub struct Day8;

const MAX_CONNECTIONS: u32 = if cfg!(test) { 10 } else { 1000 };

fn parse_input(input: &str) -> Vec<IVec3> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',').map(|n| n.parse().unwrap()).collect();
            IVec3::from_slice(&coords)
        })
        .collect()
}

fn get_sorted_distances(points: Vec<IVec3>) -> Vec<(f32, IVec3, IVec3)> {
    let mut distances = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.as_vec3().distance(b.as_vec3()), *a, *b))
        .collect_vec();

    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    distances
}

impl Solution<Answer> for Day8 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        let points: Vec<IVec3> = parse_input(input);
        let num_points = points.len();

        let distances = get_sorted_distances(points);

        let mut in_circuit: HashMap<IVec3, usize> = HashMap::new();
        let mut circuits: Vec<Vec<IVec3>> = vec![];

        let mut connections = 0;

        for (_, p1, p2) in distances {
            if connections >= MAX_CONNECTIONS {
                break;
            }
            connections += 1;

            match (in_circuit.get(&p1).copied(), in_circuit.get(&p2).copied()) {
                (Some(idx1), Some(idx2)) if idx1 != idx2 => {
                    let (keep, merge) = (idx1.min(idx2), idx1.max(idx2));
                    let merged = std::mem::take(&mut circuits[merge]);
                    for point in &merged {
                        in_circuit.insert(*point, keep);
                    }
                    circuits[keep].extend(merged);
                }
                (Some(_), Some(_)) => {} // Same circuit, skip
                (Some(idx), None) => {
                    in_circuit.insert(p2, idx);
                    circuits[idx].push(p2);
                }
                (None, Some(idx)) => {
                    in_circuit.insert(p1, idx);
                    circuits[idx].push(p1);
                }
                (None, None) => {
                    let idx = circuits.len();
                    in_circuit.insert(p1, idx);
                    in_circuit.insert(p2, idx);
                    circuits.push(vec![p1, p2]);
                }
            }
        }

        let standalone = num_points - in_circuit.len();

        let mut sizes: Vec<_> = circuits
            .iter()
            .map(|c| c.len() as Answer)
            .filter(|&size| size > 0)
            .chain(std::iter::repeat(1).take(standalone))
            .collect();

        sizes.sort_by(|a, b| b.cmp(a));
        Ok(sizes.into_iter().take(3).product())
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let points: Vec<IVec3> = parse_input(input);

        let distances = get_sorted_distances(points);

        let mut in_circuit: HashMap<IVec3, usize> = HashMap::new();
        let mut circuits: Vec<Vec<IVec3>> = vec![];

        let mut last_connection: (Answer, Answer) = (0, 0);
        for (_, p1, p2) in distances {
            let made_connection = match (in_circuit.get(&p1).copied(), in_circuit.get(&p2).copied())
            {
                (Some(idx1), Some(idx2)) if idx1 != idx2 => {
                    let (keep, merge) = (idx1.min(idx2), idx1.max(idx2));
                    let merged = std::mem::take(&mut circuits[merge]);
                    for point in &merged {
                        in_circuit.insert(*point, keep);
                    }
                    circuits[keep].extend(merged);
                    true
                }
                (Some(_), Some(_)) => false, // Same circuit, skip
                (Some(idx), None) => {
                    in_circuit.insert(p2, idx);
                    circuits[idx].push(p2);
                    true
                }
                (None, Some(idx)) => {
                    in_circuit.insert(p1, idx);
                    circuits[idx].push(p1);
                    true
                }
                (None, None) => {
                    let idx = circuits.len();
                    in_circuit.insert(p1, idx);
                    in_circuit.insert(p2, idx);
                    circuits.push(vec![p1, p2]);
                    true
                }
            };

            if made_connection {
                last_connection = (p1.x as Answer, p2.x as Answer);
            }
        }

        Ok(last_connection.0 * last_connection.1)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day8, Solution};

    #[test]
    fn test_part1() {
        let test = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;
        let result = Day8::part1(test);
        assert_eq!(result.unwrap(), 40)
    }

    #[test]
    fn test_part2() {
        let test = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;
        let result = Day8::part2(test);
        assert_eq!(result.unwrap(), 25272)
    }
}
