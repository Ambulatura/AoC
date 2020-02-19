use std::fs;

const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JUMP_TRUE: i32 = 5;
const JUMP_FALSE: i32 = 6;
const LESS_THAN: i32 = 7;
const EQUAL_TO: i32 = 8;
const STOP: i32 = 99;
const SYSTEM_ID: i32 = 5;

fn main() {
    let filename = "input.txt";
    let mut intcodes = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().expect("Failed to parse numbers!"))
        .collect::<Vec<i32>>();

    process_intcodes(&mut intcodes);
}

fn process_intcodes(intcodes: &mut Vec<i32>) {
    let mut i: usize = 0;
    let mut opcode: i32;
    let mut intcode: String;
    let mut index: usize;
    let mut all_params: [u32; 3];
    let mut params: Vec<u32>;

    while i < intcodes.len() {
        intcode = intcodes[i].to_string();
        all_params = [0; 3];
        if intcode.len() == 1 {
            opcode = intcodes[i];
        } else {
            opcode = intcode[intcode.len() - 2..].parse::<i32>().unwrap();
            params = intcode[..intcode.len() - 2]
                .chars()
                .rev()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            all_params[..params.len()].clone_from_slice(&params[..]);
        };

        match opcode {
            ADD => {
                let mut values: (i32, i32) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                }
                index = intcodes[i + 3] as usize;
                intcodes[index] = values.0 + values.1;
                i += 4;
            }
            MULTIPLY => {
                let mut values: (i32, i32) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                }
                index = intcodes[i + 3] as usize;
                intcodes[index] = values.0 * values.1;
                i += 4;
            }
            INPUT => {
                if all_params[0] == 1 {
                    index = i + 1;
                } else {
                    index = intcodes[i + 1] as usize;
                }
                intcodes[index] = SYSTEM_ID;
                i += 2;
            }
            OUTPUT => {
                if all_params[0] == 1 {
                    index = i + 1;
                } else {
                    index = intcodes[i + 1] as usize;
                }
                println!("Diagnostic code: {}", intcodes[index]);
                i += 2;
            }
            JUMP_TRUE => {
                let mut values: (i32, i32) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                }
                if values.0 != 0 {
                    i = values.1 as usize;
                } else {
                    i += 3;
                }
            }
            JUMP_FALSE => {
                let mut values: (i32, i32) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                }
                if values.0 == 0 {
                    i = values.1 as usize;
                } else {
                    i += 3;
                }
            }
            LESS_THAN => {
                let mut values: (i32, i32) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                }
                index = intcodes[i + 3] as usize;
                if values.0 < values.1 {
                    intcodes[index] = 1;
                } else {
                    intcodes[index] = 0;
                }
                i += 4;
            }
            EQUAL_TO => {
                let mut values: (i32, i32) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                }
                index = intcodes[i + 3] as usize;
                if values.0 == values.1 {
                    intcodes[index] = 1;
                } else {
                    intcodes[index] = 0;
                }
                i += 4;
            }
            STOP => {
                println!("Program ran successfully!");
                break;
            }
            _ => unreachable!(),
        }
    }
}
