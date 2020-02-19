fn main() {
    let (lower_limit, upper_limit): (u32, u32) = (172851, 675869);

    let result = find_passwords(lower_limit, upper_limit);

    println!("How many different passwords within the range given in your puzzle input meet these criteria? {}", result);
}

fn find_passwords(lower_limit: u32, upper_limit: u32) -> u32 {
    let mut password: [u32; 6] = [0, 0, 0, 0, 0, 0];
    let mut counter = 0;
    let mut same_adjacent: bool;
    let mut last: u32;

    for mut i in lower_limit..upper_limit + 1 {
        for digit in (0..6).rev() {
            password[5 - digit] = i / 10_u32.pow(digit as u32);
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
            same_adjacent = false;
            last = password[0];
            for j in 1..password.len() {
                if password[j] == last {
                    same_adjacent = true;
                } else {
                    last = password[j];
                }
            }
            if same_adjacent {
                counter = counter + 1;
            }
        }
    }
    counter
}
