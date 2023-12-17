use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
fn heuristic(from: &(usize, usize), to: &(usize, usize)) -> usize {
    (to.0 - from.0) + (to.1 - from.1)
}
// fn neighbours(
//     curr: &(usize, usize),
//     max_size: &(usize, usize),
//     came_from: &HashMap<(usize, usize), (usize, usize)>,
// ) -> Vec<((usize, usize), usize)> {
//     let mut previous = came_from.get(&curr);
//     let last_step = match previous {
//         Some(p) => (
//             (curr.0 as isize - p.0 as isize) as isize,
//             (curr.1 as isize - p.1 as isize) as isize,
//         ),
//         None => (0, 0),
//     };
//     let mut in_row = 1;
//     while last_step != (0, 0) {
//         let curr = previous.unwrap();
//         previous = came_from.get(&previous.unwrap());
//         let prev_step = match previous {
//             Some(p) => (
//                 (curr.0 as isize - p.0 as isize) as isize,
//                 (curr.1 as isize - p.1 as isize) as isize,
//             ),
//             None => (0, 0),
//         };
//         if prev_step == last_step && in_row < 4 {
//             in_row += 1;
//         } else {
//             break;
//         }
//     }
//     let mut poss: Vec<_> = DIRECTIONS
//         .iter()
//         .filter(|&n| {
//             *n != (-last_step.0, -last_step.1) && (in_row < 3 || *n != (last_step.0, last_step.1))
//         })
//         .collect();
//     if in_row < 2 && poss.contains(&&last_step) {
//         for i in 1..3 - in_row {
//             poss.push(&(last_step.0 * i, last_step.1 * i));
//         }
//     }
//     poss.iter()
//         .map(|(x, y)| {
//             curr.0.checked_add_signed(*x).and_then(|a| {
//                 curr.1
//                     .checked_add_signed(*y)
//                     .and_then(|b| Some(((a, b), max(x, y).abs() as usize)))
//             })
//         })
//         .filter_map(|n| n)
//         .filter(|((x, y), _)| *x <= max_size.0 && *y <= max_size.1)
//         .collect()
// }
impl Direction {
    fn orthogonal(&self) -> Vec<Direction> {
        match self {
            Direction::Left | Direction::Right => vec![Direction::Down, Direction::Up],
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
        }
    }
    fn add(
        &self,
        coords: &(usize, usize),
        len: usize,
        max: &(usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Left if coords.0 > len => Some((coords.0 - len, coords.1)),
            Direction::Right if len <= max.0 && coords.0 <= max.0 - len => {
                Some((coords.0 + len, coords.1))
            }
            Direction::Up if coords.1 > len => Some((coords.0, coords.1 - len)),
            Direction::Down if len <= max.1 && coords.1 <= max.1 - len => {
                Some((coords.0, coords.1 + len))
            }
            _ => None,
        }
    }
    fn subtract(
        &self,
        coords: &(usize, usize),
        len: usize,
        max: &(usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Right if coords.0 > len => Some((coords.0 - len, coords.1)),
            Direction::Left if len <= max.0 && coords.0 <= max.0 - len => {
                Some((coords.0 + len, coords.1))
            }
            Direction::Down if coords.1 > len => Some((coords.0, coords.1 - len)),
            Direction::Up if len <= max.1 && coords.1 <= max.1 - len => {
                Some((coords.0, coords.1 + len))
            }
            _ => None,
        }
    }
}
fn neighbours(curr: &Node, max_size: &(usize, usize)) -> Option<Vec<Node>> {
    (1..=3)
        .map(|i| {
            curr.dir
                .orthogonal()
                .iter()
                .map(move |d| {
                    d.add(&curr.coords, i, &max_size).and_then(|c| {
                        Some(Node {
                            coords: c,
                            dir: d.clone(),
                            len: i,
                        })
                    })
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|n| n.is_some())
        .collect()
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Node {
    coords: (usize, usize),
    dir: Direction,
    len: usize,
}
fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let weights: Vec<Vec<usize>> = contents
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let end = (weights[0].len() - 1, weights.len() - 1);

    // let mut pred_cost: HashMap<(usize, usize), usize> = HashMap::new();
    // pred_cost.insert((0, 0), heuristic(&(0, 0), &end, &weights));
    // let mut current = vec![(0, 0)];
    // let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    // let mut cost = weights[end.1][end.0];
    // let mut path = vec![end];
    // while current.len() > 0 {
    //     println!(
    //         "{:?}",
    //         current
    //             .iter()
    //             .map(|n| (n, pred_cost.get(&n).unwrap_or(&99)))
    //             .collect::<Vec<_>>()
    //     );
    //     let curr = current
    //         .iter()
    //         .min_by(|a, b| {
    //             pred_cost
    //                 .get(&a)
    //                 .unwrap_or(&usize::MAX)
    //                 .cmp(pred_cost.get(&b).unwrap_or(&usize::MAX))
    //         })
    //         .unwrap()
    //         .clone();
    //     if curr == end {
    //         //done
    //         let mut curr = curr;
    //         while let Some(prev) = came_from.get(&curr) {
    //             curr = *prev;
    //             path.push(curr);
    //             cost += weights[curr.1][curr.0];
    //         }

    //         break;
    //     }
    //     current.retain(|&n| n != curr);

    //     for neighbour in neighbours(&curr, &end, &came_from) {
    //         let score = costs[&curr] + weights[neighbour.1][neighbour.0];
    //         if score < *costs.get(&neighbour).unwrap_or(&usize::MAX) {
    //             came_from
    //                 .entry(neighbour)
    //                 .and_modify(|v| *v = curr)
    //                 .or_insert(curr);
    //             costs
    //                 .entry(neighbour)
    //                 .and_modify(|v| *v = score)
    //                 .or_insert(score);
    //             pred_cost
    //                 .entry(neighbour)
    //                 .and_modify(|v| *v = score + heuristic(&neighbour, &end, &weights))
    //                 .or_insert(score + heuristic(&neighbour, &end, &weights));
    //             if !current.contains(&neighbour) {
    //                 current.push(neighbour);
    //             }
    //             //make sure this doesn't lead to 4 in a row
    //         }
    //     }
    // }
    let mut dist: HashMap<Node, usize> = HashMap::new();
    dist.insert(
        Node {
            coords: (0, 0),
            dir: Direction::Right,
            len: 1,
        },
        0,
    );
    dist.insert(
        Node {
            coords: (0, 0),
            dir: Direction::Down,
            len: 1,
        },
        0,
    );
    let mut prev: HashMap<Node, Node> = HashMap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    while dist.len() > 0 {
        let dd = dist.clone();
        let cur = dd.iter().min_by_key(|v| v.1).unwrap().clone();
        // println!("{:?}", cur);
        dist.remove_entry(&cur.0);
        if visited.contains(cur.0) {
            continue;
        }
        visited.insert(cur.0.clone());
        if cur.0.coords == end {
            println!("{}", cur.1);
            break;
        }
        let neighs = neighbours(cur.0, &end);
        // println!("{:?}", neighs);
        if let Some(neighs) = neighs {
            for neighbour in neighs {
                let cost = (0..neighbour.len)
                    .map(|i| neighbour.dir.subtract(&neighbour.coords, i, &end).unwrap())
                    .map(|(x, y)| weights[y][x])
                    .sum::<usize>();
                println!("{:?}: {:?} c:{:?}", cur, neighbour, cost);
                let cost = cur.1 + cost;
                if neighbour.coords == end {
                    println!("cost:{}", cost);
                }

                dist.entry(neighbour.clone())
                    .and_modify(|v| {
                        if cost < *v {
                            *v = cost;
                            // prev.entry(neighbour)
                            //     .and_modify(|p| *p = *cur.0)
                            //     .or_insert(*cur.0);
                        }
                    })
                    .or_insert_with(|| {
                        // prev.entry(neighbour)
                        //     .and_modify(|p| *p = *cur.0)
                        //     .or_insert(*cur.0);
                        cost
                    });
            }
        }
    }
    let mut cost = 0;
    let mut path = vec![&end];
    // println!("{:?}", visited);
    // println!("calculating path");
    // let mut cur = visited.iter().filter(|n| n.coords == end).next().unwrap();
    // while let Some(p) = prev.get(cur) {
    //     // println!("p {:?}: cur {:?}", p, cur);
    //     path.push(&p.coords);
    //     cost += (1..=cur.len)
    //         .map(|i| cur.dir.add(&p.coords, i, &end).unwrap())
    //         // .map(|x| dbg!(x))
    //         .map(|(x, y)| weights[y][x])
    //         .sum::<usize>();
    //     cur = p;
    // }

    // println!("{:?}", path);
    // println!("{}", cost);

    Ok(())
}
