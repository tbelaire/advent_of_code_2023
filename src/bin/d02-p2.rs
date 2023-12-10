use AdventOfCode2023Theo;

use peg::{self, str::LineCol};

enum Color {
    Red(u32),
    Blue(u32),
    Green(u32),
}
#[derive(Default)]
struct Sample {
    red: u32,
    blue: u32,
    green: u32,
}

pub struct Game {
    id: u32,
    samples: Vec<Sample>
}

peg::parser!{
    grammar game_parser() for str {
        pub rule games() -> Vec<Game>
         = g:(game() ++ "\n") "\n"? {g}
        rule game() -> Game
         = "Game " n:number() ": " s:(sample() ++ "; ") { Game{id:n, samples:s} }
        rule sample() -> Sample
         = c:(color() ++ ", ") {
            build_sample(c)
         }
        rule color() -> Color
         = (n:number() " red" { Color::Red(n)}) 
         / (n:number() " blue" { Color::Blue(n)}) 
         / (n:number() " green" { Color::Green(n)}) 
        rule number() -> u32
         = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }
    }
}

fn build_sample(colors: Vec<Color>) -> Sample {
    let mut s: Sample = Sample::default();
    for c in colors {
        use Color::*;
        match c {
            Red(r) => {s.red = r;}
            Blue(b) => {s.blue = b;}
            Green(g) => {s.green = g;}
        }
    }
    s
}

fn valid_sample(sample: &Sample) -> bool {
    !(sample.red > 12 || sample.green > 13 || sample.blue > 14)
}

fn dump_error_line(input: &str, line_col: LineCol) -> ! {
    panic!("Error {} in line {}", line_col, input.lines().skip(line_col.line-2).next().unwrap());
}

fn main() {
    let input = AdventOfCode2023Theo::load_input("d02-p1.txt");

    let games = match game_parser::games(&input) {
        Ok(g) => g,
        Err(parse_error) => dump_error_line(&input, parse_error.location),
    };
    let mut total = 0;
    for game in games {
        let r = game.samples.iter().map(|s| s.red).max().unwrap();
        let g = game.samples.iter().map(|s| s.green).max().unwrap();
        let b = game.samples.iter().map(|s| s.blue).max().unwrap();
        total += r*g*b
    }
    println!("Total: {}", total);
}
