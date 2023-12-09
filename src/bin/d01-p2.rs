use std::os::unix::process::ExitStatusExt;

use AdventOfCode2023Theo;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = AdventOfCode2023Theo::load_input("d01-p1.txt");

    let mut total = 0;
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine|zero").unwrap();
    let dict = HashMap::from([
        ("0", 0), ("zero", 0),
        ("1", 1), ("one", 1),
        ("2", 2), ("two", 2),
        ("3", 3), ("three", 3),
        ("4", 4), ("four", 4),
        ("5", 5), ("five", 5),
        ("6", 6), ("six", 6),
        ("7", 7), ("seven", 7),
        ("8", 8), ("eight", 8),
        ("9", 9), ("nine", 9),
    ]);
    for line in input.lines() {
        let res = parse_line(line, &re, &dict);
        // println!("{}: \t{:?}", line, res);
        total += res;
    }
    println!("Total: {}", total);
}

fn parse_line(line: &str, re: &Regex, dict: &HashMap<&str, u32>) -> u32 {
    
    let left_match = re.find(line).unwrap();
    let left = dict.get(left_match.as_str()).unwrap();
    let mut right_match =  None;
    for (i, c) in line.char_indices().rev() {
        if let Some(right_match_inner) = re.find_at(line, i)  {
            right_match = Some(right_match_inner);
            break;
        }
    }

    let right = dict.get(right_match.unwrap().as_str()).unwrap();

    return left * 10 + right;
}