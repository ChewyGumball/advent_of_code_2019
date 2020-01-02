use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn execute_opcode(starting_position: usize, program: &mut Vec<usize>) {
    let operation = program[starting_position];
    let operand_1 = program[program[starting_position + 1]];
    let operand_2 = program[program[starting_position + 2]];
    let destination = program[starting_position + 3];

    let result = match operation {
        1 => operand_1 + operand_2,
        2 => operand_1 * operand_2,
        _ => panic!("Unknown operation code: {}", operation)
    };

    program[destination] = result;
    
    //println!("Operation: {}, on {} {} = {}, into {}", operation, operand_1, operand_2, result, destination);
}

fn execute_program(program: &mut Vec<usize>) {
    let mut position = 0;
    while program[position] != 99 {
        execute_opcode(position, program);
        position += 4;
    }
}

fn parse_program(line: &String) -> Vec<usize> {
    return line.split(",")
               .map(|opcode| opcode.parse::<usize>())
               .filter_map(Result::ok)
               .collect();
}

fn parse_file(file_name: &Path) -> Vec<usize> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let mut line = String::new();
    let _ = match io::BufReader::new(file).read_line(&mut line) {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name.display(), why),
        Ok(line) => line
    };

    return parse_program(&line);
}

fn find_answer(program: &Vec<usize>, answer: usize) -> Result<(usize, usize), &str> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program_instance = program.to_vec();

            //Fix up error
            program_instance[1] = noun;
            program_instance[2] = verb;    
            execute_program(&mut program_instance);
            println!("Noun: {}, Verb: {} produces {}", noun, verb, program_instance[0]);
            
            if program_instance[0] == answer {
                return Ok((noun, verb));
            }
        }
    }

    return Err("Could not find an answer!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let initial_program = parse_file(&input_file);

    let (noun, verb) = match find_answer(&initial_program, 19690720) {
        Err(why) => panic!("{}", why),
        Ok(value) => value
    };
    
    let magic_number = (100 * noun) + verb;
    println!("Found noun={}, verb={} (magic number: {})", noun, verb, magic_number);
}