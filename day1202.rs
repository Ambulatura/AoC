use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs,
};

use num::integer::lcm;

fn main() {
    let filename = "input.txt";
    let moon_positions = fs::read_to_string(filename).expect("Failed to read file!");
    let moon_positions = moon_positions
        .trim()
        .replacen(char::is_alphabetic, "", 12)
        .replacen("<", "", 4)
        .replacen(">", "", 4)
        .replacen("=", "", 12)
        .replacen(",", "", 8);
    let moon_positions: Vec<_> = moon_positions
        .split('\n')
        .map(|a| {
            a.split(' ')
                .map(|b| b.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut pvx: Vec<(i32, i32)> = moon_positions.iter().map(|p| (p[0], 0)).collect();
    let mut pvy: Vec<(i32, i32)> = moon_positions.iter().map(|p| (p[1], 0)).collect();
    let mut pvz: Vec<(i32, i32)> = moon_positions.iter().map(|p| (p[2], 0)).collect();

    let x_steps = simulate_motions(&mut pvx);
    let y_steps = simulate_motions(&mut pvy);
    let z_steps = simulate_motions(&mut pvz);
    let result = lcm(x_steps, lcm(y_steps, z_steps));

    println!("How many steps does it take to reach the first state that exactly matches a previous state?\n{}", result);
}

fn calculate_motions(pv: &mut Vec<(i32, i32)>) {
    apply_gravity(pv);
    apply_velocity(pv);
}

fn simulate_motions(pv: &mut Vec<(i32, i32)>) -> u128 {
    let first_state = pv.clone();
    let mut step: u128 = 0;

    loop {
        calculate_motions(pv);
        step += 1;
        if *pv == first_state {
            break;
        }
    }
    step
}

fn apply_gravity(pv: &mut Vec<(i32, i32)>) {
    for i in 0..pv.len() {
        for j in i + 1..pv.len() {
            match pv[i].0.cmp(&pv[j].0) {
                Greater => {
                    pv[i].1 -= 1;
                    pv[j].1 += 1;
                }
                Less => {
                    pv[i].1 += 1;
                    pv[j].1 -= 1;
                }
                Equal => {}
            }
        }
    }
}

fn apply_velocity(pv: &mut Vec<(i32, i32)>) {
    for (p, v) in pv.iter_mut() {
        *p += *v;
    }
}
