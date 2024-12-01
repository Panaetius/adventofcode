use std::fs::File;
use std::io::prelude::*;
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}
impl TryFrom<&str> for Coordinate {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let coords: Vec<usize> = value
            .splitn(3, ",")
            .map(|v| v.parse::<usize>().expect("couldn't parse number"))
            .collect();
        Ok(Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Brick {
    from: Coordinate,
    to: Coordinate,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut bricks: Vec<Brick> = Vec::new();
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let (from, to) = line.split_once("~").unwrap();
        bricks.push(Brick {
            from: from.try_into().unwrap(),
            to: to.try_into().unwrap(),
        });
    }
    println!("{:?}", bricks);

    Ok(())
}
