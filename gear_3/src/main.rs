use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
#[derive(Debug)]
struct Part {
    value: u64,
    line: i32,
    from: i32,
    to: i32,
}
#[derive(Debug)]
struct Symbol<'a> {
    line: i32,
    col: i32,
    value: &'a str,
}
impl Part {
    fn is_adjacent(&self, s: &Symbol) -> bool {
        if s.line < self.line - 1 || s.line > self.line + 1 {
            return false;
        }
        if s.col < self.from - 1 || s.col > self.to {
            return false;
        }
        return true;
    }
}
impl<'a> Symbol<'a> {
    fn is_gear(&self, parts: &Vec<Part>) -> u64 {
        if self.value != "*" {
            return 0;
        }
        let adjacent_parts: Vec<u64> = parts
            .iter()
            .filter(|p| p.is_adjacent(self))
            .map(|p| p.value)
            .collect();
        if adjacent_parts.len() == 2 {
            return adjacent_parts[0] * adjacent_parts[1];
        }
        return 0;
    }
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n");

    let mut parts: Vec<Part> = vec![];
    let mut symbols: Vec<Symbol> = vec![];
    let re = Regex::new(r"(?<part>[0-9]+)|(?<symbol>[^0-9\.])").unwrap();
    for (i, line) in lines.enumerate() {
        if line.is_empty() {
            continue;
        }
        for cap in re.captures_iter(line) {
            if let Some(val) = cap.name("part") {
                parts.push(Part {
                    line: i as i32,
                    value: val.as_str().parse::<u64>().unwrap(),
                    from: val.start() as i32,
                    to: val.end() as i32,
                })
            } else if let Some(val) = cap.name("symbol") {
                symbols.push(Symbol {
                    line: i as i32,
                    col: val.start() as i32,
                    value: val.as_str(),
                })
            }
        }
    }
    println!("{:?}", parts);
    println!("{:?}", symbols);
    // let result: u64 = parts
    //     .iter()
    //     .filter(|&p| symbols.iter().any(|s| p.is_adjacent(s)))
    //     .map(|p| p.value)
    //     .sum();
    let result: u64 = symbols.iter().map(|s| s.is_gear(&parts)).sum();
    println!("{}", result);
    Ok(())
}
