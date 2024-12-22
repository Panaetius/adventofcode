use std::{collections::HashMap, fs::File, io::Read, iter::Sum, process};
#[derive(Debug)]
struct Computer {
    A: usize,
    B: usize,
    C: usize,
    pointer: usize,
    instructions: Vec<u8>,
    output: Vec<usize>,
}
impl Computer {
    fn compute(&mut self) -> bool {
        if self.pointer >= self.instructions.len() {
            return false;
        }
        // println!("{:?}", self);
        let instruction = self.instructions[self.pointer];
        let operand = self.instructions[self.pointer + 1];
        match instruction {
            0 => {
                self.A /= 2_usize.pow(self.op_value(operand) as u32);
                self.pointer += 2;
            }
            1 => {
                self.B ^= operand as usize;
                self.pointer += 2;
            }
            2 => {
                self.B = self.op_value(operand) % 8;
                self.pointer += 2;
            }
            3 => {
                if self.A != 0 {
                    self.pointer = operand as usize;
                } else {
                    self.pointer += 2;
                }
            }
            4 => {
                self.B ^= self.C;
                self.pointer += 2;
            }
            5 => {
                self.output.push(self.op_value(operand) % 8);
                self.pointer += 2;
            }
            6 => {
                self.B = self.A / 2_usize.pow(self.op_value(operand) as u32);
                self.pointer += 2;
            }
            7 => {
                self.C = self.A / 2_usize.pow(self.op_value(operand) as u32);
                self.pointer += 2;
            }
            _ => unreachable!(),
        }
        true
    }
    fn op_value(&self, operand: u8) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => unreachable!(),
        }
    }
}

// fn main() -> std::io::Result<()> {
//     let mut file = File::open("input")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     //     let contents = "Register A: 729
//     // Register B: 0
//     // Register C: 0

//     // Program: 0,1,5,4,3,0";
//     let lines: Vec<_> = contents.trim().split("\n").collect();
//     let A = lines[0]
//         .split(": ")
//         .nth(1)
//         .unwrap()
//         .parse::<usize>()
//         .unwrap();
//     let B = lines[1]
//         .split(": ")
//         .nth(1)
//         .unwrap()
//         .parse::<usize>()
//         .unwrap();
//     let C = lines[2]
//         .split(": ")
//         .nth(1)
//         .unwrap()
//         .parse::<usize>()
//         .unwrap();

//     let instructions: Vec<u8> = lines[4]
//         .split(": ")
//         .nth(1)
//         .unwrap()
//         .split(",")
//         .map(|v| v.parse::<u8>().unwrap())
//         .collect::<Vec<_>>();
//     // println!("{:?}\n-----------------------", computer);
//     let mut count = 0;
//     for i in 0.. {
//         let mut computer = Computer {
//             A: i,
//             B,
//             C,
//             pointer: 0,
//             instructions: instructions.clone(),
//             output: Vec::new(),
//         };
//         while computer.compute() {}

//         if computer
//             .output
//             .iter()
//             .zip(instructions.clone())
//             .filter(|&(a, b)| *a == b as usize)
//             .count()
//             == instructions.len()
//         {
//             count = i;
//             break;
//         }
//     }
//     println!("{}", count);
//     // println!(
//     //     "{:?}",
//     //     computer
//     //         .output
//     //         .iter()
//     //         .map(usize::to_string)
//     //         .collect::<Vec<_>>()
//     //         .join(",")
//     // );
//     Ok(())
// }
fn main() -> std::io::Result<()> {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for bb in 0..10 {
        for aa in 0..7 {}
    }

    let program = vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0];
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    // for bb in 0..8 {
    //     b = bb;
    //     b = b ^ 1;
    //     // c=a/2_usize.pow(b as usize);
    //     b = b ^ 5;
    //     // b=b^c;
    //     // a=a/2_usize.pow(b as usize);
    //     println!("b:{}", b);
    // }
    // 'outer: for cc in 0..8 {
    //     if let Some(aa) = recurse(a, b, cc, &program, program.len() - 1) {
    //         println!("{}", aa);
    //     }
    // }
    let aa = 0;
    let bb = 0;
    let pos = program.len() - 1;
    // for a in 8 * aa..(8 * (aa + 1)) {
    //     let mut b = a % 8;
    //     let bbb = b ^ 1;
    //     b = bbb ^ 5;
    //     let c = b ^ bb; //bb=b before, aa=a before
    //     let cc = a / 2_usize.pow(bbb as u32);
    //     if cc == c {
    //         //this combination works! valid c
    //         // recurse with c
    //         println!("start: {}:{}:{}", a, bb, c);
    //         if let Some(aaa) = rec(a, c, &program, pos - 1) {
    //             println!("{}", aaa);
    //             break;
    //         }
    //     }
    // }
    println!("{:?}", rec(4, &program, pos - 1));

    Ok(())
}

