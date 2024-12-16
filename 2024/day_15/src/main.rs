use std::{collections::HashMap, fmt::Display};

use glam::IVec2;
use grid::Grid;

#[derive(Debug)]
enum Tile {
    Wall,
    Empty,
    Box,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, "."),
            Tile::Box => write!(f, "0"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let (warehouse, instructions) = input.split_once("\n\n").unwrap();

    let cols = warehouse.lines().next().unwrap().chars().count();

    let robot_position = warehouse
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c == '@' {
                    return Some(IVec2::new(row as i32, col as i32));
                }
                None
            })
        })
        .collect::<Vec<IVec2>>();

    let mut robot_position = robot_position[0];

    let grid_vec = warehouse
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '#' => Tile::Wall,
                '.' | '@' => Tile::Empty,
                'O' => Tile::Box,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<Tile>>();

    let mut grid = Grid::from_vec(grid_vec, cols);
    let instructions = instructions
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<char>>();

    let directions: HashMap<char, IVec2> = HashMap::from([
        ('<', IVec2::new(0, -1)),
        ('>', IVec2::new(0, 1)),
        ('^', IVec2::new(-1, 0)),
        ('v', IVec2::new(1, 0)),
    ]);

    for instruction in &instructions {
        let next_pos = robot_position + directions.get(instruction).unwrap();
        let mut box_vec = Vec::new();

        match grid.get(next_pos.x, next_pos.y).unwrap() {
            Tile::Wall => continue,
            Tile::Empty => {
                robot_position = next_pos;
            }
            Tile::Box => {
                box_vec.push(next_pos);
                let mut free_space = false;
                for i in 1..grid.cols() + grid.rows() {
                    let mut next_free_pos = next_pos;
                    for _ in 0..i {
                        next_free_pos += directions.get(instruction).unwrap();
                    }
                    match grid.get(next_free_pos.x, next_free_pos.y).unwrap() {
                        Tile::Wall => {
                            free_space = false;
                            break;
                        }
                        Tile::Empty => {
                            free_space = true;
                            break;
                        }
                        Tile::Box => {
                            box_vec.push(next_free_pos);
                            continue;
                        }
                    }
                }
                if !free_space {
                    continue;
                }

                for item in box_vec.iter().rev() {
                    {
                        let current_cell = grid.get_mut(item.x, item.y).unwrap();
                        *current_cell = Tile::Empty;
                    }
                    let next_cell_pos = item + directions.get(instruction).unwrap();
                    let next_cell = grid.get_mut(next_cell_pos.x, next_cell_pos.y).unwrap();
                    *next_cell = Tile::Box;
                }
                robot_position = next_pos;
            }
        }
    }

    calculate_gps(&grid)
}

fn calculate_gps(grid: &Grid<Tile>) -> usize {
    let mut gps_sum = 0;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if let Some(Tile::Box) = grid.get(row, col) {
                gps_sum += 100 * row + col;
            }
        }
    }
    gps_sum
}

fn debug_grid(grid: &Grid<Tile>, robot_position: IVec2) {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let tile = if IVec2::new(row as i32, col as i32) == robot_position {
                '@'
            } else {
                match grid.get(row, col).unwrap() {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Box => 'O',
                }
            };
            print!("{tile}");
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let test_input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
         ";
        assert_eq!(super::part_01(test_input), 2028);
    }
    #[test]
    fn part_01_big() {
        let test_input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(super::part_01(test_input), 10092);
    }
}
