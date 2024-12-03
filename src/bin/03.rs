use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
struct MultiInstruct {
    a: u32,
    b: u32,
}

impl MultiInstruct {
    pub fn new(a: u32, b: u32) -> Self {
        MultiInstruct { a, b }
    }

    pub fn result(&self) -> u32 {
        self.a * self.b
    }
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mult(MultiInstruct),
}

fn get_instructions(input: &str) -> Vec<MultiInstruct> {
    let pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut res: Vec<MultiInstruct> = vec![];
    for (_, [a, b]) in pattern.captures_iter(input).map(|c| c.extract()) {
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();
        res.push(MultiInstruct::new(a, b));
    }
    res
}

fn get_instructions_v2(input: &str) -> Vec<Instruction> {
    // funny hack to ensure same number of matches in all regex capture groups :)
    let pattern = Regex::new(r"((d)(o)\(\)|(d)(o)n't\(\)|mul\(([0-9]+),([0-9]+)\))").unwrap();
    let mut res: Vec<Instruction> = vec![];
    for (x, [_, a, b]) in pattern.captures_iter(input).map(|c| c.extract()) {
        if x == "do()" {
            res.push(Instruction::Do);
        } else if x == "don't()" {
            res.push(Instruction::Dont);
        } else {
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            res.push(Instruction::Mult(MultiInstruct::new(a, b)));
        }
    }
    res
}

fn process_instructions(instruct: &[Instruction]) -> u32 {
    let (_, final_res) = instruct.iter().fold((true, 0), |acc, curr| {
        let (is_doing, result) = acc;
        match curr {
            Instruction::Do => (true, result),
            Instruction::Dont => (false, result),
            Instruction::Mult(m) => {
                if is_doing {
                    let new_res = result + m.result();
                    (true, new_res)
                } else {
                    acc
                }
            }
        }
    });
    final_res
}

pub fn part_one(input: &str) -> Option<u32> {
    let output = get_instructions(input).iter().map(|x| x.result()).sum();
    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instr = get_instructions_v2(input);
    let result = process_instructions(&instr);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
