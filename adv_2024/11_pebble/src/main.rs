use memoize::memoize;
use std::{collections::HashMap, fs::File, io::Read, iter::Sum, ops::Div};

// fn main() -> std::io::Result<()> {
//     let mut file = File::open("input1")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     // let contents = "125 17";
//     let state: Vec<u64> = contents
//         .trim()
//         .split(" ")
//         .map(|n| n.parse::<u64>().unwrap())
//         .collect();
//     let mut res = Vec::new();
//     let depth = 75;
//     for (i, n) in state.iter().enumerate() {
//         println!("{}", i);
//         res.extend(recurse(*n, depth))
//     }
//     println!("{}", res.len());
//     Ok(())
// }

// fn recurse(n: u64, count: usize) -> Vec<u64> {
//     if count == 0 {
//         vec![n]
//     } else {
//         match split(n) {
//             (a, Some(b)) => {
//                 let mut res = Vec::new();
//                 res.extend(recurse(a, count - 1));
//                 res.extend(recurse(b, count - 1));
//                 res
//             }
//             (a, None) => recurse(a, count - 1),
//         }
//     }
// }

// fn split(n: u64) -> (u64, Option<u64>) {
//     if n == 0 {
//         (1, None)
//     } else if n.ilog10() % 2 == 1 {
//         let l = (n.ilog10() + 1).div_ceil(2);
//         let f = 10u64.pow(l);

//         (n / f, Some(n - f * (n / f)))
//     } else {
//         (n * 2024, None)
//     }
// }
fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // let contents = "125 17";
    let state: Vec<u64> = contents
        .trim()
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let mut stone_cache: HashMap<u64, usize> = HashMap::new();
    for stone in state {
        *stone_cache.entry(stone).or_insert(0) += 1;
    }
    let depth = 75;
    for _ in 0..depth {
        stone_cache = process(stone_cache);
    }

    println!("{}", stone_cache.values().sum::<usize>());
    Ok(())
}

fn process(cache: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut outp: HashMap<u64, usize> = HashMap::new();
    for (k, v) in cache.into_iter() {
        if k == 0 {
            *outp.entry(1).or_insert(0) += v;
        } else if k.ilog10() % 2 == 1 {
            let l = (k.ilog10() + 1).div_ceil(2);
            let f = 10u64.pow(l);

            *outp.entry(k / f).or_insert(0) += v;
            *outp.entry(k - f * (k / f)).or_insert(0) += v;
        } else {
            *outp.entry(k * 2024).or_insert(0) += v;
        }
    }
    outp
}
