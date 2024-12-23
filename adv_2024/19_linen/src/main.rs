use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "r, wr, b, g, bwu, rb, gb, br

    // brwrr
    // bggr
    // gbbr
    // rrbgbr
    // ubwu
    // bwurrg
    // brgr
    // bbrgwb";
    let (available, patterns) = contents.trim().split_once("\n\n").unwrap();
    let mut available: Vec<&str> = available.split(", ").collect();
    let patterns: Vec<&str> = patterns.split("\n").collect();
    available.sort_by_key(|a| a.len());

    let mut count = 0;
    let totpat = patterns.len();
    let mut cache: HashMap<String, usize> = HashMap::new();

    for (i, pattern) in patterns.iter().enumerate() {
        println!("{}/{}: {}", i, totpat, pattern);
        count += is_possible(pattern.to_string(), &available, &mut cache);
    }
    println!("{}", count);
    Ok(())
}

fn is_possible(
    pattern: String,
    available: &Vec<&str>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(res) = cache.get(&pattern) {
        return *res;
    }
    let mut count = 0;
    for a in available {
        if let Some(rest) = pattern.strip_prefix(a) {
            if rest.is_empty() {
                count += 1;
            } else {
                count += is_possible(rest.to_owned(), available, cache);
            }
        }
    }
    cache.entry(pattern).or_insert(count);
    count
}
