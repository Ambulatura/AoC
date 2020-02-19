fn main() {
    let (lower_limit, upper_limit): (u32, u32) = (172851, 675869);

    let result = find_passwords(lower_limit, upper_limit);

    println!("How many different passwords within the range given in your puzzle input meet all of the criteria? {}", result);
}

fn find_passwords(lower_limit: u32, upper_limit: u32) -> u32 {
    let mut password: [u8; 6] = [0; 6];
    let mut digit_counts: [u8; 10];
    let mut twice: bool;
    let mut more_than_twice: bool;
    let mut counter = 0;

    for mut i in lower_limit..upper_limit + 1 {
        for digit in (0..6).rev() {
            password[5 - digit] = (i / 10_u32.pow(digit as u32)) as u8;
            i = i % 10_u32.pow(digit as u32);
        }

        if password[1] < password[0]
            || password[2] < password[1]
            || password[3] < password[2]
            || password[4] < password[3]
            || password[5] < password[4]
        {
            continue;
        } else {
            digit_counts = [0; 10];
            for digit in password.iter() {
                digit_counts[*digit as usize] = digit_counts[*digit as usize] + 1;
            }

            twice = false;
            more_than_twice = false;
            for count in digit_counts.iter() {
                if *count > 2 {
                    more_than_twice = true;
                } else if *count == 2 {
                    twice = true;
                } else {
                    continue;
                }
            }

            match (twice, more_than_twice) {
                (true, _) => counter = counter + 1,
                _ => continue,
            }
        }
    }
    counter
}
