use std::{collections::HashMap, fmt::Binary, fs::File, io::Read, iter::Sum, process::Output};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Block {
    Empty(usize),
    ID(u32, usize),
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // let contents = "2333133121414131402";
    let u = unroll(contents.trim());
    let d = defrag(u);
    println!("{}", d);

    Ok(())
}
fn unroll(inp: &str) -> Vec<Block> {
    let mut free = false;
    let mut id = 0;
    let mut result = Vec::new();
    for c in inp.chars() {
        let num = c.to_digit(10).unwrap();
        if free {
            result.push(Block::Empty(num as usize));
        } else {
            result.push(Block::ID(id, num as usize));
            id += 1;
        }
        free = !free;
    }
    result
}

fn defrag(inp: Vec<Block>) -> usize {
    let mut point2 = inp.len() - 1;
    let mut point1 = 0;
    let mut outp = inp.clone();

    while point2 > 0 {
        point1 = 0;
        'outer: while point1 < point2 {
            match outp[point2] {
                Block::Empty(_) => {
                    point2 -= 1;
                    continue;
                }
                Block::ID(_, l) => {
                    while point1 < point2 {
                        match outp[point1] {
                            Block::ID(_, _) => {
                                point1 += 1;
                                continue;
                            }
                            Block::Empty(fl) => {
                                if fl < l {
                                    point1 += 1;
                                    continue;
                                }
                                if fl == l {
                                    outp[point1] = outp[point2];
                                    outp[point2] = Block::Empty(l);
                                    break 'outer;
                                } else {
                                    outp[point1] = outp[point2];
                                    outp.insert(point1 + 1, Block::Empty(fl - l));
                                    point2 += 1;
                                    outp[point2] = Block::Empty(l);
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
        point2 -= 1;
    }
    println!("{:?}", outp);
    println!("{}", point1);
    println!("{}", outp.len());
    outp.iter()
        .fold((0, 0), |(i, s), b| match b {
            Block::ID(a, l) => (
                i + l,
                s + (i..i + l).map(|ii| ii * (*a as usize)).sum::<usize>(),
            ),
            Block::Empty(l) => (i + l, s),
        })
        .1
}
// #[derive(PartialEq, Eq, Clone, Copy, Debug)]
// enum Block {
//     Empty,
//     ID(u32),
// }
// fn unroll(inp: &str) -> Vec<Block> {
//     let mut free = false;
//     let mut id = 0;
//     let mut result = Vec::new();
//     for c in inp.chars() {
//         let num = c.to_digit(10).unwrap();
//         if free {
//             for _ in 0..num {
//                 result.push(Block::Empty);
//             }
//         } else {
//             for _ in 0..num {
//                 result.push(Block::ID(id));
//             }
//             id += 1;
//         }
//         free = !free;
//     }
//     result
// }

// fn defrag(inp: Vec<Block>) -> usize {
//     let mut point1 = 0;
//     let mut point2 = inp.len() - 1;
//     let mut outp = inp.clone();

//     while point1 < point2 {
//         if outp[point1] != Block::Empty {
//             point1 += 1;
//         } else if outp[point2] == Block::Empty {
//             point2 -= 1;
//         } else {
//             outp[point1] = outp[point2];
//             outp[point2] = Block::Empty;
//             point1 += 1;
//             point2 -= 1;
//         }
//     }
//     while outp[point1] != Block::Empty {
//         point1 += 1;
//     }

//     println!("{:?}", outp);
//     println!("{}", point1);
//     println!("{}", outp.len());
//     outp.iter()
//         .enumerate()
//         .map(|(i, b)| match b {
//             Block::ID(a) => i * *a as usize,
//             Block::Empty => 0,
//         })
//         .sum()
// }
