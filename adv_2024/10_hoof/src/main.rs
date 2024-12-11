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
    //     let contents = "89010123
    // 78121874
    // 87430965
    // 96549874
    // 45678903
    // 32019012
    // 01329801
    // 10456732";
    let grid: Vec<Vec<_>> = contents
        .trim()
        .split("\n")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut sum = 0;
    // let visited = vec![vec![false;grid[0].len()];grid.len()];
    let width = grid[0].len();
    let height = grid.len();
    for j in 0..height {
        for i in 0..width {
            if grid[j][i] != 0 {
                continue;
            }
            let mut visited: HashSet<(usize, usize)> = HashSet::new();

            sum += trail_score(&grid, j, i, height, width, 0, &mut visited);
            // println!("({},{}): {}", j, i, sum);
        }
    }
    println!("Result: {}", sum);
    Ok(())
}

fn trail_score(
    grid: &Vec<Vec<u32>>,
    j: usize,
    i: usize,
    height: usize,
    width: usize,
    curr: u32,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    if curr == 9 {
        // if visited.contains(&(j, i)) {
        //     return 0;
        // }
        // visited.insert((j, i));
        return 1;
    }
    let mut sum = 0;
    if j > 0 {
        let nx = grid[j - 1][i];
        if nx == curr + 1 {
            sum += trail_score(grid, j - 1, i, height, width, nx, visited);
        }
    }
    if i > 0 {
        let nx = grid[j][i - 1];
        if nx == curr + 1 {
            sum += trail_score(grid, j, i - 1, height, width, nx, visited);
        }
    }
    if j < height - 1 {
        let nx = grid[j + 1][i];
        if nx == curr + 1 {
            sum += trail_score(grid, j + 1, i, height, width, nx, visited);
        }
    }
    if i < width - 1 {
        let nx = grid[j][i + 1];
        if nx == curr + 1 {
            sum += trail_score(grid, j, i + 1, height, width, nx, visited);
        }
    }
    sum
}
