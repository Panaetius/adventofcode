use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // let res: u32 = contents
    //     .trim()
    //     .split(",")
    //     .map(|i| {
    //         i.chars()
    //             .map(|c| c as u16)
    //             .fold(0, |acc, v| (acc + v) * 17 % 256) as u32
    //     })
    //     .sum();
    let mut map: HashMap<u32, Vec<(&str, u8)>> = HashMap::new();
    for inst in contents.trim().split(",") {
        if inst.contains("=") {
            let (label, lens) = inst.split_once("=").unwrap();
            let lens: u8 = lens.parse().unwrap();
            let h = hash(label);
            map.entry(h)
                .and_modify(|v| {
                    if let Some(pos) = v.iter().position(|e| e.0 == label) {
                        std::mem::replace(&mut v[pos], (label, lens));
                    } else {
                        v.push((label, lens));
                    }
                })
                .or_insert(vec![(label, lens)]);
        } else if inst.contains("-") {
            let (label, _) = inst.split_once("-").unwrap();
            let h = hash(label);
            map.entry(h).and_modify(|v| v.retain(|&e| e.0 != label));
        }
    }
    let res: usize = map
        .iter()
        .map(|(&k, v)| {
            v.iter()
                .enumerate()
                .map(|(i, (_, l))| {
                    (usize::try_from(k).unwrap() + 1) * (i + 1) * usize::try_from(*l).unwrap()
                })
                .sum::<usize>()
        })
        .sum();
    println!("{:?}", res);
    Ok(())
}
fn hash(input: &str) -> u32 {
    input
        .chars()
        .map(|c| c as u16)
        .fold(0, |acc, v| (acc + v) * 17 % 256) as u32
}
