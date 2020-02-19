use std::fs;

const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const STOP: u32 = 99;
const VALUE: u32 = 19690720;

fn main() {
    let filename = "input.txt";
    let mut intcodes: Vec<u32> = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .split(",")
        .map(|x| x.parse().expect("Failed to parse numbers!"))
        .collect();

    let (value, result) = process_intcode(&mut intcodes);
    println!("100 * noun + verb = {} for value {}", result, value);
}

fn process_intcode(intcodes: &mut Vec<u32>) -> (u32, u32) {
    let copy_intcodes: Vec<u32> = intcodes.clone();
    let mut i: usize;
    let mut opcode: u32;
    let mut values: (u32, u32);
    let mut index: usize;

    for noun in 0..100 {
        for verb in 0..100 {
            i = 0;
            *intcodes = copy_intcodes.to_vec();
            intcodes[1] = noun;
            intcodes[2] = verb;
            while i < intcodes.len() {
                opcode = intcodes[i];
                values = (
                    intcodes[intcodes[i + 1] as usize],
                    intcodes[intcodes[i + 2] as usize],
                );
                index = intcodes[i + 3] as usize;
                match opcode {
                    ADD => {
                        intcodes[index] = values.0 + values.1;
                        i += 4;
                    }
                    MULTIPLY => {
                        intcodes[index] = values.0 * values.1;
                        i += 4;
                    }
                    STOP => {
                        if intcodes[0] == VALUE {
                            println!("Program finished successfully!");
                            return (VALUE, 100 * noun + verb);
                        } else {
                            break;
                        }
                    }
                    _ => {
                        println!("Something went wrong!");
                        return (intcodes[0], STOP);
                    }
                }
            }
        }
    }
    println!(
        "Program finished but can't find the {} at position 0.",
        VALUE
    );
    (intcodes[0], STOP)
}
