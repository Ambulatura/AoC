use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let filename = "input.txt";
    let wire_paths = fs::read_to_string(filename).expect("Failed to read file!");
    let wire_paths = wire_paths.trim().split("\n").collect::<Vec<&str>>();
    let wire_paths: (Vec<&str>, Vec<&str>) = (
        wire_paths[0].split(",").collect(),
        wire_paths[1].split(",").collect(),
    );

    let first_wire = set_wire(wire_paths.0);
    let second_wire = set_wire(wire_paths.1);
    let first_keys = first_wire.keys().collect::<HashSet<_>>();
    let second_keys = second_wire.keys().collect::<HashSet<_>>();
    let intersections = first_keys.intersection(&second_keys).collect::<Vec<_>>();

    let result = intersections
        .iter()
        .map(|position| first_wire[position] + second_wire[position])
        .min();

    println!(
        "What is the fewest combined steps the wires must take to reach an intersection? {}",
        result.unwrap()
    );
}

fn set_wire(wire_path: Vec<&str>) -> HashMap<(i32, i32), u32> {
    let mut steps: HashMap<(i32, i32), u32> = HashMap::new();
    let (mut x, mut y, mut wire_length) = (0, 0, 0);
    let mut direction: &str;
    let mut to_position: u32;

    for path in wire_path.iter() {
        direction = &path[0..1];
        to_position = path[1..].parse::<u32>().unwrap();

        let (dx, dy) = match direction {
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (1, 0),
            "D" => (-1, 0),
            _ => unreachable!(),
        };

        for _ in 1..to_position + 1 {
            x += dx;
            y += dy;
            wire_length += 1;
            steps.entry((x, y)).or_insert(wire_length);
        }
    }

    steps
}
