/*
--- Day 8: Seven Segment Search ---
*/

use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input() -> Vec<(Vec<String>, Vec<String>)> {
    /*
        Returns Vector of tuples (unique_patterns, output)
    */

    let mut segments: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    let file = fs::read_to_string("input").unwrap();
    let lines: Vec<String> = file.trim().split("\n").map(String::from).collect();

    for line in lines {
        let collection: Vec<String> = line.split("|").map(String::from).collect();
        let unique_patterns: Vec<String> = collection[0].trim().split_whitespace().map(String::from).collect();
        let output: Vec<String> = collection[1].trim().split_whitespace().map(String::from).collect();

        segments.push((unique_patterns, output));
    }

    segments
    
}

fn generate_decode_key(unique_displays: &Vec<String>) -> HashMap<String, String> {
    // Hashset for each unique patterns
    let mut sets: Vec<HashSet<String>> = Vec::new();
    let mut key: HashMap<String, String> = HashMap::new();

    for pattern in unique_displays {
        let mut chars:Vec<String> = pattern.split("").map(String::from).collect();
        chars.pop();
        chars.remove(0);
        sets.push(HashSet::from_iter(chars));
    }

    // Indicies for the following numbers
    let mut one = 0;
    let mut two = 0;
    let mut four = 0;
    let mut five = 0;
    let mut seven = 0;

    for i in 0..sets.len() {
        if sets[i].len() == 2 {
            one = i;
        }
        if sets[i].len() == 4 {
            four = i;
        }
        if sets[i].len() == 3 {
            seven = i;
        }
    }

    // Segment a mapping
    let segment_a = sets[seven].difference(&sets[one]).into_iter().collect::<Vec<&String>>()[0];
    key.insert(segment_a.clone(), String::from("a"));

    // Segment c and f mapping
    let mut segment_f = &String::from("f");
    let mut segment_c = &String::from("c");
    let mut segment_c_set: HashSet<String> = HashSet::new();

    for i in 0..sets.len() {
        // Get display sets from {2, 3, 5}
        if sets[i].len() == 5 {
            // display-2 intersection with display-4 results in length of 2
            let intersection_four = sets[i].intersection(&sets[four]).into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
            if intersection_four.len() == 2 {
                two = i;
                
                // c
                segment_c = sets[one].intersection(&intersection_four).into_iter().collect::<Vec<&String>>()[0];
                key.insert(segment_c.clone(), String::from("c"));

                // f
                segment_c_set = HashSet::from([segment_c.clone()]);
                segment_f = sets[one].difference(&segment_c_set).into_iter().collect::<Vec<&String>>()[0];
                key.insert(segment_f.clone(), String::from("f"));
            }
            else {
                // display-1 intersection with length 5 will determing index of 3 (len 2 intersection with 1) and 5 (length 1 intersection with one)
                let intersection_one = sets[i].intersection(&sets[one]).into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
                if intersection_one.len() == 1 {
                    five = i;
                }
            }
        }
    }

    // Segment d mapping
    let segment_bd: HashSet<String> = sets[four].difference(&sets[seven]).into_iter().map(|s| s.to_owned()).collect();
    let segment_d = sets[two].intersection(&segment_bd).into_iter().collect::<Vec<&String>>()[0];
    key.insert(segment_d.clone(), String::from("d"));

    // Segment b mapping
    let segment_bf: HashSet<String> = sets[five].difference(&sets[two]).into_iter().map(|s| s.to_owned()).collect();
    let segment_f_set = HashSet::from([segment_f.clone()]);
    let segment_b = segment_bf.difference(&segment_f_set).into_iter().collect::<Vec<&String>>()[0];
    key.insert(segment_b.clone(), String::from("b"));

    // Segment e mapping
    let segments_ec: HashSet<String> = sets[two].difference(&sets[five]).into_iter().map(|s| s.to_owned()).collect();
    let segment_e = segments_ec.difference(&sets[one]).into_iter().collect::<Vec<&String>>()[0];
    key.insert(segment_e.clone(), String::from("e"));

    // Segment g mapping
    let segments_ag: HashSet<String> = sets[five].difference(&sets[four]).into_iter().map(|s| s.to_owned()).collect();
    let segment_g = segments_ag.difference(&sets[seven]).into_iter().collect::<Vec<&String>>()[0];
    key.insert(segment_g.clone(), String::from("g"));

    key

}

fn decode(key: HashMap<String, String>, display: &Vec<String>) -> String {
    // Vector of hashsets that form the correct number display based off correct wiring
    let seven_segment_display: Vec<HashSet<&str>> = vec![
        HashSet::from(["a", "b", "c", "e", "f", "g"]),          // 0
        HashSet::from(["c", "f"]),                              // 1
        HashSet::from(["a", "c", "d", "e", "g"]),               // 2
        HashSet::from(["a", "c", "d", "f", "g"]),               // 3
        HashSet::from(["b", "c", "d", "f"]),                    // 4
        HashSet::from(["a", "b", "d", "f", "g"]),               // 5
        HashSet::from(["a", "b", "d", "e", "f", "g"]),          // 6
        HashSet::from(["a", "c", "f"]),                         // 7
        HashSet::from(["a", "b", "c", "d", "e", "f", "g"]),     // 8
        HashSet::from(["a", "b", "c", "d", "f", "g"]),          // 9
    ];

    let mut number: String = "".to_owned();
    
    for num in display {
        let mut chars:Vec<String> = num.split("").map(String::from).collect();
        chars.pop();
        chars.remove(0);
        let numset: HashSet<&String> = chars.iter().collect();
        let mut decoded_numset:  HashSet<&str> = HashSet::new();
        
        for k in &numset {
            decoded_numset.insert(&key[*k][..]);
        }

        for i in 0..seven_segment_display.len() {
            if decoded_numset.symmetric_difference(&seven_segment_display[i]).into_iter().map(|s| s.to_owned()).collect::<HashSet<&str>>().len() == 0 {
                number.push_str(i.to_string().as_str());
            }
        }
    }

    number

}

fn part_one(input: &String) -> u32 {
    let mut count = 0;
    let mut displays:Vec<String> = input.split("").map(String::from).collect();
    displays.pop();
    displays.remove(0);
    let numbers: Vec<u32> = displays.iter().map(|i| i.parse::<u32>().unwrap()).collect();
    
    for num in numbers {
        if num == 1 || num == 4 || num == 7 || num == 8 {
            count += 1;
        }
    }

    count
}

fn part_two(input: &String) -> u32 {
    let num = input.parse::<u32>().unwrap();
    num
}

fn main() {
    let segments = parse_input();
    let mut pt_one_ctr = 0;
    let mut pt_two_ctr = 0;
    // Get the key for each entry. Decode its output.
    for segment in segments {
        let key = generate_decode_key(&segment.0);
        let output = decode(key, &segment.1);
        pt_one_ctr += part_one(&output);
        pt_two_ctr += part_two(&output);
    }
    
    println!("Part One: {}", pt_one_ctr);
    println!("Part Two: {}", pt_two_ctr);

}
