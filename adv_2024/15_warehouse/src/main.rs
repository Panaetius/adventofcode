use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

use itertools::Itertools;
#[derive(Copy, Clone, Debug)]
enum Position {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Player,
}
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn get_dir(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),

            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "##########
    // #..O..O.O#
    // #......O.#
    // #.OO..O.O#
    // #..O@..O.#
    // #O#..O...#
    // #O..O..O.#
    // #.OO.O.OO#
    // #....O...#
    // ##########

    // <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    // vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    // ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    // <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    // ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    // ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    // >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    // <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    // ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    // v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    // ";
    let (game_map, instructions) = contents.split_once("\n\n").unwrap();
    let mut player = (0, 0);
    let mut game_map: Vec<Vec<_>> = game_map
        .split("\n")
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .flat_map(|(i, c)| match c {
                    '#' => vec![Position::Wall, Position::Wall],
                    'O' => vec![Position::BoxLeft, Position::BoxRight],
                    '@' => {
                        player = (i * 2, j);
                        vec![Position::Player, Position::Empty]
                    }
                    _ => vec![Position::Empty, Position::Empty],
                })
                .collect()
        })
        .collect();
    let instructions = instructions
        .split("\n")
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    for instruction in instructions {
        // println!(
        //     "{}\nDir:{:?}",
        //     game_map
        //         .iter()
        //         .map(|l| l
        //             .iter()
        //             .map(|p| match p {
        //                 Position::Empty => ".",
        //                 Position::Wall => "#",
        //                 Position::BoxLeft => "[",
        //                 Position::BoxRight => "]",
        //                 Position::Player => "@",
        //             })
        //             .collect::<Vec<_>>()
        //             .join(""))
        //         .collect::<Vec<_>>()
        //         .join("\n"),
        //     instruction,
        // );
        if let Some(moves) = move_object(&game_map, player, instruction) {
            let moves: Vec<_> = moves.iter().unique().collect();
            println!("{:?}", moves);

            //swap positions iteratively
            for (to, from) in moves {
                let tmp = game_map[from.1][from.0];
                game_map[from.1][from.0] = game_map[to.1][to.0];
                game_map[to.1][to.0] = tmp;
                player = *to;
            }
        }
    }
    println!(
        "{}",
        game_map
            .iter()
            .map(|l| l
                .iter()
                .map(|p| match p {
                    Position::Empty => ".",
                    Position::Wall => "#",
                    Position::BoxLeft => "[",
                    Position::BoxRight => "]",
                    Position::Player => "@",
                })
                .collect::<Vec<_>>()
                .join(""))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let res: usize = game_map
        .iter()
        .enumerate()
        .map(|(j, l)| {
            l.iter()
                .enumerate()
                .map(|(i, c)| match c {
                    Position::BoxLeft => i + 100 * j,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum();
    println!("{}", res);

    Ok(())
}

fn move_object(
    game_map: &Vec<Vec<Position>>,
    position: (usize, usize),
    direction: Direction,
) -> Option<Vec<((usize, usize), (usize, usize))>> {
    let dir = direction.get_dir();
    let new_pos = (
        (position.0 as isize + dir.0 as isize) as usize,
        (position.1 as isize + dir.1 as isize) as usize,
    );
    let nx = game_map[new_pos.1][new_pos.0];

    match (nx, direction) {
        (Position::Empty, _) => Some(vec![(new_pos, position)]),
        (Position::Wall, _) => None,
        (Position::BoxLeft, Direction::Up) | (Position::BoxLeft, Direction::Down) => {
            move_object(game_map, new_pos, direction).and_then(|r| {
                move_object(game_map, (new_pos.0 + 1, new_pos.1), direction)
                    .map(|r2| [r, r2, vec![(new_pos, position)]].concat())
            })
        }
        (Position::BoxRight, Direction::Up) | (Position::BoxRight, Direction::Down) => {
            move_object(game_map, new_pos, direction).and_then(|r| {
                move_object(
                    game_map,
                    ((new_pos.0 as isize - 1) as usize, new_pos.1),
                    direction,
                )
                .map(|r2| [r, r2, vec![(new_pos, position)]].concat())
            })
        }
        (Position::BoxLeft, _) => move_object(game_map, new_pos, direction)
            .map(|r| [r, vec![(new_pos, position)]].concat()),
        (Position::BoxRight, _) => move_object(game_map, new_pos, direction)
            .map(|r| [r, vec![(new_pos, position)]].concat()),
        (Position::Player, _) => unreachable!(),
    }
}

// Dir:Down
// ####################
// ##....[]....[]..[]##
// ##............[]..##
// ##..[][]....[]..[]##
// ##...[].......[]..##
// ##[]##....[]......##
// ##[]......[]..[]..##
// ##..[][]..@[].[][]##
// ##........[]......##
// ####################
// Dir:Up
// [((10, 4), (10, 5)), ((11, 4), (11, 5)), ((10, 5), (10, 6)), ((11, 4), (11, 5)), ((10, 4), (10, 5)), ((11, 5), (11, 6)), ((10, 6), (10, 7))]
// ####################
// ##....[]....[]..[]##
// ##............[]..##
// ##..[][]....[]..[]##
// ##...[]...[...[]..##
// ##[]##....[]......##
// ##[]......@]..[]..##
// ##..[][]...[].[][]##
// ##........[]......##
// ####################
// Dir:Down
// [((10, 7), (10, 6))]
