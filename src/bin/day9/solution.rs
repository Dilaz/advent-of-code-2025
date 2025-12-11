#[path = "../../utils.rs"]
pub mod utils;

use std::{
    cmp::{max, min},
    collections::HashSet,
};

use glam::IVec2;
use itertools::Itertools;
use rayon::prelude::*;
pub use utils::{Result, Solution};

pub type Answer = u64;

pub struct Day9;

fn get_range(start: i32, end: i32) -> impl Iterator<Item = i32> {
    if start < end {
        start..end
    } else {
        end..start
    }
}

impl Solution<Answer> for Day9 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<Answer> {
        Ok(input
            .lines()
            .filter_map(|l| l.split_once(','))
            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
            .tuple_combinations()
            .map(|(a, b)| ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1))
            .max()
            .unwrap_or(0) as Answer)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<Answer> {
        let mut points = input
            .lines()
            .filter_map(|l| l.split_once(','))
            .map(|(x, y)| IVec2::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect_vec();

        let mut borders: HashSet<IVec2> = points.iter().copied().collect();
        let mut current = points.first().cloned().unwrap();

        // Add the first point as last
        points.push(current);

        for vec in points.iter().skip(1) {
            if vec.y == current.y {
                let range = get_range(current.x, vec.x);
                for new_x in range {
                    borders.insert(IVec2::new(new_x, current.y));
                }
            } else if vec.x == current.x {
                let range = get_range(current.y, vec.y);
                for new_y in range {
                    borders.insert(IVec2::new(current.x, new_y));
                }
            } else {
                unreachable!()
            }
            current = *vec;
        }
        points.pop();

        let rectangles: Vec<(i32, i32, i32, i32)> = points
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| a.x != b.x && a.y != b.y)
            .map(|(a, b)| (min(a.x, b.x), max(a.x, b.x), min(a.y, b.y), max(a.y, b.y)))
            .collect();

        let mut polygon = points.clone();
        polygon.push(polygon[0]);

        Ok(rectangles
            .par_iter()
            .filter_map(|&(min_x, max_x, min_y, max_y)| {
                let has_interior_border = borders
                    .iter()
                    .any(|p| p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y);

                if has_interior_border {
                    return None;
                }

                // Check corners and edge midpoints are inside the polygon
                let test_points = [
                    // 4 corners
                    (min_x, min_y),
                    (max_x, min_y),
                    (min_x, max_y),
                    (max_x, max_y),
                    // 4 edge midpoints
                    ((min_x + max_x) / 2, min_y),
                    ((min_x + max_x) / 2, max_y),
                    (min_x, (min_y + max_y) / 2),
                    (max_x, (min_y + max_y) / 2),
                    // center
                    ((min_x + max_x) / 2, (min_y + max_y) / 2),
                ];

                let all_inside = test_points.iter().all(|&(test_x, test_y)| {
                    // On border
                    if borders.contains(&IVec2::new(test_x, test_y)) {
                        return true;
                    }
                    // "Ray casting"
                    let mut crossings = 0i32;
                    for edge in polygon.windows(2) {
                        let (p1, p2) = (edge[0], edge[1]);
                        if p1.x == p2.x {
                            let edge_min_y = min(p1.y, p2.y);
                            let edge_max_y = max(p1.y, p2.y);
                            if p1.x > test_x && test_y >= edge_min_y && test_y < edge_max_y {
                                crossings += 1;
                            }
                        }
                    }
                    crossings % 2 == 1
                });

                if all_inside {
                    Some(((max_x - min_x + 1) as i64 * (max_y - min_y + 1) as i64) as Answer)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day9, Solution};

    #[test]
    fn test_part1() {
        let test = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
        let result = Day9::part1(test);
        assert_eq!(result.unwrap(), 50)
    }

    #[test]
    fn test_part2() {
        let test = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
        let result = Day9::part2(test);
        assert_eq!(result.unwrap(), 24)
    }
}
