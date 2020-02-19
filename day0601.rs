use std::{collections::HashMap, fs};

fn main() {
    let filename = "input.txt";
    let objects = fs::read_to_string(filename).expect("Failed to read file!");
    let mut objects_map = HashMap::new();

    for line in objects.lines() {
        let mut splitted = line.split(')');
        let orbits = objects_map
            .entry(splitted.next().unwrap())
            .or_insert_with(Vec::new);
        orbits.push(splitted.next().unwrap());
    }

    println!(
        "What is the total number of direct and indirect orbits in your map data?\n{}",
        calculate_orbits(&objects_map, "COM", 0)
    );
}

fn calculate_orbits(objects_map: &HashMap<&str, Vec<&str>>, name: &str, count: u32) -> u32 {
    let mut result = count;
    if let Some(orbit_list) = objects_map.get(name) {
        for orbit in orbit_list {
            result += calculate_orbits(objects_map, orbit, count + 1);
        }
    }
    result
}
