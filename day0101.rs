use std::fs;

fn main() {
    let filename = "input.txt";
    let fuels: Vec<u32> = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .split("\n")
        .map(|x| x.parse::<u32>().expect("Failed to parse numbers!"))
        .collect();

    println!(
        "Sum of the fuel requirements for all modules is {}",
        calculate_total_fuel(fuels)
    );
}

fn calculate_total_fuel(fuels: Vec<u32>) -> u32 {
    let mut sum_fuel: u32 = 0;

    for fuel in fuels {
        sum_fuel += fuel / 3 - 2;
    }
    sum_fuel
}
