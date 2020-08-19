mod Day1 { pub mod day1; }
mod Day2 { pub mod day2; }
mod Day3 { pub mod day3; }
mod Day4 { pub mod day4; }

use std::time::{SystemTime, UNIX_EPOCH, Instant};

fn main() {
    let start = Instant::now();
    Day3::day3::main();

    let elapsed = start.elapsed();
    println!("Time Ended: {:?}", elapsed);
}

