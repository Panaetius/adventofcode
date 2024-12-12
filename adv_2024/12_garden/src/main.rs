use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "RRRRIICCFF
    // RRRRIICCCF
    // VVRRRCCFFF
    // VVRCCCJFFF
    // VVVVCJJCFE
    // VVIVCCJJEE
    // VVIIICJJEE
    // MIIIIIJJEE
    // MIIISIJEEE
    // MMMISSJEEE";
    let grid: Vec<Vec<_>> = contents
        .trim()
        .split("\n")
        .map(|l| l.chars().collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut components: Vec<(usize, Vec<(usize, usize)>)> = Vec::new();
    for j in 0..height {
        for i in 0..width {
            if visited[j][i] {
                continue;
            }
            let res = flood_fill2(&grid, &mut visited, grid[j][i], height, width, j, i);
            components.push(res);
        }
    }
    let mut sum = 0;
    for (f, g) in components {
        // println!("{}*{}", f, g.len());
        let count = g.len();
        sum += count * f;
    }

    println!("Result: {}", sum);
    Ok(())
}

fn flood_fill(
    grid: &[Vec<char>],
    visited: &mut [Vec<bool>],
    c: char,
    height: usize,
    width: usize,
    j: usize,
    i: usize,
) -> (usize, Vec<(usize, usize)>) {
    let mut stack = vec![(i, j)];
    let mut res = Vec::new();
    let mut f = 0;
    while let Some((i, j)) = stack.pop() {
        if visited[j][i] {
            continue;
        }
        res.push((i, j));
        visited[j][i] = true;
        if i > 0 && grid[j][i - 1] == c {
            stack.push((i - 1, j));
        } else {
            f += 1;
        }

        if j > 0 && grid[j - 1][i] == c {
            stack.push((i, j - 1));
        } else {
            f += 1;
        }
        if i < width - 1 && grid[j][i + 1] == c {
            stack.push((i + 1, j));
        } else {
            f += 1;
        }
        if j < height - 1 && grid[j + 1][i] == c {
            stack.push((i, j + 1));
        } else {
            f += 1;
        }
    }
    (f, res)
}
fn flood_fill2(
    grid: &[Vec<char>],
    visited: &mut [Vec<bool>],
    c: char,
    height: usize,
    width: usize,
    j: usize,
    i: usize,
) -> (usize, Vec<(usize, usize)>) {
    let mut stack = vec![(i, j)];
    let mut res = Vec::new();
    let mut corners = 0;
    while let Some((i, j)) = stack.pop() {
        if visited[j][i] {
            continue;
        }
        res.push((i, j));
        visited[j][i] = true;
        let mut check = false;
        let mut first_check = false;
        if i > 0 && grid[j][i - 1] == c {
            stack.push((i - 1, j));
        } else {
            check = true;
            first_check = true;
        }
        if j > 0 && grid[j - 1][i] == c {
            stack.push((i, j - 1));
            if !check && (i == 0 || grid[j - 1][i - 1] != c) {
                corners += 1;
            }
            check = false;
        } else {
            if check {
                corners += 1;
            }
            check = true;
        }
        if i < width - 1 && grid[j][i + 1] == c {
            stack.push((i + 1, j));
            if !check && (j == 0 || grid[j - 1][i + 1] != c) {
                corners += 1;
            }
            check = false;
        } else {
            if check {
                corners += 1;
            }
            check = true;
        }
        if j < height - 1 && grid[j + 1][i] == c {
            stack.push((i, j + 1));
            if !check && (i == width || grid[j + 1][i + 1] != c) {
                corners += 1;
            }
            check = false;
        } else {
            if check {
                corners += 1;
            }
            check = true;
        }
        if !check && !first_check && (i == 0 || j == height || grid[j + 1][i - 1] != c) {
            corners += 1;
        }
        if check && first_check {
            corners += 1;
        }
    }
    (corners, res)
}
