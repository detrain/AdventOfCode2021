/*

--- Day 5: Hydrothermal Venture ---

*/

use std::fs;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Line (Point, Point);

fn parse_input() -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let file: String = fs::read_to_string("input").unwrap();
    let entries: Vec<&str> = file.trim().split("\n").collect();
    
    for entry in entries {
        // Separate x1,y1 and x2,y2
        let coords: Vec<&str> = entry.split("->").map(|tmp| tmp.trim()).collect();
        // x1,y1
        let start: Vec<u32> = coords[0].split(',').map(|tmp| tmp.parse::<u32>().unwrap()).collect();
        // x2,y2
        let end: Vec<u32> = coords[1].split(',').map(|tmp| tmp.parse::<u32>().unwrap()).collect();
        // Add line to vector
        lines.push(Line(Point {x: start[0], y: start[1]}, Point {x: end[0], y: end[1]}));
    }

    lines
}

fn create_grid(lines: &Vec<Line>) -> Vec<Vec<u32>> {

    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;

    for line in lines {
        if line.0.x > max_width {
            max_width = line.0.x;
        }
        if line.1.x > max_width {
            max_width = line.1.x;
        }
        if line.0.y > max_height {
            max_height = line.0.y;
        }
        if line.1.y > max_height {
            max_height = line.1.y;
        }
    }

    // +1 to account for proper indexing
    max_height += 1;
    max_width += 1;

    let grid = vec![vec![0; max_width as usize]; max_height as usize];

    grid
}

fn horizontal_vertical(lines: &Vec<Line>, grid: &mut Vec<Vec<u32>>) {
    
    for line in lines {
        // Check for vertical line (x1 == x2)
        if line.0.x == line.1.x {
            // Create the "y" values that need to get their counts updated
            let y_vals: Vec<u32>;
            if line.0.y > line.1.y {
                y_vals = (line.1.y..(line.0.y+1)).collect();
            }
            else {
                y_vals = (line.0.y..(line.1.y+1)).collect();
            }

            for val in y_vals {
                grid[val as usize][line.0.x as usize] += 1;
            }
        }
        // Check for horizontal line (y1 == y2)
        if line.0.y == line.1.y {
            // Create the "x" values that need to get their counts updated
            let x_vals: Vec<u32>;
            if line.0.x > line.1.x {
                x_vals = (line.1.x..(line.0.x+1)).collect();
            }
            else {
                x_vals = (line.0.x..(line.1.x+1)).collect();
            }

            for val in x_vals {
                grid[line.0.y as usize][val as usize] += 1;
            }
        }
    }
}

fn diagonal_lines(lines: &Vec<Line>, grid: &mut Vec<Vec<u32>>) {
    /* Assumption: Slope of diagonal lines is always one */
    for line in lines {
        // Check for diagonal lines (x1 != x2 and y1 != y2)
        if line.0.x != line.1.x && line.0.y != line.1.y {
            let x_vals: Vec<u32>;
            let y_vals: Vec<u32>;

            // Get y values (reverse the order if y1 > y2)
            if line.0.y > line.1.y {
                y_vals = (line.1.y..(line.0.y+1)).rev().collect();
            }
            else {
                y_vals = (line.0.y..(line.1.y+1)).collect();
            }
            // Get x values (reverse the order if x1 > x2)
            if line.0.x > line.1.x {
                x_vals = (line.1.x..(line.0.x+1)).rev().collect();
            }
            else {
                x_vals = (line.0.x..(line.1.x+1)).collect();
            }

            // x_vals.len() == y_vals.len()
            for i in 0..x_vals.len() {
                grid[y_vals[i] as usize][x_vals[i] as usize] += 1;
            }
        }
    }
}

fn get_overlap(grid: &Vec<Vec<u32>>) -> u32 {
    let mut overlaps = 0;
    for row in grid {
        for col in row {
            if *col > 1 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

fn main() {
    // Get a vector of lines from input file
    let lines: Vec<Line> = parse_input();

    // Create grid
    let mut grid: Vec<Vec<u32>> = create_grid(&lines);
    
    // "Draw" lines on the grid
    horizontal_vertical(&lines, &mut grid);
    let part_one = get_overlap(&grid);

    diagonal_lines(&lines, &mut grid);
    let part_two = get_overlap(&grid);

    println!("Part One {}", part_one);
    println!("Part Two {}", part_two);
    
}