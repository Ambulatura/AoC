use std::fs;

use num::integer::gcd;

fn main() {
    let filename = "input.txt";
    let asteroids = fs::read_to_string(filename).expect("Failed to read file!");
    let mut asteroid_map: Vec<Vec<String>> = Vec::new();

    for line in asteroids.trim().lines() {
        asteroid_map.push(line.chars().map(|a| a.to_string()).collect::<Vec<_>>());
    }

    // Monitoring station is from day 10 part 1
    let monitoring_station = (26, 29);
    let asteroid = vaporize_asteroids(monitoring_station, 200, &asteroid_map);

    println!("The Elves are placing bets on which will be the 200th asteroid to be vaporized. Win the bet by determining which asteroid that will be; what do you get if you multiply its X coordinate by 100 and then add its Y coordinate?\n{}", asteroid.0 * 100 + asteroid.1);
}

fn vaporize_asteroids(
    center: (i32, i32),
    nth_vapourized: usize,
    asteroid_map: &[Vec<String>],
) -> (i32, i32) {
    let asteroids = find_asteroids(asteroid_map);
    let sorted_clockwise = sort_clockwise(center, &asteroids);
    let mut vaporized_asteroids: Vec<(i32, i32)> = Vec::new();
    let mut slope_points: Vec<(i32, i32)> = Vec::new();

    for other in sorted_clockwise.iter() {
        let dx = other.0 - center.0;
        let dy = other.1 - center.1;
        let gcd = gcd(dx, dy);
        let reduced_dx = dx / gcd;
        let reduced_dy = dy / gcd;
        if !slope_points.contains(&(reduced_dx, reduced_dy))
            && !vaporized_asteroids.contains(&other)
        {
            vaporized_asteroids.push(*other);
            slope_points.push((reduced_dx, reduced_dy));
        }
    }

    vaporized_asteroids[nth_vapourized - 1]
}

// https://stackoverflow.com/questions/6989100/sort-points-in-clockwise-order/6989383#6989383
// https://en.wikipedia.org/wiki/Cross_product#Computational_geometry
fn closer(center: (i32, i32), other1: (i32, i32), other2: (i32, i32)) -> bool {
    if center.0 <= other1.0 && center.0 > other2.0 {
        return true;
    } else if center.0 > other1.0 && center.0 <= other2.0 {
        return false;
    } else if center.0 == other1.0 && center.0 == other2.0 {
        if other1.1 <= center.1 || other2.1 <= center.1 {
            return other1.1 > other2.1;
        } else {
            return other2.1 > other1.1;
        }
    }

    let cross_product = (other1.0 - center.0) * (other2.1 - center.1)
        - (other2.0 - center.0) * (other1.1 - center.1);

    match cross_product {
        cross_product if cross_product > 0 => true,
        cross_product if cross_product < 0 => false,
        cross_product if cross_product == 0 => {
            let distance1 = (other1.0 - center.0) * (other1.0 - center.0)
                + (other1.1 - center.1) * (other1.1 - center.1);
            let distance2 = (other2.0 - center.0) * (other2.0 - center.0)
                + (other2.1 - center.1) * (other2.1 - center.1);

            distance1 < distance2
        }
        _ => unreachable!(),
    }
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

fn sort_clockwise(center: (i32, i32), asteroids: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut clockwise: Vec<((i32, i32), i32)> = Vec::new();
    let mut asteroids = asteroids.to_owned();
    let center_position = asteroids.iter().position(|x| *x == center).unwrap();
    asteroids.remove(center_position);

    for other1 in asteroids.iter() {
        let mut counter = 0;
        for other2 in asteroids.iter() {
            if other1 != other2 {
                for ((x, y), _) in clockwise.iter() {
                    if (*x, *y) == *other2 {
                        continue;
                    }
                }
                let closer = closer(center, *other1, *other2);
                if closer {
                    counter += 1;
                }
            }
        }
        clockwise.push((*other1, counter));
    }
    clockwise.sort_by(|a, b| a.1.cmp(&b.1));
    clockwise.reverse();
    clockwise
        .iter()
        .map(|(a, _)| (a.0, a.1))
        .collect::<Vec<(i32, i32)>>()
}
