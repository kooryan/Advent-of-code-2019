use std::fs;
use std::ops::{Sub, BitAnd};


pub fn main() {
    let file = fs::read_to_string("src/Day4/day4.txt").expect("No file found");

    let mut pass_range: Vec<u32> = file
        .trim()
        .split("-")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("1. Possibilities: {}", calculate_combinations(pass_range.clone()));
    println!("2. Possibilities: {}", constrained_combinations(pass_range.clone()));

}

fn split_number(n: u32) -> Vec<u32> {
    let digits: Vec<u32> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    digits
}

fn repeated_twice(mut num: Vec<u32>) -> bool {
    let mut count = 0;

    (0..5).any(|i| match i { // taken
        0 => (num[0] == num[1]) && (num[0] != num[2]),
        4 => (num[4] == num[5]) && (num[4] != num[3]),
        n => (num[n] == num[n + 1]) && (num[n] != num[n - 1]) && (num[n] != num[n + 2]),
    })
}

fn constrained_combinations (mut range: Vec<u32>) -> u32 {
    let mut count = 0;

    for x in range[0]..=range[1] {
        let mut number = split_number(x);

        if is_increasing(number.clone())
            && (repeated_twice(number.clone())) {
            println!("Array is: {:?}", number);
            count += 1;
        }
    }

    count
}

fn calculate_combinations(mut range: Vec<u32>) -> u32 {

    let mut count = 0;

    for x in range[0]..=range[1] {
        let mut number = split_number(x);

        if is_increasing(number.clone())
            && is_repeating(number.clone()) {
            count += 1;
        }
    }

    count
}


fn is_repeating(num: Vec<u32>) -> bool {
    let mut valid = false;
    for x in 0..5 {
        if num[x+1] == num[x] {
            valid = true;
        }
    }

    valid
}

fn is_increasing(num: Vec<u32>) -> bool  {
    let mut valid = false;

    for x in 0..5 {
        if num[x+1] >= num[x] {
            valid = true;
        } else {
            valid = false;
            break;
        }
    }

    valid
}