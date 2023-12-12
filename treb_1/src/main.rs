use std::fs::File;
use std::io::prelude::*;

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
        println!("{}", line);
        let mut newline = String::new();
        let linelen = line.len();
        let mut i = 0;
        while i < linelen {
            let s = &line[i..];
            if s.starts_with("one") {
                newline += "1";
                i += 1;
            } else if s.starts_with("two") {
                newline += "2";
                i += 1;
            } else if s.starts_with("three") {
                newline += "3";
                i += 1;
            } else if s.starts_with("four") {
                newline += "4";
                i += 1;
            } else if s.starts_with("five") {
                newline += "5";
                i += 1;
            } else if s.starts_with("six") {
                newline += "6";
                i += 1;
            } else if s.starts_with("seven") {
                newline += "7";
                i += 1;
            } else if s.starts_with("eight") {
                newline += "8";
                i += 1;
            } else if s.starts_with("nine") {
                newline += "9";
                i += 1;
            } else {
                newline.push(s.chars().next().expect("expect char"));
                i += 1;
            }
        }
        println!("{}", newline);
        let numbers: Vec<char> = newline.chars().filter(|c| c.is_digit(10)).collect();
        let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap());
        println!("{}", number);
        println!("---");
        sum += number.parse::<i32>().expect("expected to parse number");
    }
    println!("{}", sum);
    Ok(())
}
