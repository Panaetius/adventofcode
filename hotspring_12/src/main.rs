use memoize::memoize;
use std::fs::File;
use std::io::prelude::*;
#[memoize]
fn search(pattern: Vec<char>, pos: usize, blocks: Vec<usize>) -> usize {
    let mut pos = pos;
    while pos < pattern.len() && pattern[pos] == '.' {
        pos += 1;
    }
    if blocks.len() == 0 && pos <= pattern.len() {
        if pos < pattern.len() && pattern[pos..].iter().any(|&c| c == '#') {
            //all blocks used but leftover hashes
            return 0;
        }
        // println!("{}", curr);
        return 1;
    } else if pos + blocks.iter().sum::<usize>() > pattern.len() || pos >= pattern.len() {
        return 0;
    }
    match pattern[pos] {
        '?' => {
            let mut res = 0;
            if (pos + blocks[0] < pattern.len() && pattern[pos + blocks[0]] != '#')
                && pattern[pos..pos + blocks[0]].iter().all(|&c| c != '.')
            {
                res += search(pattern.clone(), pos + blocks[0] + 1, blocks[1..].to_vec());
            } else if blocks.len() == 1
                && pos + blocks[0] == pattern.len()
                && pattern[pos..pos + blocks[0]].iter().all(|&c| c != '.')
            {
                // println!("{}{}", curr, "#".repeat(blocks[0]));
                return 1;
            }
            res + search(pattern, pos + 1, blocks.clone())
        } //recurse on block or not block
        '#' => {
            if (pos + blocks[0] < pattern.len() && pattern[pos + blocks[0]] == '#')
                || pattern[pos..pos + blocks[0]].iter().any(|&c| c == '.')
            {
                0
            } else {
                if blocks.len() == 1 && pos + blocks[0] == pattern.len() {
                    // println!("{}{}", curr, "#".repeat(blocks[0]));
                    return 1;
                }
                search(pattern, pos + blocks[0] + 1, blocks[1..].to_vec())
            }
        } //put whole next block
        _ => unreachable!(),
    }
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
        let (pattern, blocks) = line.trim().split_once(" ").unwrap();
        let blocks: Vec<usize> = blocks.split(",").map(|b| b.parse().unwrap()).collect();
        let pattern: Vec<char> = (0..5)
            .map(|_| pattern.trim())
            .collect::<Vec<&str>>()
            .join("?")
            .chars()
            .collect();
        let blocks = blocks.repeat(5);
        let result = search(pattern, 0, blocks);
        println!("{:?}", result);
        sum += result;
    }
    println!("{}", sum);
    Ok(())
}
