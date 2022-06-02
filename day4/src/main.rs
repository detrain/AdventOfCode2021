use std::fs;

fn parse_input() -> (Vec<u32>, Vec<Vec<Vec<(u32, bool)>>>)  {
    let file: String = fs::read_to_string("input").unwrap();
    let mut input: Vec<&str> = file.trim().split('\n').collect();

    // First index is list of random integers
    let numbers: Vec<u32> = input[0].split(",").map(|num| num.parse::<u32>().unwrap()).collect();
    input.drain(0..1);

    let mut boards: Vec<Vec<Vec<(u32, bool)>>> = Vec::new();
    let mut board: Vec<Vec<(u32, bool)>> = Vec::new();
    let mut row: Vec<(u32, bool)> = Vec::new();

    for b in input {
        // New board
        if b == "" {
            if board.len() > 0 {
                boards.push(board.clone());
                board.clear();
            }
        }
        // Parse each row of the board. Initialize each entry with false (not marked)
        else {
            let row_input: Vec<u32> = b.split_ascii_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
            for val in row_input {
                row.push((val, false));
            }
            // Push the row (Vec<(u32, bool)>) to board
            board.push(row.clone());
            row.clear();
        }
    }
    // Push the last board
    boards.push(board);

    (numbers, boards)
}

fn update_board(number: &u32, boards: &mut Vec<Vec<Vec<(u32, bool)>>>) {
    /* 
        Updates each board if the drawn number is on the respective board
    */
    for board in 0..boards.len() {
        for x in 0..boards[board].len() {
            for y in 0..boards[board][x].len() {
                if boards[board][x][y].0 == *number {
                    boards[board][x][y].1 = true;
                }
            }
        }
    }
}

fn check_winner(boards: &mut Vec<Vec<Vec<(u32, bool)>>>) -> (bool, usize) {
    /*
        Checks for BINGO
    */
    let mut winner: (bool, usize) = (false, 0);
    let mut vertical = 0;
    let mut horizontal = 0;

    for board in 0..boards.len() {
        for x in 0..boards[board].len() {
            for y in 0..boards[board][x].len() {
                if boards[board][x][y].1 {
                    horizontal += 1;
                }
                if boards[board][y][x].1 {
                    vertical += 1;
                }
            }

            if horizontal == 5 || vertical == 5 {
                winner.0 = true;
                winner.1 = board;
                break;
            }

            vertical = 0;
            horizontal = 0;
        }
        if winner.0 {
            break;
        }
    }
    winner
}

fn calc_last_board(boards: &mut Vec<Vec<Vec<(u32, bool)>>>) {
    let mut reset_loop = false;
    let mut cont = true;
    let mut counter = 0;

    while cont {
        for board in 0..boards.len() {
            counter += 1;
            for x in 0..boards[board].len() {
                let mut vertical = 0;
                let mut horizontal = 0;

                for y in 0..boards[board][x].len() {
                    if boards[board][x][y].1 {
                        horizontal += 1;
                    }
                    if boards[board][y][x].1 {
                        vertical += 1;
                    }
                }
                // Current board is a winner remove it...
                if horizontal == 5 || vertical == 5 {
                    // Break when we find our last winner (dont remove it)
                    if boards.len() == 1 {
                        reset_loop = true;
                        cont = false;
                        break;
                    }

                    boards.remove(board);
                    reset_loop = true;
                    // Break from loop now that we modified boards
                    break;
                }
            }
            // Break from loop now that we modified boards
            if reset_loop {
                break;
            }
        }
        reset_loop = false;
        // If we didnt reach the end of the vector we need to iterate through again removing the winners
        if counter >= boards.len()-1 {
            cont = false;
        }
        counter = 0;
    }

}

fn main() {
    let game_params = parse_input();
    let mut boards = game_params.1;
    let numbers = game_params.0;
    let mut winner: ((bool, usize), u32) = ((false, 0), 0);
    let mut sum = 0;
    
    // Part One

    // Go through the drawn numbers
    /*
    for num in numbers {
        winner.1 = num;
        update_board(&num, &mut boards);
        winner.0 = check_winner(&mut boards);
        
        // Part One
        if winner.0.0 {
            break;
        }
    }
    
    for i in 0..boards[winner.0.1].len() {
        for j in 0..boards[winner.0.1][i].len() {
            if boards[winner.0.1][i][j].1 == false {
                sum += boards[winner.0.1][i][j].0;
            }
        }
    }
    
    println!("Final Score {}", sum*winner.1);
    */

    // Part Two
    for num in numbers {
        winner.1 = num;
        update_board(&num, &mut boards);
        calc_last_board(&mut boards);
        if boards.len() == 1 {
            winner.0 = check_winner(&mut boards);
            // Check if the last board is a winner, if so break/dont update anymore of the board
            if winner.0.0 {
                break;
            }
        }

        println!("Boards Length {}, Current Number {}", boards.len(), num);
    }

    for i in 0..boards[0].len() {
        println!("{:?}", boards[0][i]);
    }

    for x in 0..boards[0].len() {
        for y in 0..boards[0][x].len() {
            if boards[0][x][y].1 == false {
                sum += boards[0][x][y].0;
            }
        }
    }

    println!("Final Score {}", sum*winner.1);

}
