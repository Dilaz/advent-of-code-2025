#[path = "../../utils.rs"]
pub mod utils;
pub use utils::Solution;
use miette::Result;
use glam::IVec2;
pub struct Day4;

const MAX_NUMBER_AROUND: usize = 4;
const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(-1, -1),
    IVec2::new(-1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 0),
    IVec2::new(1, 1),
];

fn check_if_valid(map: &[Vec<char>], coord: IVec2, chr: char) -> bool {
    if chr != '@' {
        return false;
    }

    DIRECTIONS.iter().filter(|dir| {
        let new_coord = coord + **dir;
        if new_coord.x < 0
        || new_coord.y < 0
        || map.len() as i32 <= new_coord.y
        || map.first().unwrap().len() as i32 <= new_coord.x {
            return false;
        }

        *map.get(new_coord.y as usize).unwrap().get(new_coord.x as usize).unwrap() == '@'
    }).count() < MAX_NUMBER_AROUND
}

impl Solution<u32> for Day4 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u32> {
        let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

        let valid = map.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().filter(|(x, cell)| {
                check_if_valid(&map, IVec2::new(*x as i32, y as i32), **cell)
            }).count()
        }).sum::<usize>();

        Ok(valid as u32)
    }
    
    #[tracing::instrument]
    fn part2(input: &str) -> Result<u32> {
        let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    
    let mut total = 0;
    loop {
        let mut to_remove = vec![];
        let valid = map.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().filter(|(x, cell)| {
                let is_valid = check_if_valid(&map, IVec2::new(*x as i32, y as i32), **cell);
                if is_valid {
                    to_remove.push(IVec2::new(*x as i32, y as i32));
                }
                is_valid
            }).count()
        }).sum::<usize>();

        if valid == 0 {
            break;
        }

        to_remove.into_iter().for_each(|coord| {
            *map.get_mut(coord.y as usize).unwrap().get_mut(coord.x as usize).unwrap() = '.';
        });

        total += valid;
        }

        Ok(total as u32)
    }

}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{Day4, Solution};

    #[test]
    fn test_part1() {
        let test = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        let result = Day4::part1(test);
        assert_eq!(result.unwrap(), 13)
    }

    #[test]
    fn test_part2() {
        let test = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        let result = Day4::part2(test);
        assert_eq!(result.unwrap(), 43)
    }
}
