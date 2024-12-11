use hashbrown::HashMap;

advent_of_code::solution!(11);

// fn process_stone(stone: &String) -> Vec<String> {
//     if stone == "0" {
//         return vec!["1".to_string()];
//     }

//     if stone.len() % 2 == 0 {
//         let (first, last) = stone.split_at(stone.len() / 2);
//         let last_without_leading_zeroes = last.trim_start_matches('0');
//         let last = if last_without_leading_zeroes.is_empty() {
//             "0"
//         } else {
//             last_without_leading_zeroes
//         };
//         return vec![first.to_string(), last.to_string()];
//     }

//     let parsed = stone.parse::<usize>().expect(format!("Cannot parse stone: {}", stone).as_str());
//     let new_val = parsed * 2024;
//     let new_stone = new_val.to_string();
//     vec![new_stone]
// }

fn process_single(value: &usize) -> Vec<usize> {
    if *value == 0 {
        return vec![1];
    }

    let as_string = value.to_string();
    let len = as_string.len();
    let is_even_digits = len % 2 == 0;
    if is_even_digits {
        let (left, right) = as_string.split_at(len / 2);
        return vec![
            left.parse::<usize>().unwrap(),
            right.parse::<usize>().unwrap(),
        ];
    }

    let new_val = *value * 2024;
    vec![new_val]
}

fn blink_rec(
    value: usize,
    remaining_steps: usize,
    process_cache: &mut HashMap<usize, Vec<usize>>,
    result_cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if remaining_steps == 0 {
        return 1;
    }

    // check if we've already run to end with this initial number and remaining count
    if let Some(v) = result_cache.get(&(value, remaining_steps)) {
        return *v;
    }

    let next = process_cache
        .entry(value)
        .or_insert_with(|| process_single(&value))
        .clone();
    next.into_iter()
        .map(|v| blink_rec(v, remaining_steps - 1, process_cache, result_cache))
        .sum()
}

fn blink_rec_2(
    value: usize,
    remaining_steps: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if remaining_steps == 0 {
        return 1;
    }

    if let Some(&v) = cache.get(&(value, remaining_steps)) {
        return v;
    }

    let res = if value == 0 {
        blink_rec_2(1, remaining_steps - 1, cache)
    } else {
        let digits = (value as f64).log10().floor() as u32 + 1;
        if digits % 2 != 0 {
            return blink_rec_2(value * 2024, remaining_steps - 1, cache);
        }
        let divisor = 10usize.pow(digits / 2);

        let left = value / divisor;
        let right = value % divisor;
        blink_rec_2(left, remaining_steps - 1, cache)
            + blink_rec_2(right, remaining_steps - 1, cache)
    };

    cache.insert((value, remaining_steps), res);
    res
}

// fn blink(stones: Vec<String>) -> Vec<String> {
//     stones.iter().map(|s| process_stone(s)).concat()
// }

// fn read_input(input: &str) -> Vec<String> {
//     let stones: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();
//     stones
// }

pub fn part_one(input: &str) -> Option<usize> {
    // let mut init = read_input(input);
    // for _ in 0..25 {
    //     init = blink(init);
    // }
    // Some(init.len())
    let nums: Vec<usize> = input
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut process_cache: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut res_cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut final_res: usize = 0;
    for n in nums {
        let this_res = blink_rec(n, 25, &mut process_cache, &mut res_cache);
        final_res += this_res;
    }
    Some(final_res)
}

pub fn part_two(input: &str) -> Option<usize> {
    let nums: Vec<usize> = input
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    //let mut process_cache: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut res_cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut final_res: usize = 0;
    for n in nums {
        let this_res = blink_rec_2(n, 75, &mut res_cache);
        final_res += this_res;
    }
    Some(final_res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    // no part 2 example
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
