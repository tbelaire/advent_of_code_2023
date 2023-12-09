use AdventOfCode2023Theo;

fn main() {
    let input = AdventOfCode2023Theo::load_input("d01-p1.txt");

    let mut total = 0;
    for line in input.lines() {
        let res = parse_line(line);
        // println!("{}: \t{:?}", line, res);
        total += res;
    }
    println!("Total: {}", total);
}

fn parse_line(line: &str) -> u32 {
    let mut left = None;
    let mut right = None;
    for (_i,c) in line.char_indices() {
        if c.is_ascii_digit() {
            left = c.to_digit(10);
            break;
        }
    }
    for (_i,c) in line.char_indices().rev() {
        if c.is_ascii_digit() {
            right = c.to_digit(10);
            break;
        }
    }
    left.unwrap() * 10 + right.unwrap()
}