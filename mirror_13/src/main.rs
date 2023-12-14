use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

// fn mirror_axis(data: Vec<&str>) -> Option<usize> {
//     let map: HashMap<&str, Vec<usize>> = data.iter().filter(|l| !l.is_empty()).enumerate().fold(
//         HashMap::new(),
//         |mut map, (i, line)| {
//             map.entry(line).and_modify(|e| e.push(i)).or_insert(vec![i]);
//             map
//         },
//     );
//     println!("{:?}", map);
//     let boundaries: Vec<HashSet<usize>> = map
//         .values()
//         .map(|v| {
//             v.windows(2)
//                 .filter(|v| v[1] - v[0] > 1)
//                 .map(|d| ((d[0] + d[1]) / 2))
//                 .collect::<HashSet<usize>>()
//         })
//         .collect();
//     println!("{:?}", boundaries);
//     let boundaries = boundaries.iter().filter(|s| !s.is_empty()).fold(
//         None,
//         |acc: Option<HashSet<usize>>, hs| {
//             acc.map(|s| s.intersection(&hs).map(|e| *e).collect())
//                 .or(Some(hs.clone()))
//         },
//     );
//     boundaries.map(|hs| hs.iter().next().map(|e| *e)).flatten()
// }
// fn mirror_axis(data: Vec<&str>) -> Option<usize> {
//     if let Some((i, _)) = data
//         .iter()
//         .skip(1)
//         .rev()
//         .enumerate()
//         .find(|(_, &l)| l == data[0])
//     {
//         Some(i / 2)
//     } else if let Some((i, _)) = data
//         .iter()
//         .rev()
//         .skip(1)
//         .rev()
//         .enumerate()
//         .find(|(_, &l)| l == data[data.len() - 1])
//     {
//         Some((data.len() - i) / 2)
//     } else {
//         None
//     }
// }
fn mirror_axis(data: Vec<&str>) -> Option<usize> {
    let mut found: Option<(usize, usize)> = None;
    for i in 1..data.len() {
        // check each potential position
        let len = min(i, data.len() - i);
        println!("{:?}..{:?}", &data[i - len..i], &data[i..i + len]);
        let rev = data[i..i + len]
            .iter()
            .rev()
            .map(|h| *h)
            .collect::<Vec<&str>>();
        let rev = rev.as_slice();
        if &data[i - len..i] == rev {
            found = match found {
                Some((oldlen, _)) if oldlen > len => found,
                _ => Some((len, i)),
            }
        }
    }
    found.map(|(_, i)| i)
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let blocks = contents.split("\n\n");
    let mut sum = 0;
    for block in blocks {
        let lines: Vec<&str> = block.split("\n").filter(|l| !l.is_empty()).collect();
        let mut axis = mirror_axis(lines.clone());
        if let Some(i) = axis {
            sum += i * 100;
        } else {
            //transpose text
            let l: Vec<String> = (0..lines[0].len())
                .map(|i| {
                    lines
                        .iter()
                        .map(|inner| inner.chars().nth(i).unwrap().clone())
                        .collect::<String>()
                })
                .collect();
            let l = l.iter().map(|s| s.as_str()).collect();
            axis = mirror_axis(l);
            if let Some(i) = axis {
                sum += i;
            }
        }
        println!("{:?}", axis);
    }
    println!("{}", sum);
    Ok(())
}
