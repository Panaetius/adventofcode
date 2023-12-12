use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines: Vec<&str> = contents.split("\n").filter(|l| !l.is_empty()).collect();
    lines.reverse();
    let mut multiplier: HashMap<usize, u32> = HashMap::new();
    let mut sum = 0;
    while let Some(line) = lines.pop() {
        // println!("{}", line);
        let (gamenum, numbers) = line.split_once(": ").unwrap();
        let gamenum = gamenum
            .rsplit_once(" ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        sum += multiplier.get(&gamenum).unwrap_or(&1u32);
        // println!("{}", gamenum);
        let (winning, actual) = numbers.split_once(" | ").unwrap();
        let winning: HashSet<&str> = HashSet::from_iter(
            winning
                .split(" ")
                .map(|n| n.trim())
                .filter(|n| !n.is_empty()),
        );
        let actual: HashSet<&str> = HashSet::from_iter(
            actual
                .split(" ")
                .map(|n| n.trim())
                .filter(|n| !n.is_empty()),
        );
        let total: HashSet<_> = winning.intersection(&actual).collect();
        if total.len() == 0 {
            continue;
        }
        // println!("total {}", total.len());
        let mul = multiplier.get(&gamenum).unwrap_or(&1).clone();
        for i in ((gamenum + 1)..=(gamenum + total.len())).rev() {
            // println!("pushing game{}", i);
            multiplier
                .entry(i)
                .and_modify(|e| *e += mul)
                .or_insert(mul + 1);
        }
    }
    println!("{}", sum);

    Ok(())
}
