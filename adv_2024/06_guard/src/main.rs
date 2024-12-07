use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...";
    let grid: Vec<Vec<_>> = contents
        .trim()
        .split("\n")
        .map(|l| l.chars().collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();
    let mut start: (i32, i32) = (0, 0);
    let mut sdir: (i32, i32) = (0, 1);
    for j in 0..height {
        for i in 0..width {
            match grid[j][i] {
                '^' => {
                    start = (i as i32, j as i32);
                    sdir = (0, -1);
                    break;
                }
                '>' => {
                    start = (i as i32, j as i32);
                    sdir = (1, 0);
                    break;
                }
                'v' => {
                    start = (i as i32, j as i32);
                    sdir = (0, 1);
                    break;
                }
                '<' => {
                    start = (i as i32, j as i32);
                    sdir = (-1, 0);
                    break;
                }
                _ => continue,
            }
        }
    }
    let mut loops = 0;
    for j in 0..height {
        for i in 0..width {
            if i == start.0 as usize && j == start.1 as usize {
                continue;
            }
            let mut pos = start.clone();
            let mut dir = sdir.clone();
            let mut ggrid = grid.clone();
            ggrid[j][i] = '#';
            let mut visited = vec![vec![0u8; height]; width];
            loop {
                let dirm = match dir {
                    (0, -1) => 1,
                    (1, 0) => 1 << 1,
                    (0, 1) => 1 << 2,
                    (-1, 0) => 1 << 3,
                    _ => unreachable!(),
                };
                if visited[pos.1 as usize][pos.0 as usize] & dirm > 0 {
                    // found a loop
                    loops += 1;
                    break;
                }

                visited[pos.1 as usize][pos.0 as usize] |= dirm;
                let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if next_pos.0 < 0
                    || next_pos.0 >= width as i32
                    || next_pos.1 < 0
                    || next_pos.1 >= height as i32
                {
                    break;
                }
                if ggrid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
                    dir = match dir {
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => unreachable!(),
                    }
                } else {
                    pos = next_pos;
                }
            }
        }
    }
    println!("{}", loops);
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
//     let grid: Vec<Vec<_>> = contents
//         .trim()
//         .split("\n")
//         .map(|l| l.chars().collect())
//         .collect();

//     let width = grid[0].len();
//     let height = grid.len();
//     let mut pos: (i32, i32) = (0, 0);
//     let mut dir: (i32, i32) = (0, 1);
//     for j in 0..height {
//         for i in 0..width {
//             match grid[j][i] {
//                 '^' => {
//                     pos = (i as i32, j as i32);
//                     dir = (0, -1);
//                     break;
//                 }
//                 '>' => {
//                     pos = (i as i32, j as i32);
//                     dir = (1, 0);
//                     break;
//                 }
//                 'v' => {
//                     pos = (i as i32, j as i32);
//                     dir = (0, 1);
//                     break;
//                 }
//                 '<' => {
//                     pos = (i as i32, j as i32);
//                     dir = (-1, 0);
//                     break;
//                 }
//                 _ => continue,
//             }
//         }
//     }
//     let mut visited = vec![vec![0u32; height]; width];
//     loop {
//         visited[pos.1 as usize][pos.0 as usize] = 1;
//         let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
//         if next_pos.0 < 0
//             || next_pos.0 >= width as i32
//             || next_pos.1 < 0
//             || next_pos.1 >= height as i32
//         {
//             break;
//         }
//         if grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
//             dir = match dir {
//                 (0, -1) => (1, 0),
//                 (1, 0) => (0, 1),
//                 (0, 1) => (-1, 0),
//                 (-1, 0) => (0, -1),
//                 _ => unreachable!(),
//             }
//         } else {
//             pos = next_pos;
//         }
//     }
//     println!(
//         "{}",
//         visited.iter().map(|l| l.iter().sum::<u32>()).sum::<u32>()
//     );
//     Ok(())
// }
