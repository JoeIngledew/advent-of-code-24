advent_of_code::solution!(1);

use std::collections::HashMap;

use regex::Regex;

fn line_to_number_pair(line: &str) -> (u32, u32) {
    let pattern = Regex::new(r"([0-9]+)\s+([0-9]+)").unwrap();
    let mut results: (u32, u32) = (0, 0);
    for (_, [l, r]) in pattern.captures_iter(line).map(|c| c.extract()) {
        results = (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap());
    }
    results
}

fn diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    input.lines().map(line_to_number_pair).for_each(|(l, r)| {
        left.push(l);
        right.push(r);
    });
    left.sort();
    right.sort();
    let res = left.into_iter().zip(right).map(|(a, b)| diff(a, b)).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hmap: HashMap<u32, u32> = HashMap::new();
    let mut left: Vec<u32> = vec![];
    input.lines().map(line_to_number_pair).for_each(|(l, r)| {
        *hmap.entry(r).or_insert(0) += 1;
        left.push(l);
    });
    let res = left
        .iter()
        .map(|l| {
            if let Some(v) = hmap.get(l) {
                *v * *l
            } else {
                0
            }
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
