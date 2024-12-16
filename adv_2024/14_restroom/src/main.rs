use std::{collections::HashMap, fs::File, io::Read, iter::Sum, ops::Rem};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64,
}
impl Robot {
    fn steps(&self, steps: usize, width: usize, height: usize) -> Robot {
        Robot {
            x: (self.x + self.v_x * steps as i64).rem_euclid(width as i64),
            y: (self.y + self.v_y * steps as i64).rem_euclid(height as i64),
            v_x: self.v_x,
            v_y: self.v_y,
        }
    }
    fn quadrant(&self, width: usize, height: usize) -> Option<usize> {
        println!("{:?}", self);
        if self.x == width as i64 / 2 || self.y == height as i64 / 2 {
            None
        } else {
            Some((2 * self.x / width as i64 + 2 * (2 * self.y / height as i64)) as usize)
        }
    }
}
fn parse_robot(i: &str) -> IResult<&str, Robot> {
    let (i, ((x, y), (vx, vy))) = separated_pair(
        preceded(tag("p="), separated_pair(digit1, tag(","), digit1)),
        tag(" "),
        preceded(
            tag("v="),
            separated_pair(
                map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
                    s.parse::<i64>()
                }),
                tag(","),
                map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
                    s.parse::<i64>()
                }),
            ),
        ),
    )(i)?;
    let r = Robot {
        x: x.parse::<i64>().unwrap(),
        y: y.parse::<i64>().unwrap(),
        v_x: vx,
        v_y: vy,
    };
    Ok((i, r))
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3
    // "
    //     .to_string();
    let (_, r) = many1(terminated(parse_robot, tag("\n")))(&contents).unwrap();
    let (width, height) = (101, 103);
    // let (width, height) = (11, 7);
    println!(
        "{:?}",
        Robot {
            x: 2,
            y: 4,
            v_x: 2,
            v_y: -3
        }
        .steps(0, 11, 7)
    );
    let mut quadrants = vec![0, 0, 0, 0];
    println!("{:?}", r);
    for steps in 0.. {
        println!("-------------\nsteps: {}", steps);
        let mut field = vec![vec!["."; width]; height];

        for robot in &r {
            // if let Some(quadrant) = robot.steps(steps, width, height).quadrant(width, height) {
            //     println!("{}", quadrant);
            //     quadrants[quadrant] += 1;
            // }
            let rob = robot.steps(steps, width, height);
            field[rob.y as usize][rob.x as usize] = "X";
        }
        let straight = field
            .iter()
            .map(|l| {
                l.iter().fold((0, 0), |(m, a), x| {
                    if x == &"X" {
                        (m, a + 1)
                    } else if a > m {
                        (a, 0)
                    } else {
                        (m, 0)
                    }
                })
            })
            .fold(0, |a, x| if x.0 > a { x.0 } else { a });
        if straight > 8 {
            println!(
                "{}",
                field
                    .iter()
                    .map(|v| v.join(""))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            break;
        }
    }
    println!("{:?}: {}", quadrants, quadrants.iter().product::<i32>());

    Ok(())
}
