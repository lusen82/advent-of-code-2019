use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::Display;


#[test]
fn version() {
    let test1 = "3,9,8,9,10,9,4,9,99,-1,8";
    assert_that!(&["--version"], starts_with("rustfmt "));
    assert_that!(&["--version"], starts_with("rustfmt "));
    assert_that!(&["--", "-V"], starts_with("rustfmt "));
    assert_that!(&["--", "--version"], starts_with("rustfmt "));
}

pub fn day_5() {
    let mut file = File::open("inp5.txt").unwrap();
    let reader = BufReader::new(file);
    let inp: Vec<String> = reader.lines().into_iter().map(|d| {
        let numbers = d.unwrap();
        numbers
    }).collect();
    let first_line = inp.first().unwrap();
    println!("First line {}", first_line);
    let numbers_i: Vec<&str> = first_line.split(',').collect();
    println!("n {:?}", &numbers_i);
    let numbers: Vec<i32> = numbers_i.into_iter().map(|integ| integ.parse::<i32>().unwrap()).collect();

    let result_vec: Vec<i32> = numbers.clone();
    let mut memory: Memory = Memory {result_vec: result_vec, pointer: 0  };
    while memory.pointer < numbers.len() {
        let option = **&memory.result_vec.get(memory.pointer).unwrap();
//        println!("Option is {}", &option);
//        println!(" Pointer is {} ", memory.pointer);
        match option {
            1 => memory = Instr_01::new("Instruction 1").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            2 => memory = Instr_02::new("Instruction 2").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            3 => memory = Instr_03::new("Instruction 3").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            4 =>memory = Instr_04::new("Instruction 4").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            99 => {
                memory = Memory {result_vec: memory.result_vec, pointer: numbers.len()};
            },
            _ => {
                // "ABCDE of instruction where DE is the instruction op:
                let instruction_op = option % 100;
                //println!("parameter mde {}", &instruction_op);

                let parameter_mode = option % 1000;
                let paameter_mode = parameter_mode / 100;
               // println!("First parameter mde {}", paameter_mode);

                let paameter_mode2 = option / 1000;
               // println!("Second parameter mde {}", paameter_mode2);

                let param_1_mode = match paameter_mode {
                    0 => ParamMode::POSITION,
                    1 => ParamMode::IMMEDIATE,
                    _ => panic!("Wrong mode!")
                };

                let param_2_mode = match paameter_mode2 {
                    0 => ParamMode::POSITION,
                    1 => ParamMode::IMMEDIATE,
                    _ => panic!("Wrong mode!")
                };

                memory = match instruction_op {
                    1 => Instr_01::new("Instruction 1").execute(memory, param_1_mode, param_2_mode),
                    2 => Instr_02::new("Instruction 2").execute(memory, param_1_mode, param_2_mode),
                    3 => Instr_03::new("Instruction 3").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
                    4 => Instr_04::new("Instruction 4").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
                    99 => Memory {result_vec: memory.result_vec, pointer: numbers.len()},
                    _ => panic!("Other instr op")
                };
            }
        }
    }

}


