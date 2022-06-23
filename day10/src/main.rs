use std::fs;
use std::collections::HashMap;

fn parse_input() -> Vec<String> {
    let file = fs::read_to_string("input").unwrap();
    let lines: Vec<String> = file.trim().split("\n").map(String::from).into_iter().collect();
    lines
}

fn find_corrupt_chunks(lines: &Vec<String>) -> (Vec<char>, Vec<usize>) {
    let mut syntax_errors: Vec<char> = Vec::new();
    let mut corrupt_lines: Vec<usize> = Vec::new();
    let brackets: HashMap<char, char> = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    
    for line in 0..lines.len() {

        let mut syntax_stack: Vec<char> = Vec::new();

        for element in lines[line].chars() {

            let keys: Vec<char> = brackets.keys().map(|c| c.to_owned()).collect();

            if keys.contains(&element) {
                let last_bracket = syntax_stack.pop().unwrap();

                if last_bracket != brackets[&element] {
                    syntax_errors.push(element);
                    corrupt_lines.push(line);
                    break;
                }
            } else {
                syntax_stack.push(element);
            }
        }
    }
    (syntax_errors, corrupt_lines)
}

fn calculate_points(syntax_errors: Vec<char>) -> u32 {
    let values: HashMap<char, u32> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut score = 0;

    for error in syntax_errors {
        score += values[&error];
    }

    score
}

fn remove_corrupt_chunks(lines: &mut Vec<String>, corrupt_chunks: Vec<usize>) {
    for itr in corrupt_chunks.iter().rev() {
        lines.remove(*itr);
    }
}

fn fix_chunks(lines: &mut Vec<String>) -> Vec<Vec<char>> {
    let closing_brackets: HashMap<char, char> = HashMap::from([(')', '('), (']', '['),('}', '{'), ('>', '<')]);
    let opening_brackets: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'),('{', '}'), ('<', '>')]);
    let mut brackets_added: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        let mut added: Vec<char> = Vec::new();

        for element in line.chars() {
            let keys: Vec<char> = closing_brackets.keys().map(|c| c.to_owned()).collect();
            if keys.contains(&element) {
                stack.pop().unwrap();
            } else {
                stack.push(element);
            }
        }

        for bracket in stack.iter().rev() {
            line.push(opening_brackets[&bracket]);
            added.push(opening_brackets[&bracket]);
        }

        brackets_added.push(added);
    }

    brackets_added
}

fn calculate_autocompletion_score(bracket_list: Vec<Vec<char>>) -> u64 {
    let values: HashMap<char, u64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores: Vec<u64> = Vec::new();

    for brackets in bracket_list {
        let mut score = 0;
        for bracket in brackets {
            score *= 5;
            score += values[&bracket];
        }
        scores.push(score);
    }

    scores.sort();

    let index = scores.len()/2;

    scores[index]
}

fn main() {
    let mut lines = parse_input();
    let syntax_errors = find_corrupt_chunks(&lines);
    let score = calculate_points(syntax_errors.0);
    remove_corrupt_chunks(&mut lines, syntax_errors.1);

    println!("Part One: {}", score);

    let brackets_added = fix_chunks(&mut lines);
    let score = calculate_autocompletion_score(brackets_added);

    println!("Part Two: {}", score);
}