advent_of_code::solution!(2);

fn to_num_vecs(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').filter_map(|x| x.parse::<i32>().ok()).collect())
        .collect()
}

fn diff(a: i32, b: i32) -> i32 {
    a - b
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    Increase,
    Decrease,
}

fn is_vec_safe(input: &[i32]) -> bool {
    let (is_safe, _, _) = input.iter().fold((true, None, None), |acc, curr| {
        let (passing, prev, direction) = acc;
        if !passing {
            return acc;
        }
        if let Some(v) = prev {
            let diff_prev = diff(v, *curr);
            if diff_prev == 0 {
                return (false, None, None);
            }
            let new_direction = if diff_prev > 0 && diff_prev < 4 {
                Some(Direction::Decrease)
            } else if diff_prev < 0 && diff_prev > -4 {
                Some(Direction::Increase)
            } else {
                None
            };
            let is_safe = if let Some(dir) = direction {
                new_direction.is_some_and(|ndir| ndir == dir) && diff_prev != 0
            } else {
                passing && diff_prev < 4 && diff_prev > -4 && diff_prev != 0
            };
            (is_safe, Some(*curr), new_direction)
        } else {
            (true, Some(*curr), None)
        }
    });
    // println!("vec {:?} is safe? {}", input, is_safe);
    is_safe
}

// fn is_vec_safe_v2(input: &Vec<i32>) -> bool {
//     let (is_safe, _, _, _) = input.iter().fold((true, None, None, false), |acc, curr| {
//         let (passing, prev, direction, skipped) = acc;
//         if !passing {
//             return acc;
//         }
//         if let Some(v) = prev {
//             let diff_prev = diff(v, *curr);
//             if diff_prev == 0 {
//                 if skipped {
//                     return (false, None, None, false);
//                 } else {
//                     return (passing, prev, direction, true);
//                 }
//             }
//             let new_direction = if diff_prev > 0 && diff_prev < 4 {
//                 Some(Direction::Decrease)
//             } else if diff_prev < 0 && diff_prev > -4 {
//                 Some(Direction::Increase)
//             } else {
//                 None
//             };
//             let is_safe = if let Some(dir) = direction {
//                 new_direction.is_some_and(|ndir| ndir == dir) && diff_prev != 0
//             } else {
//                 passing && diff_prev < 4 && diff_prev > -4 && diff_prev != 0
//             };
//             if !is_safe && !skipped {
//                 (passing, prev, direction, true)
//             } else {
//                 (is_safe, Some(*curr), new_direction, false)
//             }
//         } else {
//             return (true, Some(*curr), None, false);
//         }
//     });
//     println!("vec {:?} is safe? {}", input, is_safe);
//     is_safe
// }

fn is_unsafe_vec_tolerable(xs: &[i32]) -> bool {
    let len = xs.len();
    for x in 0..len {
        let mut new_vec = xs.to_owned();
        new_vec.remove(x);
        if is_vec_safe(&new_vec) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let x = to_num_vecs(input)
        .into_iter()
        .filter(|xs| is_vec_safe(xs))
        .count();
    Some(x)
}

pub fn part_two(input: &str) -> Option<usize> {
    let safe_and_unsafe_og: Vec<(bool, Option<Vec<i32>>)> = to_num_vecs(input)
        .into_iter()
        .map(|xs| {
            if is_vec_safe(&xs) {
                (true, None)
            } else {
                (false, Some(xs))
            }
        })
        .collect();
    let safe_count = safe_and_unsafe_og
        .clone()
        .into_iter()
        .filter(|(a, _)| *a)
        .count();
    let unsafe_vecs: Vec<Vec<i32>> = safe_and_unsafe_og
        .clone()
        .into_iter()
        .filter_map(|(_, vs)| vs)
        .collect();
    let addnl_count = unsafe_vecs
        .into_iter()
        .filter(|xs| is_unsafe_vec_tolerable(xs))
        .count();
    Some(safe_count + addnl_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
