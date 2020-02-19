use std::fs;

const ADD: i128 = 1;
const MULTIPLY: i128 = 2;
const INPUT: i128 = 3;
const OUTPUT: i128 = 4;
const JUMP_TRUE: i128 = 5;
const JUMP_FALSE: i128 = 6;
const LESS_THAN: i128 = 7;
const EQUAL_TO: i128 = 8;
const ADJUST_RELATIVE: i128 = 9;
const STOP: i128 = 99;
const INPUT_VALUE: i128 = 1;

fn main() {
    let filename = "input.txt";
    let mut intcodes = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .split(',')
        .map(|x| x.parse::<i128>().expect("Failed to parse numbers!"))
        .collect::<Vec<i128>>();
    process_intcodes(&mut intcodes);
}

fn process_intcodes(intcodes: &mut Vec<i128>) {
    let mut i: usize = 0;
    let mut opcode: i128;
    let mut intcode: String;
    let mut index: i128;
    let mut all_params: [u32; 3];
    let mut params: Vec<u32>;
    let mut relative_base: i128 = 0;
    let mut memory: Vec<i128> = vec![0; 100_000_000];

    memory[..intcodes.len()].copy_from_slice(&intcodes[..]);
    *intcodes = memory;
    while i < intcodes.len() {
        intcode = intcodes[i].to_string();
        all_params = [0; 3];
        if intcode.len() == 1 {
            opcode = intcodes[i];
        } else {
            opcode = intcode[intcode.len() - 2..].parse::<i128>().unwrap();
            params = intcode[..intcode.len() - 2]
                .chars()
                .rev()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            all_params[..params.len()].clone_from_slice(&params[..]);
        };

        match opcode {
            ADD => {
                let mut values: (i128, i128) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else if all_params[0] == 0 {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                } else {
                    values.0 = intcodes[(intcodes[i + 1] + relative_base) as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else if all_params[1] == 0 {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                } else {
                    values.1 = intcodes[(intcodes[i + 2] + relative_base) as usize];
                }
                if all_params[2] == 2 {
                    index = intcodes[i + 3] + relative_base;
                } else {
                    index = intcodes[i + 3];
                }
                intcodes[index as usize] = values.0 + values.1;
                i += 4;
            }
            MULTIPLY => {
                let mut values: (i128, i128) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else if all_params[0] == 0 {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                } else {
                    values.0 = intcodes[(intcodes[i + 1] + relative_base) as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else if all_params[1] == 0 {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                } else {
                    values.1 = intcodes[(intcodes[i + 2] + relative_base) as usize];
                }

                if all_params[2] == 2 {
                    index = intcodes[i + 3] + relative_base;
                } else {
                    index = intcodes[i + 3];
                }
                intcodes[index as usize] = values.0 * values.1;
                i += 4;
            }
            INPUT => {
                if all_params[0] == 1 {
                    index = i as i128 + 1;
                } else if all_params[0] == 0 {
                    index = intcodes[i + 1];
                } else {
                    index = intcodes[i + 1] + relative_base;
                }
                intcodes[index as usize] = INPUT_VALUE;
                i += 2;
            }
            OUTPUT => {
                if all_params[0] == 1 {
                    index = i as i128 + 1;
                } else if all_params[0] == 0 {
                    index = intcodes[i + 1];
                } else {
                    index = intcodes[i + 1] + relative_base;
                }
                println!("Diagnostic code: {}", intcodes[index as usize]);
                i += 2;
            }
            JUMP_TRUE => {
                let mut values: (i128, i128) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else if all_params[0] == 0 {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                } else {
                    values.0 = intcodes[(intcodes[i + 1] + relative_base) as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else if all_params[1] == 0 {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                } else {
                    values.1 = intcodes[(intcodes[i + 2] + relative_base) as usize];
                }
                if values.0 != 0 {
                    i = values.1 as usize;
                } else {
                    i += 3;
                }
            }
            JUMP_FALSE => {
                let mut values: (i128, i128) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else if all_params[0] == 0 {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                } else {
                    values.0 = intcodes[(intcodes[i + 1] + relative_base) as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else if all_params[1] == 0 {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                } else {
                    values.1 = intcodes[(intcodes[i + 2] + relative_base) as usize];
                }
                if values.0 == 0 {
                    i = values.1 as usize;
                } else {
                    i += 3;
                }
            }
            LESS_THAN => {
                let mut values: (i128, i128) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else if all_params[0] == 0 {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                } else {
                    values.0 = intcodes[(intcodes[i + 1] + relative_base) as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else if all_params[1] == 0 {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                } else {
                    values.1 = intcodes[(intcodes[i + 2] + relative_base) as usize];
                }
                if all_params[2] == 2 {
                    index = intcodes[i + 3] + relative_base;
                } else {
                    index = intcodes[i + 3];
                }
                if values.0 < values.1 {
                    intcodes[index as usize] = 1;
                } else {
                    intcodes[index as usize] = 0;
                }
                i += 4;
            }
            EQUAL_TO => {
                let mut values: (i128, i128) = (0, 0);
                if all_params[0] == 1 {
                    values.0 = intcodes[i + 1];
                } else if all_params[0] == 0 {
                    values.0 = intcodes[intcodes[i + 1] as usize];
                } else {
                    values.0 = intcodes[(intcodes[i + 1] + relative_base) as usize];
                }
                if all_params[1] == 1 {
                    values.1 = intcodes[i + 2];
                } else if all_params[1] == 0 {
                    values.1 = intcodes[intcodes[i + 2] as usize];
                } else {
                    values.1 = intcodes[(intcodes[i + 2] + relative_base) as usize];
                }
                if all_params[2] == 2 {
                    index = intcodes[i + 3] + relative_base;
                } else {
                    index = intcodes[i + 3];
                }
                if values.0 == values.1 {
                    intcodes[index as usize] = 1;
                } else {
                    intcodes[index as usize] = 0;
                }
                i += 4;
            }
            ADJUST_RELATIVE => {
                if all_params[0] == 1 {
                    relative_base += intcodes[i + 1];
                } else if all_params[0] == 0 {
                    relative_base += intcodes[intcodes[i + 1] as usize];
                } else {
                    relative_base += intcodes[(intcodes[i + 1] + relative_base) as usize];
                }
                i += 2;
            }
            STOP => {
                println!("Program ran successfully!");
                break;
            }
            _ => unreachable!(),
        }
    }
}
