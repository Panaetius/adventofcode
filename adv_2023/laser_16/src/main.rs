use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn next(self, coords: (usize, usize), max: (usize, usize)) -> Option<Vec<Beam>> {
        match self {
            Direction::Left if coords.0 > 0 => Some(vec![Beam {
                direction: self,
                coords: (coords.0 - 1, coords.1),
            }]),
            Direction::Right if coords.0 < max.0 => Some(vec![Beam {
                direction: self,
                coords: (coords.0 + 1, coords.1),
            }]),
            Direction::Up if coords.1 > 0 => Some(vec![Beam {
                direction: self,
                coords: (coords.0, coords.1 - 1),
            }]),
            Direction::Down if coords.1 < max.1 => Some(vec![Beam {
                direction: self,
                coords: (coords.0, coords.1 + 1),
            }]),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum CellType {
    Mirror(char),
    Splitter(char),
}
impl CellType {
    fn next(&self, beam: &Beam, max: (usize, usize)) -> Option<Vec<Beam>> {
        match self {
            Self::Splitter(c) => match c {
                '-' if beam.direction == Direction::Left || beam.direction == Direction::Right => {
                    beam.direction.next(beam.coords, max)
                }
                '|' if beam.direction == Direction::Up || beam.direction == Direction::Down => {
                    beam.direction.next(beam.coords, max)
                }
                '-' => Some(
                    vec![
                        Direction::Left.next(beam.coords, max),
                        Direction::Right.next(beam.coords, max),
                    ]
                    .into_iter()
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
                    .flatten()
                    .collect(),
                ),
                '|' => Some(
                    vec![
                        Direction::Up.next(beam.coords, max),
                        Direction::Down.next(beam.coords, max),
                    ]
                    .into_iter()
                    .filter(|v| v.is_some())
                    .map(|v| v.unwrap())
                    .flatten()
                    .collect(),
                ),
                _ => unreachable!(),
            },
            Self::Mirror(c) => match c {
                '/' => match beam.direction {
                    Direction::Left => Direction::Down.next(beam.coords, max),
                    Direction::Right => Direction::Up.next(beam.coords, max),
                    Direction::Up => Direction::Right.next(beam.coords, max),
                    Direction::Down => Direction::Left.next(beam.coords, max),
                },
                '\\' => match beam.direction {
                    Direction::Left => Direction::Up.next(beam.coords, max),
                    Direction::Right => Direction::Down.next(beam.coords, max),
                    Direction::Up => Direction::Left.next(beam.coords, max),
                    Direction::Down => Direction::Right.next(beam.coords, max),
                },
                _ => unreachable!(),
            },
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash, Copy)]
struct Beam {
    direction: Direction,
    coords: (usize, usize),
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut cells: HashMap<(usize, usize), CellType> = HashMap::new();
    let mut max = (0, 0);
    for (y, line) in contents.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        for (x, char) in line.trim().chars().enumerate() {
            match char {
                '/' | '\\' => {
                    cells.insert((x, y), CellType::Mirror(char));
                }
                '-' | '|' => {
                    cells.insert((x, y), CellType::Splitter(char));
                }
                _ => {}
            }
            max = (x, y);
        }
    }
    let mut max_act = 0;
    let time = Instant::now();
    for x in 0..max.0 {
        println!("{:.2?}:x {}/{}", time.elapsed(), x, max.0);
        let res = fill_beam(
            Beam {
                coords: (x, 0),
                direction: Direction::Down,
            },
            &cells,
            max,
        );
        if res > max_act {
            max_act = res;
        }
        let res = fill_beam(
            Beam {
                coords: (x, max.1),
                direction: Direction::Up,
            },
            &cells,
            max,
        );
        if res > max_act {
            max_act = res;
        }
    }
    for y in 0..max.1 {
        println!("{:.2?}:y {}/{}", time.elapsed(), y, max.1);
        let res = fill_beam(
            Beam {
                coords: (0, y),
                direction: Direction::Right,
            },
            &cells,
            max,
        );
        if res > max_act {
            max_act = res;
        }
        let res = fill_beam(
            Beam {
                coords: (max.0, y),
                direction: Direction::Down,
            },
            &cells,
            max,
        );
        if res > max_act {
            max_act = res;
        }
    }
    println!("{}", max_act);

    Ok(())
}

fn fill_beam(start: Beam, cells: &HashMap<(usize, usize), CellType>, max: (usize, usize)) -> usize {
    let mut stack = vec![start];
    let mut visited: HashSet<Beam> = HashSet::new();
    visited.insert(start);

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        let next = match cells.get(&current.coords) {
            Some(cell) => cell.next(&current, max),
            None => current.direction.next(current.coords, max),
        };
        if next.is_none() {
            continue;
        }
        for b in next.unwrap() {
            if !visited.contains(&b) {
                visited.insert(b.clone());
                stack.push(b.clone());
            }
        }
    }
    // println!(
    //     "{}",
    //     (0..max.1 + 1)
    //         .map(|y| {
    //             (0..max.0 + 1)
    //                 .map(|x| match visited.iter().any(|b| b.coords == (x, y)) {
    //                     true => "#",
    //                     false => ".",
    //                 })
    //                 .join("")
    //         })
    //         .join("\n")
    // );
    // println!("{:?}", max);
    let res = visited.iter().map(|b| b.coords).unique().count();
    res
}
