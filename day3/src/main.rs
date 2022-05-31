/*

-- Day 3 Binary Diagnostic --

*/

use std::fs;

fn parse_data() -> Vec<String> {
    let file: String = fs::read_to_string("input").unwrap();
    let binary_data: Vec<String> = file.trim().split("\n").map(|line| String::from(line)).collect();
    binary_data
}

fn part_one(binary_data: &Vec<String>) {
    let mut most_common = Vec::new();

    // Assume width of each line is same
    for x in 0..binary_data[0].chars().count() {
        let mut bit0 = 0;
        let mut bit1 = 0;

        // Iterate through each column
        for y in 0..binary_data.len() {
            if binary_data[y].chars().nth(x).unwrap() == '0' {
                bit0 += 1;
            }
            else {
                bit1 += 1;
            }
        }

        if bit0 > bit1 {
            most_common.push("0");
        }

        else {
            most_common.push("1");
        }
    }

    let mut epsilon_rate = String::new();
    let mut gamma_rate = String::new();

    // Invert gammma to get epsilon
    for bin in most_common {
        let i = bin.chars().next().unwrap();
        gamma_rate.push(i);
        if i == '1' {
            epsilon_rate.push('0');
        }
        else {
            epsilon_rate.push('1');
        }
    }

    let gamma_rate = u32::from_str_radix(&gamma_rate[..], 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate[..], 2).unwrap();

    println!("Gamma: {}\nEpsilon: {}", gamma_rate, epsilon_rate);

    println!("Fuel Consumption: {}", gamma_rate*epsilon_rate);
}

fn part_two(binary_data: &Vec<String>) {
    
    let mut bit0;
    let mut bit1;
    let mut oxygen = binary_data.clone();
    let mut co2 = binary_data.clone();
    let mut common;
    let length = binary_data[0].len();

    for i in 0..length {
        // Reset couters for most common
        bit0 = 0;
        bit1 = 0;
        // Oxygen rating (most common)
        for y in 0..oxygen.len() {
            if oxygen[y].chars().nth(i).unwrap() == '0' {
                bit0 += 1;
            }
            else {
                bit1 += 1;
            }
        }
        if bit0 <= bit1 {
            common = '1';
        }
        else {
            common = '0';
        }
        //println!("Most Common Bit {}: {}", i, common);
        // Remove all indicies that do not have the most common bit in position i
        if oxygen.len() > 1 {
            oxygen.retain(|line| line.chars().nth(i).unwrap() != common);
        }
        
        // Reset couters for least common
        bit0 = 0;
        bit1 = 0;
        // CO2 rating (least common)
        for y in 0..co2.len() {
            if co2[y].chars().nth(i).unwrap() == '0' {
                bit0 += 1;
            }
            else {
                bit1 += 1;
            }
        }
        if bit0 <= bit1 {
            common = '0';
        }
        else {
            common = '1';
        }
        //println!("Least Common Bit {}: {}", i, common);
        // Remove all indicies that do not have the least common bit in position i
        if co2.len() > 1 {
            co2.retain(|line| line.chars().nth(i).unwrap() != common);
        }
    }
    let oxygen_rating = isize::from_str_radix(&oxygen[0][..], 2).unwrap();
    let co2_rating = isize::from_str_radix(&co2[0][..], 2).unwrap();
    println!("Life Support Rating: {:?}", oxygen_rating*co2_rating);
}

fn main() {
    let binary_data = parse_data();
    part_one(&binary_data);
    part_two(&binary_data);
}
