use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<_> = contents.split("\n").collect();
    let n_rows = lines.len();
    let n_colums = lines[0].trim().chars().count();
    let mut row_cost = vec![1_000_000; n_rows];
    let mut col_cost = vec![1_000_000; n_colums];
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            if c == '#' {
                galaxies.push((row, col));
                row_cost[row] = 1;
                col_cost[col] = 1;
            }
        }
    }
    let res: usize = galaxies
        .iter()
        .combinations(2)
        .map(|v: Vec<&(usize, usize)>| -> usize {
            let mut a = vec![v[0].0, v[1].0];
            let mut b = vec![v[0].1, v[1].1];
            a.sort();
            b.sort();
            row_cost[a[0]..a[1]].iter().sum::<usize>() + col_cost[b[0]..b[1]].iter().sum::<usize>()
        })
        .sum();
    println!("{}", res);

    Ok(())
}
