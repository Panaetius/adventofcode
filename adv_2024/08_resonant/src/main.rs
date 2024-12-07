use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "............
    // ........0...
    // .....0......
    // .......0....
    // ....0.......
    // ......A.....
    // ............
    // ............
    // ........A...
    // .........A..
    // ............
    // ............";
    let spl: Vec<_> = contents.trim().split("\n").collect();
    let height = spl.len() as i32;
    let width = spl[0].len() as i32;

    let positions: HashMap<char, Vec<(i32, i32)>> = spl
        .iter()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| c != &'.')
                .map(move |(i, c)| (c, (i, j)))
        })
        .fold(HashMap::new(), |mut acc, (c, (i, j))| {
            acc.entry(c)
                .and_modify(|e| {
                    (*e).push((i as i32, j as i32));
                })
                .or_insert(vec![(i as i32, j as i32)]);
            acc
        });
    let tot = positions
        .iter()
        .flat_map(|(_, v)| {
            v.iter().permutations(2).flat_map(|v| {
                let a = v[0];
                let b = v[1];
                let mut res = Vec::new();
                let mut x = 0;
                loop {
                    let ant = ((x + 1) * a.0 - x * b.0, (x + 1) * a.1 - x * b.1);
                    if ant.0 < 0 || ant.0 >= width || ant.1 < 0 || ant.1 >= height {
                        break;
                    }
                    res.push(ant);
                    x += 1;
                }
                let mut x = 0;
                loop {
                    let ant = ((x + 1) * b.0 - x * a.0, (x + 1) * b.1 - x * a.1);
                    if ant.0 < 0 || ant.0 >= width || ant.1 < 0 || ant.1 >= height {
                        break;
                    }
                    res.push(ant);
                    x += 1;
                }
                res
            })
        })
        .filter(|(i, j)| i >= &0 && j >= &0 && i < &width && j < &height)
        .unique()
        .count();
    println!("tot:{}", tot);
    Ok(())
}
