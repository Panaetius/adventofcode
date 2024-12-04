use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"mul\((?P<a>[0-9]+),(?P<b>[0-9]+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut sum = 0;
    let mut on = true;
    for cap in re.captures_iter(contents.as_str()) {
        match cap.get(0).unwrap().as_str() {
            "don't()" => {
                on = false;
            }
            "do()" => {
                on = true;
            }
            _ => {
                if on {
                    let a = cap.name("a").unwrap().as_str().parse::<u32>().unwrap();
                    let b = cap.name("b").unwrap().as_str().parse::<u32>().unwrap();
                    sum += a * b;
                }
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
