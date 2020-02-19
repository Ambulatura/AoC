use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "input.txt";
    let wire_paths = fs::read_to_string(filename).expect("Failed to read file!");
    let wire_paths: Vec<&str> = wire_paths.trim().split("\n").collect();
    let wire_paths: (Vec<&str>, Vec<&str>) = (
        wire_paths[0].split(",").collect(),
        wire_paths[1].split(",").collect(),
    );
    let first_path = set_wire(wire_paths.0);
    let second_path = set_wire(wire_paths.1);
    let intersections = first_path.intersection(&second_path).collect::<Vec<_>>();

    let result = intersections.iter().map(|(x, y)| x.abs() + y.abs()).min();

    println!(
        "Manhattan distance from central port to the closest distance is {}",
        result.unwrap()
    );
}

fn set_wire(wire_path: Vec<&str>) -> HashSet<(i32, i32)> {
    let mut steps: HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0, 0);
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
            steps.insert((x, y));
        }
    }
    steps
}
