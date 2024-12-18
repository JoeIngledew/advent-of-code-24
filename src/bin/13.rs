// use hashbrown::HashMap;
// use regex::Regex;

advent_of_code::solution!(13);

// struct ClawMachine {
//     a_x: u32,
//     a_y: u32,
//     b_x: u32,
//     b_y: u32,
//     prize_x: u32,
//     prize_y: u32,
// }

// impl ClawMachine {
//     fn new(a_x: u32, a_y: u32, b_x: u32, b_y: u32, prize_x: u32, prize_y: u32) -> Self {
//         ClawMachine {
//             a_x,
//             a_y,
//             b_x,
//             b_y,
//             prize_x,
//             prize_y,
//         }
//     }
// }

// fn min_prize_cost_cheaper_a(
//     target_x: u32,
//     target_y: u32,
//     machine: &ClawMachine,
//     b_count: u32,
// ) -> Option<u32> {
//     if target_x == 0 && target_y == 0 {
//         return Some(b_count);
//     }

//     if b_count > 100 {
//         return None;
//     }

//     if (target_x < machine.a_x && target_x < machine.b_x)
//         || (target_y < machine.a_y && target_y < machine.b_y)
//     {
//         return None;
//     }

//     let remainder_x = target_x % machine.a_x;
//     let remainder_y = target_y % machine.a_y;
//     if remainder_x == 0 && remainder_y == 0 {
//         let a_cost = (target_x / machine.a_x) * 3;
//         let total_cost = b_count + a_cost;
//         Some(total_cost)
//     } else if target_x < machine.b_x || target_y < machine.b_y {
//         None
//     } else {
//         let next_target_x = target_x - machine.b_x;
//         let next_target_y = target_y - machine.b_y;
//         min_prize_cost_cheaper_b(next_target_x, next_target_y, machine, b_count + 1)
//     }
// }

// fn min_prize_cost_cheaper_b(
//     target_x: u32,
//     target_y: u32,
//     machine: &ClawMachine,
//     a_count: u32,
// ) -> Option<u32> {
//     if target_x == 0 && target_y == 0 {
//         return Some(a_count * 3);
//     }

//     if a_count > 100 {
//         return None;
//     }

//     if (target_x < machine.a_x && target_x < machine.b_x)
//         || (target_y < machine.a_y && target_y < machine.b_y)
//     {
//         return None;
//     }

//     let remainder_x = target_x % machine.b_x;
//     let remainder_y = target_y % machine.b_y;
//     if remainder_x == 0 && remainder_y == 0 {
//         let a_cost = a_count * 3;
//         let total_cost = (target_x / machine.b_x) + a_cost;
//         Some(total_cost)
//     } else if target_x < machine.a_x || target_y < machine.a_y {
//         None
//     } else {
//         let next_target_x = target_x - machine.a_x;
//         let next_target_y = target_y - machine.a_y;
//         min_prize_cost_cheaper_b(next_target_x, next_target_y, machine, a_count + 1)
//     }
// }

// fn min_of_options(a: Option<u32>, b: Option<u32>) -> Option<u32> {
//     if let Some(a_v) = a {
//         if let Some(b_v) = b {
//             Some(a_v.min(b_v))
//         } else {
//             a
//         }
//     } else if b.is_some() {
//         b
//     } else {
//         None
//     }
// }

// fn min_cost(
//     target_x: u32,
//     target_y: u32,
//     machine: &ClawMachine,
//     a_count: u32,
//     b_count: u32,
//     cache: &mut HashMap<(u32, u32, u32, u32), Option<u32>>,
// ) -> Option<u32> {
//     // println!("TEST: A {} B {} TARGET {} {}", a_count, b_count, target_x, target_y);
//     if let Some(&opt) = cache.get(&(target_x, target_y, a_count, b_count)) {
//         return opt;
//     }

//     if target_x == 0 && target_y == 0 {
//         cache.insert(
//             (target_x, target_y, a_count, b_count),
//             Some((a_count * 3) + b_count),
//         );
//         return Some((a_count * 3) + b_count);
//     }

//     if (a_count + b_count) > 100 {
//         cache.insert((target_x, target_y, a_count, b_count), None);
//         return None;
//     }

