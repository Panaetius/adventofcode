use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;

fn get_adj(
    pos: &(usize, usize),
    label: &char,
    symbols: &mut [[char; 141]; 141],
) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    match label {
        '|' => {
            symbols[pos.1][pos.0] = '│';
            res.push((pos.0, pos.1 - 1));
            res.push((pos.0, pos.1 + 1));
        }
        '-' => {
            symbols[pos.1][pos.0] = '─';
            res.push((pos.0 - 1, pos.1));
            res.push((pos.0 + 1, pos.1));
        }
        'L' => {
            symbols[pos.1][pos.0] = '└';
            res.push((pos.0, pos.1 - 1));
            res.push((pos.0 + 1, pos.1));
        }
        'J' => {
            symbols[pos.1][pos.0] = '┘';
            res.push((pos.0, pos.1 - 1));
            res.push((pos.0 - 1, pos.1));
        }
        '7' => {
            symbols[pos.1][pos.0] = '┐';
            res.push((pos.0 - 1, pos.1));
            res.push((pos.0, pos.1 + 1));
        }
        'F' => {
            symbols[pos.1][pos.0] = '┌';
            res.push((pos.0, pos.1 + 1));
            res.push((pos.0 + 1, pos.1));
        }
        '.' => {}
        'S' => {
            symbols[pos.1][pos.0] = 'S';
        }
        _ => unreachable!(),
    }
    res
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<_> = contents.split("\n").collect();
    let n_rows = lines.len();
    let n_columns = lines[0].chars().count();
    let mut start: Option<(usize, usize)> = None;
    let mut adj_list: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut symbols = [[' '; 141]; 141];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let cur_pos = (x + 1, y + 1);
            let adj = get_adj(&cur_pos, &c, &mut symbols);
            adj_list
                .entry(cur_pos)
                .and_modify(|e| e.extend(adj.clone()))
                .or_insert(HashSet::from_iter(adj.clone()));

            if c == 'S' {
                start = Some(cur_pos);
            }
        }
    }
    let start = start.unwrap();
    //link from start
    adj_list
        .clone()
        .iter()
        .filter(|(k, v)| v.contains(&start))
        .map(|(k, v)| k)
        .for_each(|k| {
            adj_list.entry(start).and_modify(|e| {
                e.insert(*k);
            });
        });
    // println!("{:?}", adj_list);
    println!("{:?}", start);
    println!("{:?}", adj_list.get(&start));
    let mut pos = HashSet::from([*adj_list.get(&start).unwrap().iter().next().unwrap()]);
    println!("pos {:?}", pos);
    let mut prev = HashSet::from([start]);
    let mut count = 1;
    let mut loop_tiles: Vec<(usize, usize)> = Vec::new();
    while true {
        count += 1;
        let new_prev = pos.clone();
        let new_pos: HashSet<_> = pos
            .clone()
            .iter()
            .flat_map(|p| adj_list.get(&p).unwrap())
            .map(|e| *e)
            .collect();
        pos = new_pos.difference(&prev).map(|e| *e).collect();
        loop_tiles.extend(prev);
        prev = new_prev;
        if pos.len() == 1 && *pos.iter().next().unwrap() == start
        /*|| count > 100 */
        {
            loop_tiles.extend(prev);
            break;
        }
    }
    println!("{}", count);
    // loop_tiles = loop_tiles.into_iter().rev().collect();
    println!("{:?}", loop_tiles);
    let first = loop_tiles.iter().next().unwrap();
    let last = loop_tiles.iter().last().unwrap();
    let mut inside_visited: HashSet<(usize, usize)> = loop_tiles
        .windows(2)
        .flat_map(|v| {
            vec![
                (v[1].0 + v[0].1 - v[1].1, v[1].1 + v[1].0 - v[0].0),
                (v[0].0 + v[0].1 - v[1].1, v[0].1 + v[1].0 - v[0].0),
            ]
        })
        .filter(|n| !loop_tiles.contains(n))
        .collect();
    let mut current = inside_visited.clone();
    while true {
        println!("inside{:?},{:?}", inside_visited, inside_visited.len());
        let new_inside: HashSet<(usize, usize)> = current
            .iter()
            .flat_map(|n| {
                vec![
                    (n.0, n.1 + 1),
                    (n.0, n.1 - 1),
                    (n.0 + 1, n.1),
                    (n.0 - 1, n.1),
                ]
            })
            .filter(|n| !inside_visited.contains(n) && !loop_tiles.contains(n))
            .collect();
        if new_inside.len() == 0 {
            break;
        }
        inside_visited.extend(new_inside.clone());
        current = inside_visited.clone();
    }
    println!("{:?}", inside_visited.len());
    let mut grid = [[' '; 141]; 141];
    for n in loop_tiles {
        grid[n.1][n.0] = symbols[n.1][n.0];
    }
    for n in inside_visited {
        grid[n.1][n.0] = 'X';
    }

    for l in grid {
        println!("{}", l.iter().collect::<String>());
    }

    Ok(())
}
