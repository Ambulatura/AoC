use std::fs;

const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const STOP: u32 = 99;

fn main() {
    let filename = "input.txt";

    let mut intcodes: Vec<u32> = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .split(",")
        .map(|x| x.parse().expect("Failed to parse numbers!"))
        .collect();

    //let mut intcodes: Vec<u32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    println!(
        "Value of the position 0 is {}",
        process_intcode(&mut intcodes)
    );
}

fn process_intcode(intcodes: &mut Vec<u32>) -> u32 {
    let mut i: usize = 0;
    let mut opcode: u32;
    let mut values: (u32, u32);
    intcodes[1] = 12;
    intcodes[2] = 2;
    while i < intcodes.len() {
        opcode = intcodes[i];
        let index = (intcodes[i + 1] as usize, intcodes[i + 2] as usize);
        values = (intcodes[index.0], intcodes[index.1]);
        i += 3;

        if opcode == STOP {
            println!("Program finished.");
            break;
        } else if opcode == ADD {
            let index = intcodes[i] as usize;
            intcodes[index] = values.0 + values.1;
            i += 1;
        } else if opcode == MULTIPLY {
            let index = intcodes[i] as usize;
            intcodes[index] = values.0 * values.1;
            i += 1;
        } else {
            println!("Something went wrong!");
            break;
        }
    }
    intcodes[0]
}
