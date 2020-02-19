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

fn main() {
    let filename = "input.txt";
    let mut intcodes = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().expect("Failed to parse numbers!"))
        .collect::<Vec<i32>>();

    println!(
        "What is the highest signal that can be sent to the thrusters?
{}",
        process_intcodes(&mut intcodes)
    );
}

fn process_intcodes(intcodes: &mut Vec<i32>) -> i32 {
    let copy_intcodes = intcodes.clone();
    let phase_settings = phase_settings(0, 4);
    let mut amplifiers: [[i32; 3]; 5];
    let mut i: usize;
    let mut opcode: i32;
    let mut intcode: String;
    let mut index: usize;
    let mut all_params: [u32; 3];
    let mut params: Vec<u32>;
    let mut input_count;
    let mut thruster = 0;

    for phase in phase_settings.iter() {
        amplifiers = [[0; 3]; 5];
        amplifiers[0][0] = phase[0];
        amplifiers[1][0] = phase[1];
        amplifiers[2][0] = phase[2];
        amplifiers[3][0] = phase[3];
        amplifiers[4][0] = phase[4];

        for amplifier in 0..amplifiers.len() {
            *intcodes = copy_intcodes.clone();
            input_count = 0;
            i = 0;
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
                        if input_count == 0 {
                            intcodes[index] = amplifiers[amplifier as usize][0];
                            input_count += 1;
                        } else {
                            intcodes[index] = amplifiers[amplifier as usize][1];
                            input_count = 0;
                        }
                        i += 2;
                    }
                    OUTPUT => {
                        if all_params[0] == 1 {
                            index = i + 1;
                        } else {
                            index = intcodes[i + 1] as usize;
                        }
                        if amplifier == 4 {
                            if intcodes[index] > thruster {
                                thruster = intcodes[index];
                                println!("New thruster code: {}", thruster);
                            }
                        } else {
                            amplifiers[amplifier as usize + 1][1] = intcodes[index];
                        }
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
    }
    thruster
}

fn phase_settings(start: i32, end: i32) -> Vec<[i32; 5]> {
    let mut settings: Vec<[i32; 5]> = Vec::new();
    for i in start..=end {
        for j in start..=end {
            if i == j {
                continue;
            };
            for k in start..=end {
                if i == k || j == k {
                    continue;
                }
                for l in start..=end {
                    if i == l || j == l || k == l {
                        continue;
                    }
                    for m in start..=end {
                        if i == m || j == m || k == m || l == m {
                            continue;
                        }
                        settings.push([i, j, k, l, m]);
                    }
                }
            }
        }
    }
    settings
}