// fn rec(aa: usize, ccc: usize, program: &Vec<usize>, pos: usize) -> Option<usize> {
//     let bb = program[pos];
//     println!("{}:{}:{}", aa, ccc, pos);

//     for a in 8 * aa..(8 * (aa + 1)) {
//         let mut b = a % 8;
//         let bbb = b ^ 1;
//         b = bbb ^ 5;
//         let c = b ^ bb; //bb=b before, aa=a before
//         let cc = a / 2_usize.pow(bbb as u32);
//         if c == cc {
//             //this combination works! valid c
//             // recurse with c
//             if pos == 0 {
//                 return Some(a);
//             }
//             rec(a, c, &program, pos - 1);
//         }
//     }
//     None
// }

fn rec(aa: usize, program: &Vec<usize>, pos: usize) -> Option<usize> {
    for a in (8 * aa)..(8 * aa + 8) {
        let mut b = a % 8;
        b ^= 1;
        let c = a / 2_usize.pow(b as u32);
        b ^= 5;
        b = b ^ c;
        if b % 8 == program[pos] {
            println!("{}:{}:{}", a, b, c);
            if pos == 0 {
                return Some(a);
            }
            if let Some(aaa) = rec(a, &program, pos - 1) {
                return Some(aaa);
            }
        }
    }
    None
}

// fn recurse(aa: usize, bb: usize, cc: usize, program: &Vec<usize>, pos: usize) -> Option<usize> {
//     if bb != program[pos] {
//         println!("b:{}, pos:{}, inst:{}", bb, pos, program[pos]);
//         return None;
//     }
//     if pos == 0 {
//         return Some(aa);
//     }
//     let mut a = aa;
//     let mut b = bb;
//     let mut c = cc;
//     for a in 8 * aa..(8 * (aa + 1)) {
//         a = a * 2_usize.pow(3_u32) + i;
//         b = b ^ c;
//         b = b ^ 5;
//         // a = c * 2_usize.pow(b as u32);
//         b = b ^ 1;
//         a = b;
//         if let Some(aaa) = recurse(a, b, c, &program, pos - 1) {
//             return Some(aaa);
//         }
//     }
//     None
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut computer = Computer {
            A: 0,
            B: 0,
            C: 9,
            pointer: 0,
            instructions: vec![2, 6],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(computer.B, 1);
    }

    #[test]
    fn test2() {
        let mut computer = Computer {
            A: 10,
            B: 0,
            C: 0,
            pointer: 0,
            instructions: vec![5, 0, 5, 1, 5, 4],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(computer.output, vec![0, 1, 2]);
    }
    #[test]
    fn test3() {
        let mut computer = Computer {
            A: 2024,
            B: 0,
            C: 0,
            pointer: 0,
            instructions: vec![0, 1, 5, 4, 3, 0],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.A, 0);
    }
    #[test]
    fn test4() {
        let mut computer = Computer {
            A: 0,
            B: 29,
            C: 0,
            pointer: 0,
            instructions: vec![1, 7],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(29 ^ 7, 26);
        assert_eq!(computer.B, 26);
    }
    #[test]
    fn test5() {
        let mut computer = Computer {
            A: 0,
            B: 2024,
            C: 43690,
            pointer: 0,
            instructions: vec![4, 0],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(computer.B, 44354);
    }
    #[test]
    fn test6() {
        let mut computer = Computer {
            A: 4,
            B: 0,
            C: 0,
            pointer: 0,
            instructions: vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(computer.output, vec![0]);
    }
    #[test]
    fn test7() {
        let mut computer = Computer {
            A: 37,
            B: 3,
            C: 2,
            pointer: 0,
            instructions: vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(computer.output, vec![3, 0]);
    }
    #[test]
    fn test8() {
        let mut computer = Computer {
            A: 39,
            B: 3,
            C: 0,
            pointer: 0,
            instructions: vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0],
            output: Vec::new(),
        };
        while computer.compute() {}
        assert_eq!(computer.output, vec![3, 0]);
    }
}