pub fn day_5_b() {
    let mut file = File::open("inp5.txt").unwrap();
    let reader = BufReader::new(file);
    let inp: Vec<String> = reader.lines().into_iter().map(|d| {
        let numbers = d.unwrap();
        numbers
    }).collect();
    let first_line = inp.first().unwrap();
    println!("First line {}", first_line);
    let numbers_i: Vec<&str> = first_line.split(',').collect();
    println!("n {:?}", &numbers_i);
    let numbers: Vec<i32> = numbers_i.into_iter().map(|integ| integ.parse::<i32>().unwrap()).collect();

    let result_vec: Vec<i32> = numbers.clone();
    let mut memory: Memory = Memory {result_vec: result_vec, pointer: 0  };
    while memory.pointer < numbers.len() {
        let option = **&memory.result_vec.get(memory.pointer).unwrap();
        println!("Option is {}", &option);
        println!(" Pointer is {} ", memory.pointer);
        match option {
            1 => memory = Instr_01::new("Instruction 1").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            2 => memory = Instr_02::new("Instruction 2").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            3 => memory = Instr_03::new("Instruction 3").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            4 => memory = Instr_04::new("Instruction 4").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            5 => memory = Instr_05::new("Instruction 5").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            6 => memory = Instr_06::new("Instruction 6").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            7 => memory = Instr_07::new("Instruction 7").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            8 =>memory = Instr_08::new("Instruction 8").execute(memory, ParamMode::POSITION, ParamMode::POSITION),
            99 => {
                memory = Memory {result_vec: memory.result_vec, pointer: numbers.len()};
            },
            _ => {
                // "ABCDE of instruction where DE is the instruction op:
                println!("Option: {}", option);
                let instruction_op = option % 100;
                println!("Instruction: {}", &instruction_op);

                let parameter_mode = option % 1000;
                let paameter_mode = parameter_mode / 100;
                // println!("First parameter mde {}", paameter_mode);

                let paameter_mode2 = option / 1000;
                // println!("Second parameter mde {}", paameter_mode2);

                let param_1_mode = match paameter_mode {
                    0 => ParamMode::POSITION,
                    1 => ParamMode::IMMEDIATE,
                    _ => panic!("Wrong mode!")
                };

                let param_2_mode = match paameter_mode2 {
                    0 => ParamMode::POSITION,
                    1 => ParamMode::IMMEDIATE,
                    _ => panic!("Wrong mode!")
                };

                memory = match instruction_op {
                    1 => Instr_01::new("Instruction 1").execute(memory, param_1_mode, param_2_mode),
                    2 => Instr_02::new("Instruction 2").execute(memory, param_1_mode, param_2_mode),
                    3 => Instr_03::new("Instruction 3").execute(memory, param_1_mode, param_2_mode),
                    4 => Instr_04::new("Instruction 4").execute(memory, param_1_mode, param_2_mode),
                    5 => Instr_05::new("Instruction 5").execute(memory, param_1_mode, param_2_mode),
                    6 => Instr_06::new("Instruction 6").execute(memory, param_1_mode, param_2_mode),
                    7 => Instr_07::new("Instruction 7").execute(memory, param_1_mode, param_2_mode),
                    8 => Instr_08::new("Instruction 8").execute(memory, param_1_mode, param_2_mode),
                    99 => Memory {result_vec: memory.result_vec, pointer: numbers.len()},
                    _ => panic!("Other instr op")
                };
            }
        }
    }

}
fn get_input() -> i32 {
    return 5;
}

struct Memory {
    result_vec: Vec<i32>,
    pointer: usize,
}

struct Instr_01 {
    pub name: &'static str
}

impl Instruction for Instr_01 {
    fn new(name: &'static str) -> Self {
        return Instr_01 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {
        //self.log();
        let pointer = memory.pointer;
        println!("Pointer: {} Param 1 Mode: {:?} Param2 mode {:?}", &pointer, &param_1_mode, &param_2_mode);
        let first_value = self.get_value_for_mode((pointer + 1), param_1_mode, &memory);
        let second_value =  self.get_value_for_mode((pointer + 2), param_2_mode, &memory);
        let to_repl_index = self.get_value_for_mode_usize(pointer + 3, ParamMode::IMMEDIATE, &memory);
        println!("FIrst value: {} Second value: {}, index {}", first_value, second_value, to_repl_index);
        memory.result_vec[to_repl_index] = self.combine(first_value, second_value);
        memory.pointer = pointer + 4;
        memory
    }
    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }
    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        return first_param + second_param;
    }
}

struct Instr_02 {
    pub name: &'static str
}

impl Instruction for Instr_02 {
    fn new(name: &'static str) -> Self {
        return Instr_02 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {
        let pointer = memory.pointer;
        let first_value = self.get_value_for_mode((pointer + 1), param_1_mode, &memory);
        let second_value =  self.get_value_for_mode((pointer + 2), param_2_mode, &memory);
        let to_repl_index = self.get_value_for_mode_usize(pointer + 3, ParamMode::IMMEDIATE, &memory);

        memory.result_vec[to_repl_index] = self.combine(first_value, second_value);
        memory.pointer = pointer + 4;
        memory
    }
    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }
    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        return first_param * second_param;
    }
}

struct Instr_03 {
    pub name: &'static str
}

impl Instruction for Instr_03 {
    fn new(name: &'static str) -> Self {
        return Instr_03 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {

        let pointer = memory.pointer;
        println!("Pointer {}", pointer);
        let first_value = self.get_value_for_mode_usize((pointer + 1), ParamMode::IMMEDIATE, &memory);
        println!("to reply index = {}", first_value);

        memory.result_vec[first_value] = get_input();
        memory.pointer = pointer + 2;
        memory
    }

    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }

    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        !unreachable!()
    }
}

struct Instr_04 {
    pub name: &'static str
}

