use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    iter::Sum,
    ops::{Div, DivAssign},
};

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    p_x: i64,
    p_y: i64,
}

fn machine(i: &str) -> IResult<&str, Machine> {
    let (i, ((_, a_x, _, a_y, _), (_, b_x, _, b_y, _), (_, p_x, _, p_y, _))) = tuple((
        tuple((tag("Button A: X+"), digit1, tag(", Y+"), digit1, tag("\n"))),
        tuple((tag("Button B: X+"), digit1, tag(", Y+"), digit1, tag("\n"))),
        tuple((tag("Prize: X="), digit1, tag(", Y="), digit1, tag("\n"))),
    ))(i)?;
    Ok((
        i,
        Machine {
            a_x: a_x.parse::<i64>().unwrap(),
            a_y: a_y.parse::<i64>().unwrap(),
            b_x: b_x.parse::<i64>().unwrap(),
            b_y: b_y.parse::<i64>().unwrap(),
            p_x: p_x.parse::<i64>().unwrap() + 10000000000000,
            p_y: p_y.parse::<i64>().unwrap() + 10000000000000,
        },
    ))
}

// fn main() -> std::io::Result<()> {
//     let mut file = File::open("input")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     //     let contents = "Button A: X+94, Y+34
//     // Button B: X+22, Y+67
//     // Prize: X=8400, Y=5400

//     // Button A: X+26, Y+66
//     // Button B: X+67, Y+21
//     // Prize: X=12748, Y=12176

//     // Button A: X+17, Y+86
//     // Button B: X+84, Y+37
//     // Prize: X=7870, Y=6450

//     // Button A: X+69, Y+23
//     // Button B: X+27, Y+71
//     // Prize: X=18641, Y=10279";
//     let (_, machines) = many1(terminated(machine, tag("\n")))(&contents).unwrap();

//     let mut sum = 0;
//     for machine in machines {
//         let mut min = 999999999;
//         let mut solution = (0, 0);
//         for a in 0.. {
//             if machine.a_x * a > machine.p_x || machine.a_y * a > machine.p_y {
//                 break;
//             }
//             let rest_x = machine.p_x - machine.a_x * a;
//             let rest_x = (rest_x / machine.b_x, rest_x % machine.b_x);
//             if rest_x.1 != 0 {
//                 continue;
//             }
//             let rest_y = machine.p_y - machine.a_y * a;
//             let rest_y = (rest_y / machine.b_y, rest_y % machine.b_y);
//             if rest_y.1 != 0 || rest_x.0 != rest_y.0 {
//                 continue;
//             }
//             let cost = 3 * a + rest_x.0;
//             if cost < min {
//                 min = cost;
//                 solution = (a, rest_x.0);
//             }
//         }
//         if solution != (0, 0) {
//             println!("{:?}: {:?} - {}", machine, solution, min);
//             sum += min;
//         }
//     }
//     println!("{}", sum);

//     Ok(())
// }
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176

    // Button A: X+17, Y+86
    // Button B: X+84, Y+37
    // Prize: X=7870, Y=6450

    // Button A: X+69, Y+23
    // Button B: X+27, Y+71
    // Prize: X=18641, Y=10279

    // ";
    let (_, machines) = many1(terminated(machine, tag("\n")))(&contents).unwrap();

    let mut sum = 0.0;
    for machine in machines {
        let b = (machine.p_y * machine.a_x - machine.p_x * machine.a_y) as f64
            / (machine.b_y * machine.a_x - machine.b_x * machine.a_y) as f64;
        let a = (machine.p_x as f64 - b * machine.b_x as f64) / machine.a_x as f64;
        if a >= 0.0 && b >= 0.0 && a.round() == a && b.round() == b {
            sum += 3.0 * a + b;
        }
    }
    println!("{}", sum);

    Ok(())
}
