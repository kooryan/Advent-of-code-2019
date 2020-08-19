use std::fs;

pub fn main() {

    let file = fs::read_to_string("src/Day2/day2.txt").expect("No file found");

    let mut intcode: Vec<u32> = file
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

//    for mut x in 0..intcode.len() {
//        println!("Number: {:?}", intcode.get(x));
//    }
//
//    println!("Position 1: {:?}", intcode.get(3));
//
//    for (mut i, x) in intcode.iter().enumerate() {
//        println!("In position {} we have value {}", i, x);
//    }

    println!("Day 1: {}", execute_program(intcode.clone(), 12, 2)[0]);
    println!("Day 2: {}", instructions(intcode.clone()))

}

fn execute_program(mut code: Vec<u32>, noun: u32, verb: u32) -> Vec<u32> {

    let mut position: usize = 0;

    code[1] = noun;
    code[2] = verb;

    loop {

        let opcode = code[position];

        match opcode {

                1 => {
                    let sum = code[code[position + 1] as usize] + code[code[position + 2] as usize];
                    let third_value = code[position + 3] as usize;
                    code[third_value] = sum;
                },

                2 => {
                    let product = code[code[position + 1] as usize] * code[code[position + 2] as usize];
                    let third_value = code[position + 3] as usize;
                    code[third_value] = product;
                },

                99 => {
                    break;
                },

                _ => {
                    println!("Something went wrong");
                },
            }

        position += 4;
    }

    code
}

fn instructions(mut memory: Vec<u32>) -> u32 {
    let output = 19690720;

    let mut result: u32 = 0;
    let mut combination: u32 = 0;

    for noun in 0..100 {
        for verb in 0..100 {
            result = execute_program(memory.clone(), noun, verb)[0];
            if result == output {
                combination = 100 * noun + verb;
            }
        }
    }

    combination
}