impl Instruction for Instr_04 {
    fn new(name: &'static str) -> Self {
        return Instr_04 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {
        let pointer = memory.pointer;
        let first_value = self.get_value_for_mode_usize((pointer + 1), ParamMode::IMMEDIATE, &memory);
        println!("Result is: {}", first_value);
        println!("Result from 4: {}", &memory.result_vec[first_value]);
        memory.pointer = pointer + 2;
        memory
    }

    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }

    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        !unreachable!()
    }
}
struct Instr_05 {
    pub name: &'static str
}

impl Instruction for Instr_05 {
    fn new(name: &'static str) -> Self {
        return Instr_05 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {
        let pointer = memory.pointer;
        let first_value = self.get_value_for_mode((pointer + 1), param_1_mode, &memory);
        let second_value =  self.get_value_for_mode_usize((pointer + 2), param_2_mode, &memory);
        if first_value != 0 {
            memory.pointer = second_value;
        } else {
            memory.pointer = pointer + 3;
        }
        memory
    }

    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }

    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        !unreachable!()
    }
}
struct Instr_06 {
    pub name: &'static str
}

impl Instruction for Instr_06 {
    fn new(name: &'static str) -> Self {
        return Instr_06 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {
        let pointer = memory.pointer;
        let first_value = self.get_value_for_mode((pointer + 1), param_1_mode, &memory);
        let second_value =  self.get_value_for_mode_usize((pointer + 2), param_2_mode, &memory);
        if first_value == 0 {
            memory.pointer = second_value;
        }
        else {
            memory.pointer = pointer + 3;
        }
        memory
    }

    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }

    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        !unreachable!()
    }
}
struct Instr_07 {
    pub name: &'static str
}

impl Instruction for Instr_07 {
    fn new(name: &'static str) -> Self {
        return Instr_07 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {
        let pointer = memory.pointer;
        let first_value = self.get_value_for_mode((pointer + 1), param_1_mode, &memory);
        let second_value =  self.get_value_for_mode((pointer + 2), param_2_mode, &memory);
        let to_repl_index = self.get_value_for_mode_usize(pointer + 3, ParamMode::IMMEDIATE, &memory);
        if first_value < second_value {
            memory.result_vec[to_repl_index] = 1;

        }
        else {
            memory.result_vec[to_repl_index] = 0;
        }
        memory.pointer = pointer + 4;
        memory
    }

    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }

    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        !unreachable!()
    }
}

struct Instr_08 {
    pub name: &'static str
}

impl Instruction for Instr_08 {
    fn new(name: &'static str) -> Self {
        return Instr_08 {name};
    }

    fn execute(&self, mut memory: Memory, param_1_mode: ParamMode, param_2_mode: ParamMode) -> Memory {
        let pointer = memory.pointer;
        let first_value = self.get_value_for_mode((pointer + 1), param_1_mode, &memory);
        let second_value =  self.get_value_for_mode((pointer + 2), param_2_mode, &memory);
        let to_repl_index = self.get_value_for_mode_usize(pointer + 3, ParamMode::IMMEDIATE, &memory);
        if first_value == second_value {
            memory.result_vec[to_repl_index] = 1;

        }
        else {
            memory.result_vec[to_repl_index] = 0;
        }
        memory.pointer = pointer + 4;
        memory
    }

    fn log(&self) {
        println!("Instruction {} is now running: ", self.name);
    }

    fn combine(&self, first_param: i32, second_param: i32) -> i32 {
        !unreachable!()
    }
}

trait Instruction {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn execute(&self, memory: Memory, mode_1: ParamMode, mode_2: ParamMode) -> Memory;
    fn log(&self);
    // Traits can provide default method definitions.
    fn combine(&self, first_param: i32, second_param: i32) -> i32;

    fn get_value_for_mode(&self, pointer: usize, param_mode: ParamMode, memory: &Memory) -> i32{
        let value: i32 = match param_mode {
            ParamMode::POSITION => {
                let position = *memory.result_vec.get(pointer).unwrap() as usize;
                println!("position : {}", &position);
                *memory.result_vec.get(position).unwrap()
            },
            ParamMode::IMMEDIATE => *memory.result_vec.get(pointer).unwrap(),
            _ => panic!("Errror mode")
        };
        return value
    }

    fn get_value_for_mode_usize(&self, pointer: usize, param_mode: ParamMode, memory: &Memory) -> usize{
        let value = match param_mode {
            ParamMode::POSITION => {
                let position = *memory.result_vec.get(pointer).unwrap() as usize;
                *memory.result_vec.get(position).unwrap()
            },
            ParamMode::IMMEDIATE => *memory.result_vec.get(pointer).unwrap(),
            _ => panic!("Errror mode")
        };
        return value as usize
    }
}

#[derive(Debug)]
enum ParamMode {
    IMMEDIATE,
    POSITION,
}

