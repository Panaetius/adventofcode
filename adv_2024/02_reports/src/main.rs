use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: Vec<Vec<_>> = contents
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" ").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
    let mut safe = 0u32;
    'outer: for report in data {
        if is_safe(report.clone()) {
            safe += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut rep = report.clone();
            rep.remove(i);

            if is_safe(rep) {
                safe += 1;
                continue 'outer;
            }
        }
    }
    println!("{}", safe);
    Ok(())
}

fn is_safe(report: Vec<i32>) -> bool {
    let mut incr = false;
    let mut decr = false;
    let mut is_safe = true;
    for i in 0..(report.len() - 1) {
        if report[i] < report[i + 1] {
            incr = true;
        } else if report[i] > report[i + 1] {
            decr = true;
        }

        if !(incr ^ decr) {
            is_safe = false;
            break;
        }
        let diff = (report[i] - report[i + 1]).abs();
        if diff < 1 || diff > 3 {
            is_safe = false;
            break;
        }
    }
    return is_safe;
}
