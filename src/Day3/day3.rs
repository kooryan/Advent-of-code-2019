use std::fs;
use std::collections::HashMap;

pub fn main() {
    let file = fs::read_to_string("src/Day3/day3.txt").expect("No file found");

    let mut wires: Vec<String> = file
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut wire1: Vec<String> = wires[0].split(",").map(|s| s.parse().unwrap()).collect();
    let mut wire2: Vec<String> = wires[1].split(",").map(|s| s.parse().unwrap()).collect();

    println!("Wire length: {}", wires.len());
}

fn calculate_distance(mut wire1: Vec<u32>, mut wire2: Vec<u32>) {
  //  let mut wire = HashMap::new();

    let mut closestDistance = std::u32::MAX;




}



