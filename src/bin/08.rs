use std::collections::{HashMap, HashSet};

use advent_of_code::Point;

advent_of_code::solution!(8);

#[derive(Debug, Clone, PartialEq, Eq)]
struct AntennaMap {
    map: HashMap<Point, Option<char>>,
    antenna_points: Vec<(Point, char)>,
}

fn parse_map(input: &str) -> AntennaMap {
    let mut map: HashMap<Point, Option<char>> = HashMap::new();
    let mut antenna_points: Vec<(Point, char)> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    map.insert(Point::new(x, y), None);
                }
                c => {
                    map.insert(Point::new(x, y), Some(c));
                    antenna_points.push((Point::new(x, y), c));
                }
            }
        }
    }

    AntennaMap {
        map,
        antenna_points,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Negative,
    Positive,
}

fn abs_diff_with_direction(a: usize, b: usize) -> (usize, Direction) {
    if a > b {
        (a - b, Direction::Negative)
    } else {
        (b - a, Direction::Positive)
    }
}

fn in_line_and_double_dist(root_point: Point, test_point: Point, map_point: Point) -> bool {
    let (rdx, rdx_dir) = abs_diff_with_direction(root_point.x, map_point.x);
    let (rdy, rdy_dir) = abs_diff_with_direction(root_point.y, map_point.y);
    let (tdx, tdx_dir) = abs_diff_with_direction(test_point.x, map_point.x);
    let (tdy, tdy_dir) = abs_diff_with_direction(test_point.y, map_point.y);

    rdx_dir == tdx_dir && rdy_dir == tdy_dir && (tdx == 2 * rdx) && (tdy == 2 * rdy)
}

fn count_antinodes(antennas: &[(Point, char)], map: HashMap<Point, Option<char>>) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (p, c) in antennas {
        for map_p in map.keys() {
            if map_p != p {
                antennas
                    .iter()
                    .filter_map(|(test_point, c2)| {
                        if test_point != p
                            && c == c2
                            && in_line_and_double_dist(*p, *test_point, *map_p)
                        {
                            Some(map_p)
                        } else {
                            None
                        }
                    })
                    .for_each(|p| {
                        antinodes.insert(*p);
                    });
            }
        }
    }

    antinodes.len()
}

fn in_line(root_point: Point, test_point: Point, map_point: Point) -> bool {
    // a = mb + c
    // y = mx + c
    // (a-y) = m(b-x)
    // (a-y) / (b-x) = m
    // c = a - mb

    let b: f64 = root_point.x as f64;
    let a: f64 = root_point.y as f64;
    let y: f64 = test_point.y as f64;
    let x: f64 = test_point.x as f64;
    let m = (a - y) / (b - x);
    let c = a - (m * b);

    let (map_y, map_x): (f64, f64) = (map_point.y as f64, map_point.x as f64);
    let result = ((m * map_x) + c) - map_y;

    // little icky hack to solve floating point math errors :|
    // gotta be a better way to solve this lmao
    result < 0.00001 && result > -0.00001

    // if result < 0.1 && result > -0.1 && result != 0.0 {
    //     println!("Possible floating point err: {} = {} * {} + {}, result: {}", map_y, m, map_x, c, result);
    // }
    // map_y == (m * map_x) + c
}

// fn count_antinodes_2(antennas: &[(Point, char)], map: HashMap<Point, Option<char>>) -> usize {
//     let mut antinodes: HashSet<Point> = HashSet::new();
//     let mut antenna_sets: HashMap<char, Vec<Point>> = HashMap::new();
//     for (a_p, c) in antennas {
//         antenna_sets.entry(*c).or_default().push(*a_p);
//     }

//     for k in antenna_sets.keys() {
//         let points = antenna_sets.get(k).unwrap();
//         let (a, b) = points.split_at(1);
//         let pairs: Vec<(&Point, &Point, &Point)> = iproduct!(a, b, map.keys()).collect();

//         for (r, t, map_p) in pairs.iter() {
//             if in_line(**r, **t, **map_p) {
//                 antinodes.insert(**map_p);
//             }
//         }
//     }

//     antinodes.len()
// }

// fn print_antinode_map(nodes: &HashSet<Point>, map: HashMap<Point, Option<char>>) {
//     let ordered_points = map.keys().sorted_by(|p1, p2| {
//         let init = p1.y.cmp(&p2.y);
//         match init {
//             Ordering::Equal => p1.x.cmp(&p2.x),
//             _ => init,
//         }
//     });

//     for p in ordered_points {
//         let is_anti = nodes.contains(p);
//         let char_to_print = if is_anti { '#' } else { '.' };
//         if p.x == 0 {
//             print!("\n{}", char_to_print);
//         } else {
//             print!("{}", char_to_print);
//         }
//     }
// }

fn count_anti_3(antennas: &[(Point, char)], map: HashMap<Point, Option<char>>) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for map_p in map.keys() {
        for (root, c) in antennas {
            for (test, c2) in antennas {
                if root != test && c == c2 && in_line(*root, *test, *map_p) {
                    antinodes.insert(*map_p);
                }
            }
        }
    }
    // print_antinode_map(&antinodes, map);
    antinodes.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input);
    let count = count_antinodes(&map.antenna_points, map.map);
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_map(input);
    let count = count_anti_3(&map.antenna_points, map.map);
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
