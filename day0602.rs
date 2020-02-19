use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let filename = "input.txt";
    let objects = fs::read_to_string(filename).expect("Failed to read file!");
    let mut objects_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in objects.lines() {
        let mut splitted = line.split(')');
        let orbits = objects_map
            .entry(splitted.next().unwrap())
            .or_insert_with(Vec::new);
        orbits.push(splitted.next().unwrap());
    }

    let mut you_path: HashSet<&str> = HashSet::new();
    let mut san_path: HashSet<&str> = HashSet::new();
    find_orbital_path(&objects_map, "COM", &mut Vec::new(), "YOU", &mut you_path);
    find_orbital_path(&objects_map, "COM", &mut Vec::new(), "SAN", &mut san_path);
    let required_transfers = you_path.symmetric_difference(&san_path);
    let required_transfers_count = required_transfers.count();

    println!("What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting?
{:?}", required_transfers_count);
}

fn find_orbital_path<'a, 'b>(
    objects_map: &'a HashMap<&'b str, Vec<&'b str>>,
    name: &'b str,
    path: &'a mut Vec<&'b str>,
    target_name: &'b str,
    target_path: &'a mut HashSet<&'b str>,
) {
    let mut path = path.clone();
    path.push(name);
    if let Some(orbit_list) = objects_map.get(name) {
        for orbit in orbit_list {
            if *orbit == target_name {
                for object in path.iter() {
                    target_path.insert(object);
                }
            }
            find_orbital_path(objects_map, orbit, &mut path, target_name, target_path);
        }
    }
}
