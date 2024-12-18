advent_of_code::solution!(17);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Computer {
    reg_a: i128,
    reg_b: i128,
    reg_c: i128,
    output: Vec<i128>,
}

impl Computer {
    fn with_a(&self, new_a: i128) -> Computer {
        Computer {
            reg_a: new_a,
            reg_b: self.reg_b,
            reg_c: self.reg_c,
            output: self.output.clone(),
        }
    }

    fn with_b(&self, new_b: i128) -> Computer {
        Computer {
            reg_a: self.reg_a,
            reg_b: new_b,
            reg_c: self.reg_c,
            output: self.output.clone(),
        }
    }

    fn with_c(&self, new_c: i128) -> Computer {
        Computer {
            reg_a: self.reg_a,
            reg_b: self.reg_b,
            reg_c: new_c,
            output: self.output.clone(),
        }
    }

    fn with_appended_out(&self, new_out_val: i128) -> Computer {
        let mut new_vec = self.output.clone();
        new_vec.push(new_out_val);
        Computer {
            reg_a: self.reg_a,
            reg_b: self.reg_b,
            reg_c: self.reg_c,
            output: new_vec,
        }
    }
}

fn process_combo_op(comp: &Computer, op: u8) -> i128 {
    match op {
        0..=3 => op as i128,
        4 => comp.reg_a,
        5 => comp.reg_b,
        6 => comp.reg_c,
        _ => panic!("unknown operand"),
    }
}

fn adv(comp: &Computer, pointer: usize, instructions: &[u8]) -> (Computer, usize) {
    let combo = process_combo_op(comp, instructions[pointer + 1]);
    let divisor = 2i128.checked_pow(combo as u32);
    let result = match divisor {
        Some(0) => comp.reg_a,
        Some(d) => comp.reg_a / d,
        None => comp.reg_a,
    };
    let new_comp = comp.with_a(result);
    (new_comp, pointer + 2)
}

fn bxl(comp: &Computer, pointer: usize, instructions: &[u8]) -> (Computer, usize) {
    let operand = instructions[pointer + 1] as i128;
    let result = comp.reg_b ^ operand;
    let new_comp = comp.with_b(result);
    (new_comp, pointer + 2)
}

fn bst(comp: &Computer, pointer: usize, instructions: &[u8]) -> (Computer, usize) {
    let combo = process_combo_op(comp, instructions[pointer + 1]);
    let result = combo.rem_euclid(8);
    let new_comp = comp.with_b(result);
    (new_comp, pointer + 2)
}

fn jnz(comp: &Computer, pointer: usize, instructions: &[u8]) -> (Computer, usize) {
    let should_jmp = comp.reg_a != 0;
    if should_jmp {
        (comp.clone(), instructions[pointer + 1] as usize)
    } else {
        (comp.clone(), pointer + 2)
    }
}

fn bxc(comp: &Computer, pointer: usize, _instructions: &[u8]) -> (Computer, usize) {
    let result = comp.reg_b ^ comp.reg_c;
    let new_comp = comp.with_b(result);
    (new_comp, pointer + 2)
}

fn out(comp: &Computer, pointer: usize, instructions: &[u8]) -> (Computer, usize) {
    let combo = process_combo_op(comp, instructions[pointer + 1]);
    let val = combo.rem_euclid(8);
    let new_comp = comp.with_appended_out(val);
    (new_comp, pointer + 2)
}

fn bdv(comp: &Computer, pointer: usize, instructions: &[u8]) -> (Computer, usize) {
    let combo = process_combo_op(comp, instructions[pointer + 1]);
    let divisor = 2i128.checked_pow(combo as u32);
    let result = match divisor {
        Some(0) => comp.reg_a,
        Some(d) => comp.reg_a / d,
        None => comp.reg_a,
    };
    let new_comp = comp.with_b(result);
    (new_comp, pointer + 2)
}

fn cdv(comp: &Computer, pointer: usize, instructions: &[u8]) -> (Computer, usize) {
    let combo = process_combo_op(comp, instructions[pointer + 1]);
    let divisor = 2i128.checked_pow(combo as u32);
    let result = match divisor {
        Some(0) => comp.reg_a,
        Some(d) => comp.reg_a / d,
        None => comp.reg_a,
    };
    let new_comp = comp.with_c(result);
    (new_comp, pointer + 2)
}

fn run_instruction(
    comp: &Computer,
    instruction: u8,
    pointer: usize,
    instructions: &[u8],
) -> Computer {
    let (state, next_pointer) = match instruction {
        0 => adv(comp, pointer, instructions),
        1 => bxl(comp, pointer, instructions),
        2 => bst(comp, pointer, instructions),
        3 => jnz(comp, pointer, instructions),
        4 => bxc(comp, pointer, instructions),
        5 => out(comp, pointer, instructions),
        6 => bdv(comp, pointer, instructions),
        7 => cdv(comp, pointer, instructions),
        _ => panic!("unknown instruction"),
    };
    if next_pointer >= instructions.len() {
        state
    } else {
        let next_instr = instructions[next_pointer];
        run_instruction(&state, next_instr, next_pointer, instructions)
    }
}

fn parse_input(input: &str) -> (Computer, Vec<u8>) {
    let lines: Vec<&str> = input.lines().collect();

    let reg_a = lines[0].split(' ').last().unwrap().parse::<i128>().unwrap();
    let reg_b = lines[1].split(' ').last().unwrap().parse::<i128>().unwrap();
    let reg_c = lines[2].split(' ').last().unwrap().parse::<i128>().unwrap();
    let prog: Vec<u8> = lines[4]
        .split(' ')
        .last()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let comp = Computer {
        reg_a,
        reg_b,
        reg_c,
        output: Vec::new(),
    };
    (comp, prog)
}

pub fn part_one(input: &str) -> Option<String> {
    let (comp, instructions) = parse_input(input);
    let result = run_instruction(&comp, 0, 0, &instructions);
    let final_output = result
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Some(final_output)
}

pub fn part_two(_input: &str) -> Option<i128> {
    // let mut start_a: i128 = 1;
    // let (comp, instructions) = parse_input(input);
    // let first_instruct = instructions[0];
    // let mut solved = false;
    // let expected = &instructions.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");

    // while (start_a as u32) < u32::MAX && !solved {
    //     let loop_comp = comp.with_a(start_a);
    //     let final_out = run_instruction(&loop_comp, first_instruct, 0, &instructions);
    //     let out_res = final_out.output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    //     if &out_res == expected {
    //         solved = true;
    //     } else {
    //         start_a += 1;
    //     }
    // }

    // Some(start_a)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
