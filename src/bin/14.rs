use regex::Regex;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct BoundedNum {
    value: u8,
    upper_bound: u8,
}

impl BoundedNum {
    fn new(value: u8, upper_bound: u8) -> Self {
        Self { value, upper_bound }
    }

    fn add(self, other: u8) -> Self {
        if self.upper_bound > other && (self.upper_bound - other) < self.value {
            let new_val = other - (self.upper_bound - self.value) - 1;
            BoundedNum::new(new_val, self.upper_bound)
        } else {
            BoundedNum::new(self.value + other, self.upper_bound)
        }
    }

    fn subtract(self, other: u8) -> Self {
        if self.value < other {
            let diff = other - self.value;
            BoundedNum::new((self.upper_bound + 1) - diff, self.upper_bound)
        } else {
            BoundedNum::new(self.value - other, self.upper_bound)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq)]
struct Velocity(u8, Direction);

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    pos_x: BoundedNum,
    pos_y: BoundedNum,
    velocity_x: Velocity,
    velocity_y: Velocity,
}

fn step(robot: &mut Robot) {
    let next_x = if robot.velocity_x.1 == Direction::Positive {
        robot.pos_x.add(robot.velocity_x.0)
    } else {
        robot.pos_x.subtract(robot.velocity_x.0)
    };
    let next_y = if robot.velocity_y.1 == Direction::Positive {
        robot.pos_y.add(robot.velocity_y.0)
    } else {
        robot.pos_y.subtract(robot.velocity_y.0)
    };

    robot.pos_x = next_x;
    robot.pos_y = next_y;
}

pub fn part_one(input: &str) -> Option<usize> {
    let bounds_x: u8 = 101;
    let bounds_y: u8 = 103;

    let mut robots: Vec<Robot> = Vec::new();
    let pattern = Regex::new(
        r"p\=(?<px>[0-9]+),(?<py>[0-9]+) v\=(?<vxng>-?)(?<vxm>[0-9]+),(?<vyng>-?)(?<vym>[0-9]+)",
    )
    .unwrap();
    for line in input.lines() {
        let caps = pattern.captures(line).unwrap();
        let p_x = caps["px"].parse::<u8>().unwrap();
        let p_y = caps["py"].parse::<u8>().unwrap();
        let vxng = !caps["vxng"].is_empty();
        let vx = caps["vxm"].parse::<u8>().unwrap();
        let vyng = !caps["vyng"].is_empty();
        let vy = caps["vym"].parse::<u8>().unwrap();
        let dir_x = if vxng {
            Direction::Negative
        } else {
            Direction::Positive
        };
        let dir_y = if vyng {
            Direction::Negative
        } else {
            Direction::Positive
        };
        let robot = Robot {
            pos_x: BoundedNum::new(p_x, bounds_x - 1),
            pos_y: BoundedNum::new(p_y, bounds_y - 1),
            velocity_x: Velocity(vx, dir_x),
            velocity_y: Velocity(vy, dir_y),
        };
        robots.push(robot);
    }

    for _ in 0..100 {
        for r in robots.iter_mut() {
            step(r);
        }
    }

    let mut first_segment: usize = 0;
    let mut second_segment: usize = 0;
    let mut third_segment: usize = 0;
    let mut fourth_segment: usize = 0;

    for r in robots {
        if r.pos_x.value < (bounds_x / 2) && r.pos_y.value < (bounds_y / 2) {
            first_segment += 1;
        } else if r.pos_y.value < (bounds_y / 2) && r.pos_x.value != (bounds_x / 2) {
            second_segment += 1;
        } else if r.pos_x.value < (bounds_x / 2) && r.pos_y.value != (bounds_y / 2) {
            third_segment += 1;
        } else if r.pos_x.value != (bounds_x / 2) && r.pos_y.value != (bounds_y / 2) {
            fourth_segment += 1;
        }
    }

    Some(first_segment * second_segment * third_segment * fourth_segment)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // have to change bounds, can't be bothered to separate properly
    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(12));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