//     let b_rem_x = target_x % machine.b_x;
//     let b_rem_y = target_y % machine.b_y;
//     let a_rem_x = target_x % machine.a_x;
//     let a_rem_y = target_y % machine.b_y;

//     if a_rem_x == 0 && a_rem_y == 0 && b_rem_x == 0 && b_rem_y == 0 {
//         let a_cost = (a_count * 3) + b_count + ((target_x / machine.a_x) * 3);
//         let b_cost = (a_count * 3) + b_count + (target_x / machine.b_x);
//         cache.insert(
//             (target_x, target_y, a_count, b_count),
//             Some(a_cost.min(b_cost)),
//         );
//         return Some(a_cost.min(b_cost));
//     }

//     if a_rem_x == 0 && a_rem_y == 0 {
//         let a_cost = (a_count * 3) + b_count + ((target_x / machine.a_x) * 3);
//         cache.insert((target_x, target_y, a_count, b_count), Some(a_cost));
//         return Some(a_cost);
//     }

//     if b_rem_x == 0 && b_rem_y == 0 {
//         let b_cost = (a_count * 3) + b_count + (target_x / machine.b_x);
//         cache.insert((target_x, target_y, a_count, b_count), Some(b_cost));
//         return Some(b_cost);
//     }

//     let option_a = if target_x >= machine.a_x && target_y >= machine.a_y {
//         min_cost(
//             target_x - machine.a_x,
//             target_y - machine.a_y,
//             machine,
//             a_count + 1,
//             b_count,
//             cache,
//         )
//     } else {
//         None
//     };
//     let option_b = if target_x >= machine.b_x && target_y >= machine.b_y {
//         min_cost(
//             target_x - machine.b_x,
//             target_y - machine.b_y,
//             machine,
//             a_count,
//             b_count + 1,
//             cache,
//         )
//     } else {
//         None
//     };

//     cache.insert(
//         (target_x, target_y, a_count, b_count),
//         min_of_options(option_a, option_b),
//     );
//     min_of_options(option_a, option_b)
// }

// fn parse_machines(input: &str) -> Vec<ClawMachine> {
//     let mut a_x: u32 = 0;
//     let mut a_y: u32 = 0;
//     let mut b_x: u32 = 0;
//     let mut b_y: u32 = 0;

//     let mut machines: Vec<ClawMachine> = vec![];

//     let a_pattern = Regex::new(r"Button A: X\+(?<x>[0-9]+), Y\+(?<y>[0-9]+)").unwrap();
//     let b_pattern = Regex::new(r"Button B: X\+(?<x>[0-9]+), Y\+(?<y>[0-9]+)").unwrap();
//     let prize_pattern = Regex::new(r"Prize: X\=(?<x>[0-9]+), Y\=(?<y>[0-9]+)").unwrap();

//     for line in input.lines() {
//         if line.starts_with("Button A") {
//             let caps = a_pattern.captures(line).unwrap();
//             a_x = caps["x"].parse::<u32>().unwrap();
//             a_y = caps["y"].parse::<u32>().unwrap();
//         } else if line.starts_with("Button B") {
//             let caps = b_pattern.captures(line).unwrap();
//             b_x = caps["x"].parse::<u32>().unwrap();
//             b_y = caps["y"].parse::<u32>().unwrap();
//         } else if line.starts_with("Prize") {
//             let caps = prize_pattern.captures(line).unwrap();
//             let target_x = caps["x"].parse::<u32>().unwrap();
//             let target_y = caps["y"].parse::<u32>().unwrap();
//             machines.push(ClawMachine::new(a_x, a_y, b_x, b_y, target_x, target_y));
//         }
//     }

//     machines
// }

pub fn part_one(_input: &str) -> Option<u32> {
    // let machines = parse_machines(input);
    // let mut total: u32 = 0;
    // for machine in machines {
    //     // which gets closer - 3*B or 1*A?
    //     let mut machine_cache = HashMap::new();
    //     if let Some(v) = min_cost(
    //         machine.prize_x,
    //         machine.prize_y,
    //         &machine,
    //         0,
    //         0,
    //         &mut machine_cache,
    //     ) {
    //         total += v;
    //     }
    // }

    // Some(total)
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
