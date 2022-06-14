/*
--- Day 7: The Treachery of Whales ---
*/

use std::fs;

fn parse_input() -> Vec<i32> {
    let file = fs::read_to_string("input").unwrap();
    let mut positions: Vec<i32> = file.trim().split(",").map(|pos| pos.parse::<i32>().unwrap()).collect();
    positions.sort();
    positions
}

fn calculate_fuel_consumption(positions: &Vec<i32>, position: i32, fuel_constant: bool) -> i32 {
    let mut fuel = 0;

    if fuel_constant {
        for pos in positions {
            fuel += (pos - position).abs();
        }
    }
    else {
        for pos in positions {
            let fuel_upper_bound = (pos - position).abs();
            for inc in 1..fuel_upper_bound+1 {
                fuel += inc;
            }
        }
    }

    fuel
}


fn min_fuel_consumption(positions: &Vec<i32>, mut position: i32, fuel_constant: bool) -> i32 {
    let mut fuel = calculate_fuel_consumption(&positions, position, fuel_constant);
    let mut dir = 1;
    
    // Increasing position increases the fuel consumption then invert direction
    if fuel < calculate_fuel_consumption(positions, position + dir, fuel_constant) {
        dir = -1;
    }

    loop {
        position += dir;
        let new_pos_fuel = calculate_fuel_consumption(positions, position, fuel_constant);

        if fuel < new_pos_fuel {
            break;
        }
        
        else {
            fuel = new_pos_fuel;
        }
    }
    
    fuel

}

fn main() {
    // Returns a sorted list of integers
    let positions = parse_input();
    let part_one = min_fuel_consumption(&positions, (positions.len()/2) as i32, true);
    let part_two = min_fuel_consumption(&positions, (positions.len()/2) as i32, false);
    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  example_part_one_test() {
        let example = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(min_fuel_consumption(&example, (example.len()/2) as i32, true), 37);
    }
    
    #[test]
    fn  example_part_two_test() {
        let example = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(min_fuel_consumption(&example, (example.len()/2) as i32, false), 168);
    }
}