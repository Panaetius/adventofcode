use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
fn calc_series(data: Vec<i64>) -> i64 {
    if data.iter().all(|&v| v == 0) {
        return 0;
    }
    let new_data: Vec<i64> = data.iter().tuple_windows().map(|(a, b)| b - a).collect();
    let result = calc_series(new_data);
    data[0] - result
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    let mut sum = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let nums: Vec<i64> = line.split(" ").map(|v| v.parse().unwrap()).collect();
        sum += calc_series(nums);
    }
    println!("{}", sum);
    Ok(())
}
