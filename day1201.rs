use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs,
};

struct Coordinates {
    x: i32,
    y: i32,
    z: i32,
}

struct Moon<'a> {
    name: &'a str,
    position: Coordinates,
    velocity: Coordinates,
}

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

    let mut moons: [Moon; 4] = [
        Moon {
            name: "Io",
            position: Coordinates {
                x: moon_positions[0][0],
                y: moon_positions[0][1],
                z: moon_positions[0][2],
            },
            velocity: Coordinates { x: 0, y: 0, z: 0 },
        },
        Moon {
            name: "Europa",
            position: Coordinates {
                x: moon_positions[1][0],
                y: moon_positions[1][1],
                z: moon_positions[1][2],
            },
            velocity: Coordinates { x: 0, y: 0, z: 0 },
        },
        Moon {
            name: "Ganymede",
            position: Coordinates {
                x: moon_positions[2][0],
                y: moon_positions[2][1],
                z: moon_positions[2][2],
            },
            velocity: Coordinates { x: 0, y: 0, z: 0 },
        },
        Moon {
            name: "Callisto",
            position: Coordinates {
                x: moon_positions[3][0],
                y: moon_positions[3][1],
                z: moon_positions[3][2],
            },
            velocity: Coordinates { x: 0, y: 0, z: 0 },
        },
    ];

    simulate_motions(&mut moons, 1000);
}

fn calculate_motions(moons: &mut [Moon]) {
    apply_gravity(moons);
    apply_velocity(moons);
}

fn simulate_motions(moons: &mut [Moon], steps: i32) {
    for step in 0..=steps {
        if step != 0 {
            calculate_motions(moons);
        }
    }
    println!("-After {} steps-", steps);

    let mut sum_total_energy = 0;

    for m in moons.iter() {
        let total_energy = calculate_potential_energy(m) * calculate_kinetic_energy(m);
        println!("Total energy of {:>8}: {:>5}", m.name, total_energy);
        sum_total_energy += total_energy;
    }

    println!("\nTotal energy in the all system: {}", sum_total_energy);
}

fn apply_gravity(moons: &mut [Moon]) {
    for m in 0..moons.len() {
        for n in m..moons.len() {
            match moons[m].position.x.cmp(&moons[n].position.x) {
                Greater => {
                    moons[m].velocity.x -= 1;
                    moons[n].velocity.x += 1;
                }
                Less => {
                    moons[m].velocity.x += 1;
                    moons[n].velocity.x -= 1;
                }
                Equal => {}
            }

            match moons[m].position.y.cmp(&moons[n].position.y) {
                Greater => {
                    moons[m].velocity.y -= 1;
                    moons[n].velocity.y += 1;
                }
                Less => {
                    moons[m].velocity.y += 1;
                    moons[n].velocity.y -= 1;
                }
                Equal => {}
            }
            match moons[m].position.z.cmp(&moons[n].position.z) {
                Greater => {
                    moons[m].velocity.z -= 1;
                    moons[n].velocity.z += 1;
                }
                Less => {
                    moons[m].velocity.z += 1;
                    moons[n].velocity.z -= 1;
                }
                Equal => {}
            }
        }
    }
}

fn apply_velocity(moons: &mut [Moon]) {
    for m in moons.iter_mut() {
        m.position.x += m.velocity.x;
        m.position.y += m.velocity.y;
        m.position.z += m.velocity.z;
    }
}

fn calculate_potential_energy(moon: &Moon) -> i32 {
    moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs()
}

fn calculate_kinetic_energy(moon: &Moon) -> i32 {
    moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs()
}
