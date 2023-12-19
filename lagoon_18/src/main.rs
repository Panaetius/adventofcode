use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alphanumeric1, digit1, newline, space1};
use nom::multi::{count, many1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
impl Direction {
    fn orthogonal(&self) -> Vec<Direction> {
        match self {
            Direction::Left | Direction::Right => vec![Direction::Down, Direction::Up],
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
        }
    }
    fn add(&self, coords: &(isize, isize), len: usize) -> (isize, isize) {
        let len = TryInto::<isize>::try_into(len).unwrap();
        match self {
            Direction::Left => (coords.0 - len, coords.1),
            Direction::Right => (coords.0 + len, coords.1),
            Direction::Up => (coords.0, coords.1 - len),
            Direction::Down => (coords.0, coords.1 + len),
        }
    }
    fn subtract(
        &self,
        coords: &(usize, usize),
        len: usize,
        max: &(usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Right if coords.0 > len => Some((coords.0 - len, coords.1)),
            Direction::Left if len <= max.0 && coords.0 <= max.0 - len => {
                Some((coords.0 + len, coords.1))
            }
            Direction::Down if coords.1 > len => Some((coords.0, coords.1 - len)),
            Direction::Up if len <= max.1 && coords.1 <= max.1 - len => {
                Some((coords.0, coords.1 + len))
            }
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Line<'a> {
    start: (isize, isize),
    end: (isize, isize),
    dir: Direction,
    len: usize,
    color: &'a str,
}

impl<'a> Line<'a> {
    fn intersect_horizontal(&self, point: (isize, isize)) -> usize {
        let mut m = false;
        if point.0 == self.start.0 &&point.0==self.end.0 //vertical
            && (self.start.1 - point.1).abs() + (point.1 - self.end.1).abs()
                == (self.start.1 - self.end.1).abs()
        {
            m = true;
        } else if point.1 == self.start.1 && point.1 == self.start.1 // horizontal
            && (self.start.0 - point.0).abs() + (point.0 - self.end.0).abs()
                == (self.start.0 - self.end.0).abs()
        {
            m = true;
        }
        if !m {
            return 0;
        }
        if self.start.1 == self.end.1 {
            //horizontal line
            return 1;
        }
        if (point == self.start && self.dir == Direction::Down)
            || (point == self.end && self.dir == Direction::Up)
        {
            return 2;
        } else if (point == self.start || point == self.end) {
            return 1;
        }
        return 2;
    }
}

fn dig_plan(i: &str) -> IResult<&str, Line> {
    let (i, dir) = terminated(alt((tag("R"), tag("D"), tag("L"), tag("U"))), space1)(i)?;
    // let dir = match dir {
    //     "R" => Direction::Right,
    //     "D" => Direction::Down,
    //     "L" => Direction::Left,
    //     "U" => Direction::Up,
    //     _ => unreachable!(),
    // };
    let (i, len) = terminated(digit1, space1)(i)?;
    let (i, (len, dir)) = terminated(
        delimited(
            tag("("),
            preceded(tag("#"), pair(take(5usize), alphanumeric1)),
            tag(")"),
        ),
        newline,
    )(i)?;
    let dir = match dir {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => unreachable!(),
    };

    Ok((
        i,
        Line {
            start: (0, 0),
            end: (0, 0),
            dir,
            len: usize::from_str_radix(len, 16).unwrap(),
            color: "",
        },
    ))
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (i, nodes) = many1(dig_plan)(&contents).unwrap();
    let lines = nodes
        .iter()
        .fold(Vec::new(), |mut v: Vec<Line<'_>>, &line| {
            if v.len() == 0 {
                v.push(Line {
                    start: line.start,
                    end: line.dir.add(&line.start, line.len),
                    dir: line.dir,
                    len: line.len,
                    color: line.color,
                });
            } else {
                let last = v.last().unwrap();
                v.push(Line {
                    start: last.end,
                    end: line.dir.add(&last.end, line.len),
                    dir: line.dir,
                    len: line.len,
                    color: line.color,
                });
            }
            v
        });
    let extent = lines.iter().fold(
        ((isize::MAX, isize::MIN), (isize::MAX, isize::MIN)),
        |((m1, m2), (m3, m4)), n| {
            (
                (min(n.start.0, m1), max(n.start.0, m2)),
                (min(n.start.1, m3), max(n.start.1, m4)),
            )
        },
    );
    let mut count: usize = 0; // lines.iter().map(|n| n.len).sum();
    println!("{:?}", extent);

    // for y in extent.1 .0..=extent.1 .1 {
    //     let mut inside = false;
    //     for x in extent.0 .0..=extent.0 .1 {
    //         match lines
    //             .iter()
    //             .map(|l| l.intersect_horizontal((x, y)))
    //             .max()
    //             .unwrap()
    //         {
    //             0 if inside => {
    //                 count += 1;
    //             }
    //             1 => {
    //                 count += 1;
    //             }
    //             2 => {
    //                 count += 1;
    //                 inside = !inside;
    //             }
    //             _ => {}
    //         }
    //     }
    //     println!("{}", count);
    // }
    let vertices: Vec<(isize, isize)> = lines.iter().map(|l| l.start).collect();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in 0..vertices.len() - 1 {
        sum1 += vertices[i].0 * vertices[i + 1].1;
        sum2 += vertices[i].1 * vertices[i + 1].0;
    }
    sum1 += vertices[vertices.len() - 1].0 * vertices[0].1;
    sum2 += vertices[0].0 * vertices[vertices.len() - 1].1;
    let count: isize =
        (sum1 - sum2).abs() / 2 + lines.iter().map(|l| l.len as isize).sum::<isize>() / 2 + 1;

    println!("{:?}", lines.iter().map(|l| l.len as isize).sum::<isize>());
    println!("{:?}", lines);
    println!("{:?}", count);

    Ok(())
}
