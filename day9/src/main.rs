/*
--- Day 9: Smoke Basin ---
*/

use std::fs;
use std::collections::HashSet;

fn parse_input() -> Vec<Vec<u8>> {
    let file: Vec<String> = fs::read_to_string("input").unwrap().trim().split("\n").map(|s| s.to_owned()).collect();
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for entry in file {
        let array: Vec<u8> = entry.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        grid.push(array);
    }

    grid

}

fn print_grid(grid: &Vec<Vec<u8>>) {
    for y in grid {
        for x in y {
            print!("{}", x);
        }
        println!("");
    }
}

fn get_depressions(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;
    let mut depressions:Vec<(usize, usize)> = Vec::new();

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            let point = grid[y][x];
            let mut is_depression = false;
            // Top Row
            if y == 0 {
                // Top Left [0,0]
                if x == 0 {
                    if point < grid[y][x+1] && point < grid[y+1][x] {
                        is_depression = true;
                    }
                }
                // Top Right [0,max_x]
                else if x == max_x {
                    if point < grid[y][x-1] && point < grid[y+1][x] {
                        is_depression = true;
                    }
                }
                // Rest of top row [0, (0,max_x)]
                else {
                    if point < grid[y][x+1] && point < grid[y][x-1] && point < grid[y+1][x] {
                        is_depression = true;
                    }
                }
            }
            // Bot Row
            else if y == max_y {
                // Bot Left
                if x == 0 {
                    if point < grid[y][x+1] && point < grid[y-1][x] {
                        is_depression = true;
                    }
                }
                // Bot Right
                else if x == max_x {
                    if point < grid[y][x-1] && point < grid[y-1][x] {
                        is_depression = true;
                    }
                }
                // Rest of bot row [max_y, (0,max_x)]
                else {
                    if point < grid[y-1][x] && point < grid[y][x+1] && point < grid[y][x-1] {
                        is_depression = true;
                    }
                }
            }
            // Left Column
            else if x == 0 {
                // Not Top Left or Not Bot Left (we already accounted for them in Top Row and Bot Row)
                if point < grid[y-1][x] && point < grid[y+1][x] && point < grid[y][x+1] {
                    is_depression = true;
                }
            }
            // Right Column
            else if x == max_x {
                if point < grid[y-1][x] && point < grid[y+1][x] && point < grid[y][x-1] {
                    is_depression = true;
                }
            }
            // Remaining points
            else {
                if point < grid[y+1][x] && point < grid[y-1][x] && point < grid[y][x+1] && point < grid[y][x-1] {
                    is_depression = true;
                }
            }

            if is_depression {
                depressions.push((y,x));
            }
        }
    }
    depressions
}

fn calculate_risk(grid: &Vec<Vec<u8>>, depressions: &Vec<(usize, usize)>) -> u32 {
    let mut risk: u32 = 0;

    for depression in depressions {
        risk += (grid[depression.0][depression.1] + 1) as u32;
    }

    risk
}

fn calculate_basins(grid: &Vec<Vec<u8>>, depressions: &Vec<(usize, usize)>) -> u32 {
    let mut basins: Vec<u32> = Vec::new();
    let max_y = grid.len() - 1;
    let max_x = grid[0].len() - 1;

    // Iterative DFS for each depression
    for depression in depressions {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: Vec<(usize, usize)> = Vec::new();
        // Push start point to stack, mark as visited
        stack.push(*depression);
        visited.insert(*depression);

        while stack.len() > 0 {
            let vertex = stack.pop().unwrap();
            
            // Check Up
            if vertex.0 < max_y {
                let up = (vertex.0 + 1, vertex.1);
                if !visited.contains(&up) {
                    if grid[vertex.0][vertex.1] < grid[up.0][up.1] && grid[up.0][up.1] != 9 {
                        stack.push((up.0, up.1));
                        visited.insert(up);
                    }
                }
            }

            // Check Down
            if vertex.0 > 0 {
                let down = (vertex.0 - 1, vertex.1);
                if !visited.contains(&down) {
                    if grid[vertex.0][vertex.1] < grid[down.0][down.1] && grid[down.0][down.1] != 9 {
                        stack.push((down.0, down.1));
                        visited.insert(down);
                    }
                }
            }

            // Check Left
            if vertex.1 > 0 {
                let left = (vertex.0, vertex.1 - 1);
                if !visited.contains(&left) {
                    if grid[vertex.0][vertex.1] < grid[left.0][left.1] && grid[left.0][left.1] != 9 {
                        stack.push((left.0, left.1));
                        visited.insert(left);
                    }
                }
            }

            // Check Right
            if vertex.1 < max_x {
                let right = (vertex.0, vertex.1 + 1);
                if !visited.contains(&right) {
                    if grid[vertex.0][vertex.1] < grid[right.0][right.1] && grid[right.0][right.1] != 9 {
                        stack.push((right.0, right.1));
                        visited.insert(right);
                    }
                }
            }
        }
        basins.push(visited.len() as u32);
    }
    let mut larget_basins = 0;

    basins.sort();
    basins.reverse();

    if basins.len() > 3 {
        larget_basins = basins[0] * basins[1] * basins[2];
    }

    larget_basins
}

fn main() {
    let grid = parse_input();
    //print_grid(&grid);
    let depressions = get_depressions(&grid);
    let risk = calculate_risk(&grid, &depressions);
    println!("Part One: {}", risk);
    let larget_basins = calculate_basins(&grid, &depressions);
    println!("Part Two: {}", larget_basins);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_depressions() {
        let grid = vec![ vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                         vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                         vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                         vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                         vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]];
        
        let calculated_depressions = HashSet::from([get_depressions(&grid)]);
        let actual_depressions = vec![(0, 1), (0, 9), (2, 2), (4, 6)];
        let actual_result = HashSet::from([actual_depressions]);

        assert_eq!(calculated_depressions, actual_result);
    }
    
    #[test] 
    fn example_risk() {
        let grid = vec![ vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                         vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                         vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                         vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                         vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]];

        let actual_depressions = vec![(0, 1), (0, 9), (2, 2), (4, 6)];
        let actual_risk = 15;
        let calculated_risk = calculate_risk(grid, actual_depressions);

        assert_eq!(actual_risk, calculated_risk);
    }
}
