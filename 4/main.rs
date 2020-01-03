fn is_match(number: u32) -> bool {
    let digits = number.to_string();

    let mut previous_value =  digits.chars().nth(0).unwrap().to_digit(10).unwrap();
    let mut has_matching_adjacent_values = false;
    let mut matching_adjacent_value_count = 0;

    for digit in digits.chars().skip(1) {
        let value: u32 = digit.to_digit(10).unwrap();

        if value == previous_value {
            matching_adjacent_value_count += 1;
        } else { 
            if matching_adjacent_value_count == 1 { 
                has_matching_adjacent_values = true;
            }

            matching_adjacent_value_count = 0;
        }

        if value < previous_value {
            return false;
        }

        previous_value = value;
    }

    return has_matching_adjacent_values || matching_adjacent_value_count == 1;
}

fn main() {
    let mut matching_passwords = 0;
    for i in 359282..820402 {
        if is_match(i) {
            matching_passwords += 1;
            println!("{} is a match!", i);
        }
    }

    println!("{} matching passwords.", matching_passwords);
}