use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let file = match File::open(&input_file) {
        Err(why) => panic!("Couldn't open {}: {}", input_file.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    let mut total_fuel: i32 = 0;

    for line in lines {
        let mass_string = match line {
            Err(why) => panic!("Couldn't read line from {}: {}", input_file.display(), why),
            Ok(line) => line
        };

        let mass: i32 = match mass_string.parse() {
            Err(why) => panic!("Couldn't parse \"{}\" into a number: {}", mass_string, why),
            Ok(mass) => mass
        };

        let fuel = (mass / 3) - 2;
        total_fuel += fuel;

        println!("Fuel needed for mass {}: {}", mass, fuel);
    }

    println!("Total fuel required: {}", total_fuel);
}