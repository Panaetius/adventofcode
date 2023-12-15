use memoize::memoize;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    //transpose
    let lines: Vec<String> = (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|inner| inner.chars().nth(i).unwrap().clone())
                .collect::<String>()
        })
        .collect();
    let mut tilted = lines;
    let mut tt = Duration::new(0, 0);
    let mut rt = Duration::new(0, 0);
    let time = Instant::now();
    let mut seen: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    let mut entries: Vec<Vec<String>> = Vec::new();

    for i in 0..1_000_000_000 {
        entries.push(tilted.clone());
        if let Some(next) = seen.get(&tilted) {
            let index = entries.iter().position(|e| e == next).unwrap();
            let fin = (1_000_000_000 % (i - index + 1)) + index;
            tilted = entries[fin].clone();
            break;
        }
        let previous = tilted.clone();
        for _ in 0..4 {
            // let now = Instant::now();
            tilted = tilt(tilted);
            // tt += now.elapsed();
            // let now = Instant::now();
            tilted = rotate(tilted);
            // rt += now.elapsed();
        }
        seen.insert(previous, tilted.clone());
        if i % 10_000 == 0 {
            println!("{:.2?}", time.elapsed());
            println!("{}", i);
        }
    }
    // println!("{:?}", lines);
    println!("{:?}", tilted);
    let res: usize = tilted
        .iter()
        .map(|line| {
            line.chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| c == &'O')
                .map(|(i, _)| i + 1)
                .sum::<usize>()
        })
        .sum();
    println!("{:?}", res);

    Ok(())
}

#[memoize]
fn rotate(lines: Vec<String>) -> Vec<String> {
    let lines: Vec<String> = (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|inner| inner.chars().rev().nth(i).unwrap().clone())
                .collect::<String>()
        })
        .collect();
    lines
}

#[memoize]
fn tilt(lines: Vec<String>) -> Vec<String> {
    let tilted: Vec<_> = lines
        .iter()
        .map(|l| {
            l.split("#")
                .map(|l| {
                    // let mut chunk = l.chars().collect::<Vec<char>>();
                    // chunk.sort_by(|a, b| b.cmp(a));
                    // let chunk = chunk.into_iter().collect::<String>();
                    // chunk
                    let count = l.chars().filter(|c| c == &'O').count();
                    let mut first = "O".repeat(count);
                    first.push_str(&".".repeat(l.chars().count() - count));
                    first
                })
                .collect::<Vec<String>>()
                .join("#")
        })
        .collect();
    tilted
}
