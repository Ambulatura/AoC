use std::fs;

fn main() {
    let filename = "input.txt";
    let masses: Vec<i32> = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .split("\n")
        .map(|x| x.parse().unwrap())
        .collect();

    println!(
        "Sum of the all fuel requirements is {}",
        calculate_total_fuel(masses)
    );
}

fn calculate_total_fuel(masses: Vec<i32>) -> i32 {
    let mut sum_fuel: i32 = 0;
    let calculate_fuel = |x: i32| -> i32 { x / 3 - 2 };

    for mass in masses {
        let mut fuel = calculate_fuel(mass);
        while fuel >= 0 {
            sum_fuel += fuel;
            fuel = calculate_fuel(fuel);
        }
    }
    sum_fuel
}
