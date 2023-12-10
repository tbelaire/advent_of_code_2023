use regex::bytes::Regex;
use AdventOfCode2023Theo;

struct Number {
    value: u32,
    row: usize,
    col: usize,
    length: usize,
}

impl Number {
    fn adjacent_symbols(&self, grid: &Grid) -> Vec<u8> {
        let mut results: Vec<u8> = Vec::new();
        // row above
        if self.row > 0 {
            let row = grid[self.row - 1];
            let left = if self.col > 0 { self.col - 1 } else { 0 };
            let right = self.col + self.length+1;
            let right = if right < row.len() { right } else { row.len() };
            for col in left..right {
                results.push(row[col]);
            }
        }
        if self.value == 9999 {
            println!("9999 top:{:?}", std::str::from_utf8(&results).unwrap());
        }
        // row
        let row = grid[self.row];

        if self.col > 0 {
            let left = self.col - 1;
            results.push(row[left]);
        }
        let right = self.col + self.length;
        if right < row.len() {
            results.push(row[right]);
        }
        if self.value == 9999 {
            println!("9999 mid:{:?}, right:{}", std::str::from_utf8(&results).unwrap(), right);
        }
        // row below
        if self.row + 1 < grid.len() {
            let row = grid[self.row + 1];
            let left = if self.col > 0 { self.col - 1 } else { 0 };
            let right = self.col + self.length+1;
            let right = if right < row.len() { right } else { row.len() };
            for col in left..right {
                results.push(row[col]);
            }
            if self.value == 9999 {
                println!("row: {:?}, left:{}, right:{}", std::str::from_utf8(row).unwrap(),left, right);
            }
        }
        if self.value == 9999 {
            println!("9999 bot:{:?}", std::str::from_utf8(&results).unwrap());
        }
        results.into_iter().filter(|&x| x != '.' as u8).collect()
    }
}

type Grid<'a> = Vec<&'a [u8]>;

fn main() {
    let input = AdventOfCode2023Theo::load_input("d03.txt");

    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut numbers = Vec::new();

    let r = Regex::new(r"\d+").unwrap();
    for (row_ix, row) in grid.iter().copied().enumerate() {
        for m in r.find_iter(row) {
            let s: &str = std::str::from_utf8(m.as_bytes()).unwrap();
            let value = s.parse().unwrap();
            numbers.push(Number {
                value,
                row: row_ix,
                col: m.start(),
                length: m.len(),
            });
        }
    }

    let mut total = 0;
    for n in numbers {
        if n.value == 9999 {
            println!("9999: {:?}", std::str::from_utf8(&n.adjacent_symbols(&grid)).unwrap());
        }
        if n.adjacent_symbols(&grid).len() > 0 {
            total += n.value;
            // println!("{} is a part number", n.value);
        }
    }
    println!("Total: {}", total)
}
