use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    iter::Sum,
};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "47|53
    // 97|13
    // 97|61
    // 97|47
    // 75|29
    // 61|13
    // 75|53
    // 29|13
    // 97|29
    // 53|29
    // 61|53
    // 97|53
    // 61|29
    // 47|13
    // 75|47
    // 97|75
    // 47|61
    // 75|61
    // 47|29
    // 75|13
    // 53|13

    // 75,47,61,53,29
    // 97,61,53,29,13
    // 75,29,13
    // 75,97,47,61,53
    // 61,13,29
    // 97,13,75,29,47";
    let (rules, pages) = contents.split_once("\n\n").unwrap();
    let rules: HashSet<&str> = rules.split("\n").collect();
    let pages: Vec<Vec<_>> = pages
        .trim()
        .split("\n")
        .map(|p| p.split(",").collect())
        .collect();
    let mut sum = 0;
    'outer: for pl in pages {
        let mut correct = false;
        let mut swapped = false;
        let mut ppl = pl.clone();
        'w: loop {
            println!("{:?}", ppl);
            let mut swapped_now = false;
            'inner: for i in 1..ppl.len() {
                for j in 0..i {
                    if rules.contains(format!("{}|{}", ppl[i], ppl[j]).as_str()) {
                        swapped_now = true;
                        swapped = true;
                        ppl.swap(i, j);
                    }
                }
            }
            if swapped_now {
                continue 'w;
            }
            correct = true;
            break;
        }
        if correct && !swapped {
            continue 'outer;
        }
        let elem = ppl[ppl.len() / 2];
        sum += elem.parse::<u32>().unwrap();
        println!("sum:{}", sum);
    }
    println!("{}", sum);
    Ok(())
}
// fn main() -> std::io::Result<()> {
//     let mut file = File::open("input1")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     //     let contents = "MMMSXXMASM
//     // MSAMXMSMSA
//     // AMXSXMAAMM
//     // MSAMASMSMX
//     // XMASAMXAMM
//     // XXAMMXXAMA
//     // SMSMSASXSS
//     // SAXAMASAAA
//     // MAMMMXMMMM
//     // MXMXAXMASX";
//     let (rules, pages) = contents.split_once("\n\n").unwrap();
//     let rules: HashSet<&str> = rules.split("\n").collect();
//     let pages: Vec<Vec<_>> = pages
//         .trim()
//         .split("\n")
//         .map(|p| p.split(",").collect())
//         .collect();
//     let mut sum = 0;
//     'outer: for pl in pages {
//         for i in 1..pl.len() {
//             for j in 0..i {
//                 if rules.contains(format!("{}|{}", pl[i], pl[j]).as_str()) {
//                     continue 'outer;
//                 }
//             }
//         }
//         let elem = pl[pl.len() / 2];
//         sum += elem.parse::<u32>().unwrap();
//     }
//     println!("{}", sum);
//     Ok(())
// }
