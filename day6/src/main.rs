use std::fs;

fn parse_input() -> Vec<usize> {
    let file = fs::read_to_string("input").unwrap();
    let input: Vec<usize> = file.trim().split(",").map(|num| num.parse::<usize>().unwrap()).collect();
    input
}

fn reproduce(fishies: &Vec<usize>, mut days: u32) -> u64 {
    // Array indicies represent reproduction days [0-8] with values being the amount of fish in their respective repr day
    let mut fish: [u64; 9] = [0; 9];
    let mut population: u64 = 0;
    
    // Organize the input data
    for f in 0..fishies.len() {
        fish[fishies[f]] += 1;
        population += 1;
    }

    // Calculate population based on number of days
    while days > 0 {
        // Carry over represents the number of fish on day 0 in the cycle
        let mut carry_over = 0;
        for i in 0..fish.len() {
            if i == 0 {
                carry_over = fish[i];
            }
            // Not a respawn day shift...
            else {
                fish[i-1] = fish[i];
            }
        }

        // Handle carry over fish
        fish[6] += carry_over;
        fish[8] = carry_over;

        population += carry_over;
        days -= 1;
    }

    population
    
}

fn main() {

    let current_fish = parse_input();
    // Part One: let reproduction_days = 80;
    // Part Two: 
    let reproduction_days = 256;

    let pop = reproduce(&current_fish, reproduction_days);

    println!("Lantern Fish Population: {}", pop);
}
