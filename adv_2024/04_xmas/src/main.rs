use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "MMMSXXMASM
    // MSAMXMSMSA
    // AMXSXMAAMM
    // MSAMASMSMX
    // XMASAMXAMM
    // XXAMMXXAMA
    // SMSMSASXSS
    // SAXAMASAAA
    // MAMMMXMMMM
    // MXMXAXMASX";
    let data: Vec<Vec<_>> = contents
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    // println!("{:?}", data);
    let height = data.len() as i32;
    let width = data[0].len() as i32;
    let mut count = 0;
    for j in 0..height {
        'second: for i in 0..width {
            let mut total = 0;
            //         println!("Start {}:{}--------", i, j);
            for xs in [(-1, 2), (1, 0)] {
                for ys in [(-1, 2), (1, 0)] {
                    let result = test_xmas(&data, i + xs.1, j + ys.1, xs.0, ys.0, width, height);
                    if result {
                        total += 1;
                    }
                    if total == 2 {
                        //                     println!("Found at {}:{}-------------", i, j);
                        count += 1;
                        continue 'second;
                    }
                }
            }
        }
    }
    println!("{}", count);
    Ok(())
}

fn test_xmas(
    data: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    x_step: i32,
    y_step: i32,
    width: i32,
    height: i32,
) -> bool {
    for (n, c) in "MAS".chars().enumerate() {
        let x = i + x_step * n as i32;
        let y = j + y_step * n as i32;
        if x < 0 || y < 0 || x >= width || y >= width {
            return false;
        }
        //     println!("{}:{}:{}:{}:{}", x, y, data[y as usize][x as usize], c, n);
        if data[y as usize][x as usize] != c {
            return false;
        }
    }
    true
}
