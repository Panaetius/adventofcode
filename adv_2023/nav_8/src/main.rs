use core::panic;
use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_until1};
use nom::character::complete::{alpha1, alphanumeric1, newline, space1};
use nom::combinator::map_parser;
use nom::error::Error;
use nom::multi::{many1, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Dir {
    L,
    R,
}
impl From<&str> for Dir {
    fn from(item: &str) -> Self {
        match item {
            "L" => Dir::L,
            "R" => Dir::R,
            _ => panic!("unknown dir"),
        }
    }
}

fn dir_map(i: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (i, k) = alphanumeric1(i)?;
    let (i, _) = tag(" = ")(i)?;
    let (i, val) = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(i)?;
    let (i, _) = newline(i)?;
    Ok((i, (k, val)))
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let (i, (directions, _)) =
        many_till(alt((tag("L"), tag("R"))), newline::<_, Error<_>>)(contents.as_str()).unwrap();
    let directions: Vec<Dir> = directions.into_iter().map(|n| n.into()).collect();
    let (i, _) = newline::<_, Error<_>>(i).unwrap();
    let (_, instructions) = many1(dir_map)(i).unwrap();
    let instructions = instructions
        .into_iter()
        .fold(HashMap::new(), |mut map, entry| {
            map.insert(entry.0, entry.1);
            map
        });
    let mut starts: Vec<&&str> = instructions.keys().filter(|k| k.ends_with("A")).collect();
    let mut nums: Vec<usize> = Vec::new();
    for start in starts {
        let mut cur = *start;
        let mut steps = 0;
        let mut dirs = directions.iter().cycle();
        println!("{:?}", cur);
        while !cur.ends_with("Z") {
            steps += 1;
            cur = match dirs.next() {
                Some(Dir::L) => &instructions.get(&cur).unwrap().0,
                Some(Dir::R) => &instructions.get(&cur).unwrap().1,
                _ => unreachable!(),
            };
            if steps % 100000 == 0 {
                println!("{}: {:?}", steps, cur);
            }
        }
        nums.push(steps);
    }
    let steps = nums.iter().fold(1, |acc, val| lcm(acc, *val));
    println!("{}", steps);

    Ok(())
}
