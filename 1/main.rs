use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_mass(line: std::io::Result<String>, input_filename: String) -> i32 {
    let mass_string = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", input_filename, why),
        Ok(line) => line
    };

    return match mass_string.parse() {
        Err(why) => panic!("Couldn't parse \"{}\" into a number: {}", mass_string, why),
        Ok(mass) => mass
    };
}

fn parse_file(file_name: &Path) -> Vec<i32> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();
    
    return lines
         .map(|line| parse_mass(line, file_name.display().to_string()))
         .collect();
}

fn calculate_fuel(mass: i32) -> i32 {
    // anything that would produce negative (or 0) fuel should be treated as if it had 0 fuel requirement
    if mass <= 6 {
        return 0;
    } else {
        return mass / 3 - 2;
    }
}

fn calculate_fuel_fuel(module_fuel: i32) -> i32 {
    let mut extra_fuel: i32 = 0;
    let mut fuel = calculate_fuel(module_fuel);
    while fuel > 0 {
        extra_fuel += fuel;
        fuel = calculate_fuel(fuel);
    }

    return extra_fuel;
}

fn calculate_module_fuel(masses: Vec<i32>) -> i32 {
    let mut total_fuel: i32 = 0;
    for mass in masses {
        let fuel = calculate_fuel(mass);
        total_fuel += fuel;

        println!("Fuel needed for module with mass {}: {}", mass, fuel);

        let fuel_fuel: i32 = calculate_fuel_fuel(fuel);

        total_fuel += fuel_fuel;
        println!("Extra fuel needed: {}", fuel_fuel);
    }

    return total_fuel;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);


    let masses = parse_file(&input_file);
    let total_fuel = calculate_module_fuel(masses);
    println!("Total fuel required: {}", total_fuel);
}