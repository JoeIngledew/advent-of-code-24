use std::{num::ParseIntError, str::FromStr};

use anyhow::Error;

advent_of_code::solution!(7);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Calibration {
    result: u64,
    values: Vec<u64>,
}

impl FromStr for Calibration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let first = split
            .next()
            .ok_or(Error::msg("No elements"))?
            .parse::<u64>()?;
        let last = split.next().ok_or(Error::msg("Not enough elements"))?;

        let values: Result<Vec<u64>, ParseIntError> =
            last.trim().split(' ').map(|s| s.parse::<u64>()).collect();
        match values {
            Ok(vs) => Ok(Calibration {
                result: first,
                values: vs,
            }),
            Err(e) => Err(Error::from(e)),
        }
    }
}

fn can_be_solved(acc: &[u64], curr: u64, target: u64) -> bool {
    // early exit if we've already overshot
    if curr > target {
        return false;
    }

    match acc {
        [] => curr == target,
        [head, tail @ ..] => {
            let curr_mul = if curr == 0 { 1 } else { curr };
            can_be_solved(tail, curr + head, target) || can_be_solved(tail, curr_mul * head, target)
        }
    }
}

fn can_be_solved_2(acc: &[u64], curr: u64, target: u64) -> bool {
    // early exit if we've already overshot
    if curr > target {
        return false;
    }

    match acc {
        [] => curr == target,
        [head, tail @ ..] => {
            let curr_mul = if curr == 0 { 1 } else { curr };
            let concat_res = format!("{}{}", curr, head).parse::<u64>().unwrap();
            can_be_solved_2(tail, curr + head, target)
                || can_be_solved_2(tail, curr_mul * head, target)
                || can_be_solved_2(tail, concat_res, target)
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let vals: Vec<u64> = input
        .lines()
        .filter_map(|s| {
            if let Ok(c) = s.parse::<Calibration>() {
                if can_be_solved(&c.values, 0, c.result) {
                    Some(c.result)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Some(vals.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let vals: Vec<u64> = input
        .lines()
        .filter_map(|s| {
            if let Ok(c) = s.parse::<Calibration>() {
                if can_be_solved_2(&c.values, 0, c.result) {
                    Some(c.result)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Some(vals.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
