pub mod template;

// Use this file to add helper functions and additional modules.
use anyhow::Error;
use num_traits::Num;
use std::fmt;
use std::hash::Hash;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

pub fn flat_manhattan_distance(p1: Point, p2: Point) -> usize {
    let x_dist = p1.x.abs_diff(p2.x);
    let y_dist = p1.y.abs_diff(p2.y);
    x_dist + y_dist
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub enum SplitPattern<'a> {
    Char(char),
    Str(&'a str),
}

pub fn read_line_numbers<T: Num + std::str::FromStr>(
    line: &str,
    split: SplitPattern,
) -> Result<Vec<T>, Error> {
    let split_str: Vec<&str> = match split {
        SplitPattern::Char(c) => line.split(c).collect(),
        SplitPattern::Str(s) => line.split(s).collect(),
    };

    let parsed: Vec<T> = split_str
        .iter()
        .flat_map(|s| match s.parse::<T>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();

    if parsed.len() == split_str.len() {
        Ok(parsed)
    } else {
        Err(Error::msg("Cannot parse"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line_numbers_u32_char() {
        let input = "1 2 3 4 5";
        let output: Vec<u32> = vec![1, 2, 3, 4, 5];
        let actual = read_line_numbers::<u32>(input, SplitPattern::Char(' '));
        assert!(actual.is_ok());
        assert_eq!(output, actual.unwrap());
    }

    #[test]
    fn test_read_line_numbers_f64_str() {
        let input = "1.23, 2, 3.543, 4.5";
        let output: Vec<f64> = vec![1.23, 2.0, 3.543, 4.5];
        let actual = read_line_numbers::<f64>(input, SplitPattern::Str(", "));
        assert!(actual.is_ok());
        assert_eq!(output, actual.unwrap());
    }

    #[test]
    fn test_read_line_no_parse() {
        let input = "1 2 3 hello";
        let actual = read_line_numbers::<u32>(input, SplitPattern::Char(' '));
        assert!(actual.is_err());
    }

    #[test]
    fn test_read_line_out_of_range() {
        let input = "1 | 48392048932045093274590 | 3";
        let actual = read_line_numbers::<u8>(input, SplitPattern::Str(" | "));
        assert!(actual.is_err());
    }
}
