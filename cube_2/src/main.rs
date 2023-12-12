use core::panic;
use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (game, reveals) = line.split_once(": ").unwrap();
        let (_, game) = game.split_once(" ").unwrap();
        let gamenum = game.parse::<i32>().unwrap();
        let reveals: Vec<&str> = reveals.split("; ").collect();
        println!("{:?}", reveals);
        let res = reveals
            .iter()
            .flat_map(|r| r.split(", "))
            .map(|r| r.split_once(" ").unwrap())
            .map(|(number, color)| (number.parse::<i32>().unwrap(), color))
            .fold((0, 0, 0), |m, (num, color)| match color {
                "red" => (max(m.0, num), m.1, m.2),
                "green" => (m.0, max(m.1, num), m.2),
                "blue" => (m.0, m.1, max(m.2, num)),
                _ => m,
            });
        sum += res.0 * res.1 * res.2;
    }
    println!("{}", sum);
    Ok(())
}
