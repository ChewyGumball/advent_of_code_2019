use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum ParameterMode {
    Immediate,
    Position
}

type InstructionHandler = fn(Vec<i32>, &mut Memory, &mut dyn Iterator<Item = &i32>, &mut Vec<i32>);

struct Instruction {
    opcode: u8,
    parameter_count: u8,
    write_parameters: Vec<u8>,
    handler: InstructionHandler
}

impl Instruction {
    fn new_with_writes(opcode: u8, parameter_count: u8, write_parameters: Vec<u8>, handler: InstructionHandler) -> Instruction {
        return Instruction{ opcode, parameter_count, write_parameters, handler};
    }

    fn new(opcode: u8, parameter_count: u8, handler: InstructionHandler) -> Instruction {
        return Instruction::new_with_writes(opcode, parameter_count, Vec::new(), handler);
    }

    fn decode_parameter_mode(value: i32, parameter_position: u8) -> ParameterMode {
        let mut mode = value;

        // We need to get rid of the first two digits as well, so we divide 2 extra times
        for _ in 0..parameter_position + 2 {
            mode /= 10;
        }

        let mode_value = match mode % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown parameter mode: {}!", mode)
        };

        return mode_value;
    }

    fn decode_opcode(value: i32) -> u8 {
        return (value % 100) as u8;
    }

    fn execute(&self, program_counter: i32, memory: &mut Memory, input: &mut dyn Iterator<Item = &i32>, output: &mut Vec<i32>) {
        let mut parameters = Vec::new();
        let opcode = memory.read(program_counter, ParameterMode::Immediate);
        for i in 0..self.parameter_count {
            let parameter_mode = if self.write_parameters.contains(&i) {
                                    ParameterMode::Immediate
                                } else {
                                    Instruction::decode_parameter_mode(opcode, i)
                                };
            let address = program_counter + (i as i32) + 1 as i32;
            let value = memory.read(address, parameter_mode);

            parameters.push(value);
        }

        (self.handler)(parameters, memory, input, output);
    }
}

type Program = Vec<i32>;
struct Memory {
    values: Vec<i32>
}

impl Memory {
    fn initialize(program: &Program) -> Memory {
        return Memory {
            values: program.clone()
        };
    }

    fn read(&self, position: i32, mode: ParameterMode) -> i32 {
        return match mode {
            ParameterMode::Immediate => self.values[position as usize],
            ParameterMode::Position => {
                let address = self.values[position as usize];
                self.values[address as usize]
            }
        };
    }

    fn write(&mut self, position: i32, value: i32) {
        self.values[position as usize] = value;
    }
}

struct IntcodeComputer {
    instructions: HashMap<u8, Instruction>
}

impl IntcodeComputer {
    
    fn new<T>(instructions: T) -> IntcodeComputer 
    where T: IntoIterator<Item = Instruction> 
    {
        let map = instructions.into_iter().map(|instruction| (instruction.opcode, instruction)).collect();
        return IntcodeComputer {
            instructions: map
        };
    }

    fn execute<T>(& self, program: & Program, input: &T, output: & mut Vec<i32>) -> Memory
    where for<'a> &'a T: IntoIterator<Item = &'a i32>
    {
        let mut memory = Memory::initialize(&program);
        let mut program_counter = 0;
        let mut input_iterator = input.into_iter();

        while memory.read(program_counter, ParameterMode::Immediate) != 99 {
            let opcode = Instruction::decode_opcode(memory.read(program_counter, ParameterMode::Immediate));
            let instruction = match self.instructions.get(&opcode) {
                None => panic!("Missing instruction definition for opcode {}!", opcode),
                Some(instruction) => instruction
            };

            instruction.execute(program_counter, &mut memory, &mut input_iterator, output);
            program_counter += instruction.parameter_count as i32 + 1;
        }

        return memory;
    }
}

fn read_program() -> Vec<i32> {
    return vec![3,225,1,225,6,6,1100,1,238,225,104,0,1002,43,69,224,101,-483,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,1101,67,60,225,1102,5,59,225,1101,7,16,225,1102,49,72,225,101,93,39,224,101,-98,224,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1102,35,82,225,2,166,36,224,101,-4260,224,224,4,224,102,8,223,223,101,5,224,224,1,223,224,223,102,66,48,224,1001,224,-4752,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1001,73,20,224,1001,224,-55,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,1102,18,41,224,1001,224,-738,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,1101,68,71,225,1102,5,66,225,1101,27,5,225,1101,54,63,224,1001,224,-117,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1,170,174,224,101,-71,224,224,4,224,1002,223,8,223,1001,224,4,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1007,226,226,224,1002,223,2,223,1006,224,329,1001,223,1,223,1007,226,677,224,102,2,223,223,1006,224,344,1001,223,1,223,108,677,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,8,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,226,224,1002,223,2,223,1005,224,404,101,1,223,223,7,677,226,224,102,2,223,223,1005,224,419,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,434,101,1,223,223,1008,226,677,224,102,2,223,223,1006,224,449,1001,223,1,223,7,226,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,108,677,226,224,102,2,223,223,1005,224,479,101,1,223,223,108,226,226,224,1002,223,2,223,1006,224,494,101,1,223,223,8,226,226,224,1002,223,2,223,1005,224,509,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,524,1001,223,1,223,1107,226,226,224,102,2,223,223,1005,224,539,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,554,101,1,223,223,107,226,677,224,102,2,223,223,1005,224,569,1001,223,1,223,1108,226,677,224,1002,223,2,223,1005,224,584,1001,223,1,223,1107,226,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,1008,226,226,224,1002,223,2,223,1005,224,614,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,629,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,644,101,1,223,223,107,677,677,224,1002,223,2,223,1005,224,659,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226];
}

fn main() {
    let mut instructions = Vec::new();

    // 1: p0 + p1 -> p2
    instructions.push(Instruction::new_with_writes(1, 3, vec![2], |parameters, memory, _input, _output| {
        memory.write(parameters[2], parameters[0] + parameters[1]);
    }));

    // 2: p0 * p1 -> p2
    instructions.push(Instruction::new_with_writes(2, 3, vec![2], |parameters, memory, _input, _output| {
        memory.write(parameters[2], parameters[0] * parameters[1]);
    }));

    // 3: input -> p0
    instructions.push(Instruction::new_with_writes(3, 1, vec![0], |parameters, memory, input, _output| {
        let value = input.next().unwrap();
        memory.write(parameters[0], *value);
    }));

    // 4: p0 -> output
    instructions.push(Instruction::new(4, 1, |parameters, _memory, _input, output| {
        output.push(parameters[0]);
    }));

    let computer = IntcodeComputer::new(instructions);

    let input = vec![1];
    let mut output = Vec::new();

    let program = read_program();

    computer.execute(&program, &input, &mut output);

    println!("Output:\n{:?}", output);
}