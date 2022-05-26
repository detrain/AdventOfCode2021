/*

-- Day 1 Sonar Sweep --

*/

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn get_sonar_data() -> Vec<u32> {
    /*
        Reads input file, converts each line to u32 and pushes to vector
        Returns: Vector of each distance recorded from sonar sensor
    */
    let file = File::open("input").expect("Error: Cannot open file...");
    let mut sonar_data: Vec<u32> = Vec::new();
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line.unwrap();
        let distance: u32 = line.parse().unwrap();
        sonar_data.push(distance);
    }
    sonar_data
}

fn sonar_part_one(sonar_data: &Vec<u32>) -> u32 {
    // Sliding window of 1
    let mut increased: u32 = 0;

    for index in 0..sonar_data.len() {
        if index > 0 {
            if sonar_data[index] > sonar_data[index-1]{
                increased += 1;
            }
        }
    }
    increased
}

fn sonar_part_two(sonar_data: &Vec<u32>) -> u32 {
    // Sliding window of 3
    let mut prev_window: u32 = 0;
    let mut cur_window: u32;
    let mut increased: u32 = 0;

    for index in 0..sonar_data.len() {
        if index > 1 {
            cur_window = sonar_data[index] + sonar_data[index-1] + sonar_data[index-2];
            if cur_window > prev_window && prev_window != 0 {
                increased += 1;
            }
            prev_window = cur_window;
        }
    }
    increased
}

fn main() -> io::Result<()> {

    let sonar_data = get_sonar_data();

    let part_one = sonar_part_one(&sonar_data);
    let part_two = sonar_part_two(&sonar_data);

    println!("Sonar Part 1: {}", part_one);
    println!("Sonar Part 2: {}", part_two);

    Ok(())
}