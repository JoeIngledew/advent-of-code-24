use hashbrown::HashMap;

advent_of_code::solution!(11);

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
        let digits = value.to_string().len() as u32;
        if digits % 2 != 0 {
            return blink_rec_2(value * 2024, remaining_steps - 1, cache);
        }
        let divisor = 10usize.pow(digits / 2);

        let left = value / divisor;
        let right = value % divisor;
        let l_res = blink_rec_2(left, remaining_steps - 1, cache);
        let r_res = blink_rec_2(right, remaining_steps - 1, cache);
        l_res + r_res
    };

    cache.insert((value, remaining_steps), res);
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let nums: Vec<usize> = input
        .split(' ')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut res_cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut final_res: usize = 0;
    for n in nums {
        let this_res = blink_rec_2(n, 25, &mut res_cache);
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
