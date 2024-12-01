use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    let mut layout: Vec<Vec<bool>> = Vec::new();
    for (y, line) in contents.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        layout.push(Vec::new());
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '.' => {
                    layout[y].push(true);
                }
                '#' => {
                    layout[y].push(false);
                }
                'S' => {
                    layout[y].push(true);
                    positions.insert((x, y));
                }
                _ => unreachable!(),
            }
        }
    }
    // println!("{:?}", layout);
    // let max = (layout[0].len() - 1, layout.len() - 1);
    // for _ in 0..200 {
    //     let mut next: HashSet<(usize, usize)> = HashSet::new();
    //     for start in positions.iter() {
    //         //get next
    //         if start.0 > 0 && layout[start.1][start.0 - 1] {
    //             next.insert((start.0 - 1, start.1));
    //         }
    //         if start.1 > 0 && layout[start.1 - 1][start.0] {
    //             next.insert((start.0, start.1 - 1));
    //         }
    //         if start.0 < max.0 && layout[start.1][start.0 + 1] {
    //             next.insert((start.0 + 1, start.1));
    //         }
    //         if start.1 < max.1 && layout[start.1 + 1][start.0] {
    //             next.insert((start.0, start.1 + 1));
    //         }
    //     }
    //     positions = next;
    //     println!("{}", positions.len());
    // }
    // println!("{:?}", max);
    // println!("{}", positions.len());
    let mut sum: isize = 0;
    let start = Instant::now();
    for y in -202300isize..=202300 {
        let cur = 202300 - y.abs();
        for x in -cur..=cur {
            if x.abs() + y.abs() % 2 == 0 {
                sum += 7808;
            } else {
                sum += 7759;
            }
        }
        if y % 1000 == 0 {
            println!("{:.2?}:{}", start.elapsed(), y);
        }
    }
    println!("{}", sum);

    Ok(())
}
