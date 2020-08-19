use std::fs::{File, read_to_string};

pub fn main() {
    let file = read_to_string("src/Day1/day1.txt").expect("File not found");
    let mass: Vec<u32> = file.lines().map(parse_module).collect();

    // Part 1
    let mut sum = 0;
    for x in mass.iter() {
        sum += (x / 3) - 2;
    }

    println!("Fuel required: {}", sum);

    // Part 2
    let mut total_sum = 0;
    for x in mass {
        let mut dividend= x;

        while dividend > 0 {
            dividend = (dividend / 3).checked_sub(2).unwrap_or(0);
            total_sum += dividend;
        }
    }

    println!("Total fuel required: {}", total_sum);
}

fn parse_module(line: &str) -> u32 {
    line.parse().expect("Line was not an integer")
}