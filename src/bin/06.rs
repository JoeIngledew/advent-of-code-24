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

fn is_loop(
    map: Vec<Vec<MapPos>>,
    max_x: usize,
    max_y: usize,
    guard: &mut Guard,
    obj_x: usize,
    obj_y: usize,
) -> bool {
    let mut is_oob = false;
    let mut visit_count_up: usize = 0;
    let mut visit_count_down: usize = 0;
    let mut visit_count_l: usize = 0;
    let mut visit_count_r: usize = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut failed_ins_count = 0;

    while !is_oob {
        let insert = visited.insert((guard.position_x, guard.position_y));
        if !insert {
            failed_ins_count += 1;
        } else {
            failed_ins_count = 0;
        }

        if failed_ins_count > 10000 {
            // if we're on the same path for 10000 consecutive steps, we're probably in an infinite loop :)
            return true;
        }

        match guard.facing {
            Facing::Down => {
                if guard.position_y == max_y {
                    is_oob = true;
                } else if map[guard.position_y + 1][guard.position_x] == MapPos::Object {
                    if (guard.position_y + 1) == obj_y && guard.position_x == obj_x {
                        visit_count_down += 1;
                    }
                    guard.facing = Facing::Left;
                } else {
                    guard.position_y += 1;
                }
            }
            Facing::Up => {
                if guard.position_y == 0 {
                    is_oob = true;
                } else if map[guard.position_y - 1][guard.position_x] == MapPos::Object {
                    if (guard.position_y - 1) == obj_y && guard.position_x == obj_x {
                        visit_count_up += 1;
                    }
                    guard.facing = Facing::Right;
                } else {
                    guard.position_y -= 1;
                }
            }
            Facing::Left => {
                if guard.position_x == 0 {
                    is_oob = true;
                } else if map[guard.position_y][guard.position_x - 1] == MapPos::Object {
                    if guard.position_y == obj_y && (guard.position_x - 1) == obj_x {
                        visit_count_l += 1;
                    }
                    guard.facing = Facing::Up;
                } else {
                    guard.position_x -= 1;
                }
            }
            Facing::Right => {
                if guard.position_x == max_x {
                    is_oob = true;
                } else if map[guard.position_y][guard.position_x + 1] == MapPos::Object {
                    if guard.position_y == obj_y && (guard.position_x + 1) == obj_x {
                        visit_count_r += 1;
                    }
                    guard.facing = Facing::Down;
                } else {
                    guard.position_x += 1;
                }
            }
        }

        if visit_count_down > 1 || visit_count_l > 1 || visit_count_r > 1 || visit_count_up > 1 {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, max_x, max_y, mut guard) = read_map(input);
    let visited = process_map(map, max_x, max_y, &mut guard);
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, max_x, max_y, guard) = read_map(input);
    let (i_x, i_y) = (guard.position_x, guard.position_y);
    let mut loop_count: u32 = 0;
    for x in 0..(max_x + 1) {
        for y in 0..(max_y + 1) {
            let is_floor = map[y][x] == MapPos::Floor;
            let is_init = x == i_x && y == i_y;

            if is_floor && !is_init {
                let mut modified_map = map.clone();
                modified_map[y][x] = MapPos::Object;
                // println!("Testing obstruction at ({}, {})", x, y);
                if is_loop(modified_map, max_x, max_y, &mut guard.clone(), x, y) {
                    loop_count += 1;
                }
            }
        }
    }
    Some(loop_count)
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
        assert_eq!(result, Some(6));
    }
}
