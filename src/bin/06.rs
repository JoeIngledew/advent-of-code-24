use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    position_x: usize,
    position_y: usize,
    facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapPos {
    Floor,
    Object,
}

fn read_map(input: &str) -> (Vec<Vec<MapPos>>, usize, usize, Guard) {
    let mut guard = Guard {
        position_x: 0,
        position_y: 0,
        facing: Facing::Up,
    };
    let map: Vec<Vec<MapPos>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let line_vec: Vec<MapPos> = line
                .chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => MapPos::Object,
                    '^' => {
                        guard = Guard {
                            position_x: x,
                            position_y: y,
                            facing: Facing::Up,
                        };
                        MapPos::Floor
                    }
                    _ => MapPos::Floor,
                })
                .collect();
            line_vec
        })
        .collect();

    let max_y = map.len() - 1;
    let max_x = map[0].len() - 1;

    (map, max_x, max_y, guard)
}

fn process_map(
    map: Vec<Vec<MapPos>>,
    max_x: usize,
    max_y: usize,
    guard: &mut Guard,
) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut is_oob = false;

    while !is_oob {
        visited.insert((guard.position_x, guard.position_y));

        match guard.facing {
            Facing::Down => {
                if guard.position_y == max_y {
                    is_oob = true;
                } else if map[guard.position_y + 1][guard.position_x] == MapPos::Object {
                    guard.facing = Facing::Left;
                } else {
                    guard.position_y += 1;
                }
            }
            Facing::Up => {
                if guard.position_y == 0 {
                    is_oob = true;
                } else if map[guard.position_y - 1][guard.position_x] == MapPos::Object {
                    guard.facing = Facing::Right;
                } else {
                    guard.position_y -= 1;
                }
            }
            Facing::Left => {
                if guard.position_x == 0 {
                    is_oob = true;
                } else if map[guard.position_y][guard.position_x - 1] == MapPos::Object {
                    guard.facing = Facing::Up;
                } else {
                    guard.position_x -= 1;
                }
            }
            Facing::Right => {
                if guard.position_x == max_x {
                    is_oob = true;
                } else if map[guard.position_y][guard.position_x + 1] == MapPos::Object {
                    guard.facing = Facing::Down;
                } else {
                    guard.position_x += 1;
                }
            }
        }
    }

    visited
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, max_x, max_y, guard) = read_map(input);
    let mut guard = guard;
    let visited = process_map(map, max_x, max_y, &mut guard);
    Some(visited.len())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
