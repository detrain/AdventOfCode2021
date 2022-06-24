/*
    --- Day 11: Dumbo Octopus ---
*/

use std::fs;

fn parse_input() -> Vec<Vec<(u8, bool)>> {
    let file = fs::read_to_string("input").unwrap();
    let columns: Vec<String> = file.trim().split("\n").map(String::from).collect();
    let mut grid: Vec<Vec<(u8, bool)>> = Vec::new();
    
    for c in columns {
        let row: Vec<(u8, bool)> = c.chars().map(|e| (e.to_digit(10).unwrap() as u8, false)).into_iter().collect();
        grid.push(row);
    }

    grid

}

#[warn(dead_code)]
fn print_grid(grid: &Vec<Vec<(u8, bool)>>) {
    for row in grid.iter() {
        for octopus in row.iter() {
            print!("[{} {}] ", octopus.0, octopus.1);
        }
        println!();
    }
}

fn take_step(grid: &mut Vec<Vec<(u8, bool)>>) {
    for row in grid.iter_mut() {
        for octopus in row.iter_mut() {
            octopus.0 += 1;
        }
    }
}

fn get_neighbors(y: usize, x: usize, max_y: usize, max_x: usize) -> (usize, usize, usize, usize) {
    // Shadows :)
    let x = x as isize;
    let y = y as isize;

    let mut neighbor_min_x = 0;
    let mut neighbor_max_x = x;
    let mut neighbor_min_y = 0;
    let mut neighbor_max_y = y;    

    if y - 1 >= 0 { neighbor_min_y = y - 1; }
    if y + 1 <= (max_y as isize) { neighbor_max_y = y + 1; }
    if x - 1 >= 0 { neighbor_min_x = x - 1; }
    if x + 1 <= (max_x as isize) { neighbor_max_x = x + 1; }

    (neighbor_min_y as usize, neighbor_max_y as usize, neighbor_min_x as usize, neighbor_max_x as usize)
}

fn flash(grid: &mut Vec<Vec<(u8, bool)>>) -> u32 {
    let mut flashes = 0;
    let mut flashing = true;
    let max_y = grid.len() - 1;
    let max_x = grid.first().unwrap().len() - 1;

    while flashing {
        let mut flash_occured = false;
        for row in 0..=max_y {
            for col in 0..=max_x {
                // Check flash
                if grid[row][col].0 > 9 && !grid[row][col].1 {
                    grid[row][col].1 = true;
                    flash_occured = true;
                    flashes += 1;
                    
                    // Increment neighbors
                    let (min_row, max_row, min_col, max_col) = get_neighbors(row, col, max_y, max_x);
                    
                    for neighor_y in min_row..=max_row {
                        for neighbor_x in min_col..=max_col {
                            grid[neighor_y][neighbor_x].0 += 1;
                        }
                    }
                }
            }
        }
        if !flash_occured {
            flashing = false;
        }
    }

    flashes
}

fn reset_flashed_octopi(grid: &mut Vec<Vec<(u8, bool)>>) {
    for row in grid.iter_mut() {
        for octopus in row.iter_mut() {
            if octopus.1 {
                octopus.0 = 0;
                octopus.1 = false;
            }
        }
    }
}

fn step(grid: &mut Vec<Vec<(u8, bool)>>, steps: u32) -> u32 {
    let mut flashes = 0;
    let mut step = 0;
    let mut sync = false;
    
    while !sync {
        step += 1;
        take_step(grid);
        flashes += flash(grid);
        reset_flashed_octopi(grid);
        sync = get_synchronized_step(grid);
    }

    // println!("Part One: {}", flashes);
    
    step
    
}

fn get_synchronized_step(grid: &Vec<Vec<(u8, bool)>>) -> bool {
    let mut synced = true;
    for row in grid.iter() {
        for octopus in row.iter() {
            if octopus.0 != 0 {
                synced = false;
            }
        }
    }

    synced
} 

fn main() {
    let steps = 100;
    let mut grid = parse_input();
    let step = step(&mut grid, steps);
    println!("Part Two {}", step);
}