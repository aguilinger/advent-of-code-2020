use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    test_calculate_accumulator();

    let instructions = read_data("src/input.txt");
    let acc = calculate_accumulator(&instructions);
    println!("looped acc {}", acc);

    let working_instruction = find_working_instruction(&instructions);
    let working_acc = calculate_accumulator(&working_instruction);
    println!("working acc {}", working_acc);
}

fn read_data(filename: &str) -> Vec<String> {
    let file = File::open(filename)
    .expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    let passes = reader.lines().map(|line| line.unwrap()).collect();

    return passes;
}

struct BootCode {
    accumulator: i64,
    index_pointer: usize,
    executed_lines: HashSet<usize>,
}

impl BootCode {
    pub fn new() -> BootCode {
        BootCode {
            accumulator: 0,
            index_pointer: 0,
            executed_lines: HashSet::new(),
        }
    }

    pub fn run_until_loop(&mut self, instructions: &Vec<String>) -> i64 {
        loop {
            if self.executed_lines.contains(&self.index_pointer) 
            || self.index_pointer == instructions.len() {
                break;
            }
            self.executed_lines.insert(self.index_pointer);
            let instruction = &instructions[self.index_pointer];
            if instruction.starts_with("acc") {
                self.acc(&instruction);
            }
            else if instruction.starts_with("jmp") {
                self.jmp(&instruction);
            }
            else {
                self.index_pointer += 1;
            }
        }

        return self.accumulator;
    }

    pub fn does_loop(&mut self, instructions: &Vec<String>) -> bool {
        self.run_until_loop(instructions);
        let does_loop = self.index_pointer == instructions.len();
        self.reset();
        return does_loop;
    }

    pub fn acc(&mut self, acc_instruction: &str) {
        let get_acc_count = |instruction| get_number_from_instruction("acc", instruction);
        self.accumulator += get_acc_count(acc_instruction);
        self.index_pointer += 1;
    }

    pub fn jmp(&mut self, jmp_instruction: &str) {
        let get_jmp_count = |instruction| get_number_from_instruction("jmp", instruction);
        self.index_pointer = (self.index_pointer as i64 + get_jmp_count(jmp_instruction)) as usize;
    }

    pub fn reset(&mut self) {
        self.index_pointer = 0;
        self.executed_lines.clear();
    }
}

fn get_number_from_instruction(command: &str, instruction: &str) -> i64 {
    let mut number = instruction.strip_prefix(command).unwrap().trim_start();
    if number.starts_with("+") {
        number = number.strip_prefix("+").unwrap();
    }

    return number.parse().unwrap_or(0);
}


fn calculate_accumulator(instructions: &Vec<String>) -> i64 {
    let mut code = BootCode::new();
    return code.run_until_loop(instructions);
}

fn find_working_instruction(instructions: &Vec<String>) -> Vec<String> {
    let mut code = BootCode::new();
    let mut last_jmp_changed = 0;
    let mut last_nop_changed = 0;
    let final_instructions: Vec<String>;
    loop {
        let mut loop_instructions = instructions.clone();
        for (index, instruction) in loop_instructions.iter().enumerate() {
            if instruction.starts_with("jmp") && index > last_jmp_changed {
                loop_instructions[index] = "nop".to_string();
                last_jmp_changed = index;
                break;
            }
            else if instruction.starts_with("nop") && index > last_nop_changed {
                loop_instructions[index] = "jmp".to_string();
                last_nop_changed = index;
                break;
            }
        }
        if code.does_loop(&loop_instructions) {
            final_instructions = loop_instructions;
            break;
        }
        else {

        }
    }
    return final_instructions.to_vec();
}


fn test_calculate_accumulator() {
    let instructions = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6".lines().map(|s| s.to_string() ).collect();
    assert_eq!(calculate_accumulator(&instructions), 5);
    let working = find_working_instruction(&instructions);

    assert_eq!(calculate_accumulator(&working), 8);

}