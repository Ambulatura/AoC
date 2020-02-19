use std::{collections::HashMap, fs};

use num::integer::gcd;

fn main() {
    let filename = "input.txt";
    let asteroids = fs::read_to_string(filename).expect("Failed to read file!");
    let mut asteroid_map: Vec<Vec<String>> = Vec::new();

    for line in asteroids.trim().lines() {
        asteroid_map.push(line.chars().map(|a| a.to_string()).collect::<Vec<_>>());
    }

    let detected_asteroids = detect_asteroids(&asteroid_map);
    let monitoring_station = detected_asteroids
        .iter()
        .max_by_key(|a| a.1.len())
        .map(|a| ((a.0), a.1.len()))
        .unwrap();

    let monitoring_station_location = monitoring_station.0;
    let monitoring_station_asteroids = monitoring_station.1;

    print_detected(&asteroid_map, *monitoring_station_location);

    println!(
        "\nMonitoring station location {:?}\nDetected asteroids: {}",
        monitoring_station_location, monitoring_station_asteroids
    );
}

fn find_asteroids(asteroid_map: &[Vec<String>]) -> Vec<(i32, i32)> {
    let mut asteroids: Vec<(i32, i32)> = Vec::new();

    for y in 0..asteroid_map.len() {
        for x in 0..asteroid_map[0].len() {
            if asteroid_map[y][x] == "#" {
                asteroids.push((x as i32, y as i32));
            }
        }
    }
    asteroids
}

fn detect_asteroids(asteroid_map: &[Vec<String>]) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let asteroid_locations = find_asteroids(asteroid_map);
    let mut detected_asteroids: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for center in asteroid_locations.iter() {
        let sorted_locations = sort_locations(*center, &asteroid_locations);
        let mut slope_points: Vec<(i32, i32)> = Vec::new();
        let mut visible_asteroids: Vec<(i32, i32)> = Vec::new();
        for other in sorted_locations.iter() {
            let dx = other.0 - center.0;
            let dy = other.1 - center.1;
            let gcd = gcd(dx, dy);
            let reduced_dx = dx / gcd;
            let reduced_dy = dy / gcd;
            let slope_point = (reduced_dx, reduced_dy);
            if !slope_points.contains(&slope_point) {
                slope_points.push(slope_point);
                visible_asteroids.push((other.0, other.1));
            }
        }
        detected_asteroids
            .entry((center.0, center.1))
            .or_insert(visible_asteroids);
    }
    detected_asteroids
}

fn sort_locations(center: (i32, i32), asteroid_locations: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut distance: i32;
    let mut distances: Vec<((i32, i32), i32)> = Vec::new();
    for (x, y) in asteroid_locations.iter() {
        if (*x, *y) != center {
            distance = ((y - center.1) + (x - center.0)).abs();
            distances.push(((*x, *y), distance));
        }
    }

    distances.sort_by(|a, b| a.1.cmp(&b.1));

    let sorted_locations: Vec<(i32, i32)> = distances.iter().map(|((a, b), _)| (*a, *b)).collect();
    sorted_locations
}

fn print_detected(asteroid_map: &[Vec<String>], location: (i32, i32)) {
    println!("Asteroid map:");
    for y in 0..asteroid_map.len() {
        print!(" ");
        for x in 0..asteroid_map[0].len() {
            if asteroid_map[y][x] == "#" {
                if (x as i32, y as i32) == location {
                    print!("S");
                } else {
                    print!("#");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}
