use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

// fn main() -> std::io::Result<()> {
//     let mut file = File::open("input1")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     let (mut left, mut right): (Vec<_>, Vec<_>) = contents
//         .split("\n")
//         .filter(|l| !l.is_empty())
//         .map(|l| l.split_once("   ").unwrap())
//         .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
//         .unzip();

//     left.sort();
//     right.sort();
//     let len = left.len();
//     let mut sum = 0;
//     for i in 0..len {
//         sum += (left[i] - right[i]).abs();
//     }
//     println!("Result: {}", sum);
//     Ok(())
// }
fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (mut left, right): (Vec<_>, Vec<_>) = contents
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .unzip();

    left.sort();
    let mut dict: HashMap<i32, i32> = HashMap::new();
    for n in right.iter() {
        dict.entry(*n).and_modify(|v| *v += 1).or_insert(1);
    }
    let result: i32 = left.iter().map(|l| l * dict.get(l).unwrap_or(&0)).sum();
    println!("Result: {}", result);
    Ok(())
}
