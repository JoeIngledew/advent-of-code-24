advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy)]
struct Rule {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct PrntInput {
    order_rule: Vec<Rule>,
    updates: Vec<Vec<u32>>,
}

fn update_is_valid(update: &[u32], rules: &[Rule]) -> bool {
    for rule in rules {
        let mut iter_x = update.iter();
        let mut iter_y = update.iter();
        if let Some(ix_x) = iter_x.position(|p| p == &rule.x) {
            if let Some(ix_y) = iter_y.position(|p| p == &rule.y) {
                if ix_x > ix_y {
                    //println!("update {:?} is INVALID (ixX: {}, ixY: {}, ruleX: {}, ruleY: {})", update, ix_x, ix_y, rule.x, rule.y);
                    return false;
                }
            }
        }
    }

    //println!("update {:?} is valid", update);

    true
}

fn get_print_input(input: &str) -> PrntInput {
    let mut recording_pages = false;
    let mut rules: Vec<Rule> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        if recording_pages {
            let nums: Vec<u32> = line.split(',').map(|c| c.parse::<u32>().unwrap()).collect();
            updates.push(nums);
        } else if line.is_empty() {
            recording_pages = true;
        } else {
            let nums: Vec<u32> = line.split('|').map(|c| c.parse::<u32>().unwrap()).collect();
            let rule = Rule {
                x: nums[0],
                y: nums[1],
            };
            rules.push(rule);
        }
    }

    PrntInput {
        order_rule: rules,
        updates,
    }
}

fn get_middle_number(update: &[u32]) -> u32 {
    let len = update.len();
    let ix = len / 2;
    update[ix]
}

fn make_valid(xs: &mut [u32], rules: &[Rule]) -> Vec<u32> {
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            let (curr, next) = (xs[i], xs[j]);
            let relevant_rules_ys: Vec<u32> =
                rules.iter().filter(|r| r.x == curr).map(|r| r.y).collect();
            if !relevant_rules_ys.contains(&next) {
                (xs[i], xs[j]) = (next, curr)
            }
        }
    }
    Vec::from(xs)
}

pub fn part_one(input: &str) -> Option<u32> {
    let print_input = get_print_input(input);
    let sum = print_input
        .updates
        .iter()
        .filter(|u| update_is_valid(u, &print_input.order_rule))
        .map(|u| get_middle_number(u))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let print_input = get_print_input(input);
    let mut incorrects: Vec<Vec<u32>> = print_input
        .updates
        .iter()
        .filter(|u| !update_is_valid(u, &print_input.order_rule))
        .cloned()
        .collect();
    let sums = incorrects
        .iter_mut()
        .map(|xs| make_valid(xs, &print_input.order_rule))
        .map(|xs| get_middle_number(&xs))
        .sum();
    Some(sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
