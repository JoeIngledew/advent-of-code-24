use advent_of_code::Point;
use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeightmapNode {
    coord: Point,
    height: u8,
}

impl HeightmapNode {
    fn new(coord: Point, height: u8) -> Self {
        HeightmapNode { coord, height }
    }
}

fn read_input(input: &str) -> HashMap<Point, HeightmapNode> {
    let mut result: HashMap<Point, HeightmapNode> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point::new(x, y);
            let height = if c == '.' {
                u8::MAX
            } else {
                c.to_digit(10).unwrap() as u8
            };
            result.insert(point, HeightmapNode::new(point, height));
        }
    }
    result
}

fn get_starting_points(map: &HashMap<Point, HeightmapNode>) -> Vec<Point> {
    map.values()
        .filter_map(|v| if v.height == 0 { Some(v.coord) } else { None })
        .collect()
}

fn get_points_around(p: &Point) -> Vec<Point> {
    let mut around: Vec<Point> = vec![];
    around.push(Point::new(p.x, p.y + 1));
    around.push(Point::new(p.x + 1, p.y));

    if p.x > 0 {
        around.push(Point::new(p.x - 1, p.y));
    }

    if p.y > 0 {
        around.push(Point::new(p.x, p.y - 1));
    }
    around
}

fn get_points_around_2(p: &Point, map: &HashMap<Point, HeightmapNode>) -> Vec<Point> {
    let mut around: Vec<Point> = vec![];
    let curr_height = map.get(p).unwrap().height;
    around.push(Point::new(p.x, p.y + 1));
    around.push(Point::new(p.x + 1, p.y));

    if p.x > 0 {
        around.push(Point::new(p.x - 1, p.y));
    }

    if p.y > 0 {
        around.push(Point::new(p.x, p.y - 1));
    }
    around
        .iter()
        .filter_map(|p| {
            map.get(p).and_then(|v| {
                if v.height == (curr_height + 1) {
                    Some(v.coord)
                } else {
                    None
                }
            })
        })
        .collect()
}

fn walkable_paths(start: &Point, curr: u8, map: &HashMap<Point, HeightmapNode>) -> u32 {
    if curr == 9 {
        return 1;
    }

    let next_candidates = get_points_around(start);
    let mut path_sum: u32 = 0;
    for candidate in next_candidates {
        if let Some(v) = map.get(&candidate) {
            if v.height == curr + 1 {
                path_sum += walkable_paths(&candidate, v.height, map);
            }
        }
    }
    path_sum
}

// fn walkable_paths_2(start: &Point, curr: u8, map: &HashMap<Point, HeightmapNode>) -> u32 {
//     if curr == 9 {
//         return 1;
//     }

//     let next_candidates = get_points_around_2(start, map);
//     if next_candidates.is_empty() {
//         return 0;
//     }

//     let mut path_sum: u32 = 0;
//     for candidate in next_candidates {
//         path_sum += walkable_paths(&candidate, curr + 1, map);
//     }
//     path_sum
// }

fn walkable_paths_3(start: &Point, curr: u8, map: &HashMap<Point, HeightmapNode>) -> Vec<Point> {
    if curr == 9 {
        return vec![*start];
    }

    let next_candidates = get_points_around_2(start, map);
    if next_candidates.is_empty() {
        return vec![];
    }

    let mut final_points: Vec<Point> = vec![];
    for candidate in next_candidates {
        let mut walkable = walkable_paths_3(&candidate, curr + 1, map);
        final_points.append(&mut walkable);
    }
    final_points
}

fn get_total_paths_from_point(point: &Point, map: &HashMap<Point, HeightmapNode>) -> u32 {
    let end_points = walkable_paths_3(point, 0, map);
    end_points.iter().unique().count() as u32
}

fn get_total_paths_from_point_2(point: &Point, map: &HashMap<Point, HeightmapNode>) -> u32 {
    walkable_paths(point, 0, map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = read_input(input);
    let starts = get_starting_points(&map);
    let sum = starts
        .iter()
        .map(|p| get_total_paths_from_point(p, &map))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = read_input(input);
    let starts = get_starting_points(&map);
    let sum = starts
        .iter()
        .map(|p| get_total_paths_from_point_2(p, &map))
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
