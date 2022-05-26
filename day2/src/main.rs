/*

-- Day 2 Dive! --

*/

use std::fs;

fn get_sub_commands() -> Vec<String> {
    /*
        Parses input file of submarine commands
        Returns: vector of submarine commands
    */
    
    // BufReader (day1's approach) is better for repeated reads to the same file or network sockets.
    // Below is a better approach for single reads to a file.
    let file = fs::read_to_string("input").expect("Issue with file");
    let sub_cmds: Vec<String> = file.trim().split("\n").map(|line| String::from(line)).collect();
    sub_cmds
}

fn parse_cmd(sub_cmd: &String) -> Vec<String> {
    let cmd: Vec<String> = sub_cmd.split_whitespace().map(|str| String::from(str)).collect();
    cmd
}

fn part_one(sub_cmds: &Vec<String>) {
    // Both horizontal and depth start at 0
    let mut horizontal = 0;
    let mut depth = 0;

        // Adjust position
        for sub_cmd in sub_cmds {
            let cmd = parse_cmd(&sub_cmd);
            if cmd[0] == "forward" {
                let num: i32 = cmd[1].parse().unwrap();
                horizontal += num;
            }
            // UP decreases depth (subtract)
            else if cmd[0] == "up" {
                let num: i32 = cmd[1].parse().unwrap();
                depth -= num;
            }
            // DOWN increases depth (addition)
            else {
                let num: i32 = cmd[1].parse().unwrap();
                depth += num;
            }
        }
    println!("Part 1 position: {}", horizontal*depth);
}

fn part_two(sub_cmds: &Vec<String>) {
    // Both horizontal, depth, and aim start at 0
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for sub_cmd in sub_cmds {
        let cmd = parse_cmd(&sub_cmd);

        if cmd[0] == "forward" {
            let num: i32 = cmd[1].parse().unwrap();
            horizontal += num;
            depth += num*aim;
        }
        // UP decreases aim (subtract)
        else if cmd[0] == "up" {
            let num: i32 = cmd[1].parse().unwrap();
            aim -= num;
        }
        // DOWN increases aim (addition)
        else {
            let num: i32 = cmd[1].parse().unwrap();
            aim += num;
        }
    }
    println!("Part 2 position: {}", horizontal*depth);
}

fn main() {

    let sub_cmds = get_sub_commands();
    part_one(&sub_cmds);
    part_two(&sub_cmds);

    
}
