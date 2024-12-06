use std::collections::{HashMap, HashSet};

use grid::Grid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Top,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let mut distinct_positions = HashSet::new();
    let mut guard_pos = Position { x: 0, y: 0 };
    let line_len = input.lines().next().unwrap().chars().count();
    let grid_vec = input
        .lines()
        .enumerate()
        .flat_map(|(col_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(row_idx, c)| {
                    if c == '^' {
                        guard_pos = Position {
                            x: row_idx as isize,
                            y: col_idx as isize,
                        };
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>();

    let grid = Grid::from_vec(grid_vec, line_len);
    distinct_positions.insert(guard_pos.clone());

    let mut direction = Direction::Top;
    while let (next_pos, Some(next_char)) = move_guard(guard_pos.clone(), &grid, direction.clone())
    {
        distinct_positions.insert(guard_pos.clone());
        match next_char {
            '#' => match direction {
                Direction::Top => direction = Direction::Right,
                Direction::Right => direction = Direction::Down,
                Direction::Down => direction = Direction::Left,
                Direction::Left => direction = Direction::Top,
            },
            _ => guard_pos = next_pos,
        }
    }
    distinct_positions.insert(guard_pos.clone());
    distinct_positions.len()
}

fn part_02(input: &str) -> usize {
    let mut guard_start_pos = Position { x: 0, y: 0 };
    let line_len = input.lines().next().unwrap().chars().count();
    let grid_vec = input
        .lines()
        .enumerate()
        .flat_map(|(col_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(row_idx, c)| {
                    if c == '^' {
                        guard_start_pos = Position {
                            x: row_idx as isize,
                            y: col_idx as isize,
                        };
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>();

    let grid = Grid::from_vec(grid_vec, line_len);

    let mut possible_grids = Vec::new();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if *grid.get(row, col).unwrap() == '^' {
                possible_grids.push(grid.clone());
            }
            let mut possible_grid = grid.clone();
            let cel = possible_grid.get_mut(row, col).unwrap();
            *cel = '#';
            possible_grids.push(possible_grid);
        }
    }

    possible_grids
        .iter()
        .filter(|possible_grid| {
            let mut pair_counts: HashMap<(Position, Direction), usize> = HashMap::new();
            let mut direction = Direction::Top;
            let mut guard_pos = guard_start_pos.clone();
            while let (next_pos, Some(next_char)) =
                move_guard(guard_pos.clone(), possible_grid, direction.clone())
            {
                match next_char {
                    '#' => match direction {
                        Direction::Top => {
                            let entry = pair_counts.entry((next_pos, direction)).or_insert(0);
                            *entry += 1;
                            if *entry > 1 {
                                return true;
                            }
                            direction = Direction::Right
                        }
                        Direction::Right => {
                            let entry = pair_counts.entry((next_pos, direction)).or_insert(0);
                            *entry += 1;
                            if *entry > 1 {
                                return true;
                            }
                            direction = Direction::Down
                        }
                        Direction::Down => {
                            let entry = pair_counts.entry((next_pos, direction)).or_insert(0);
                            *entry += 1;
                            if *entry > 1 {
                                return true;
                            }
                            direction = Direction::Left
                        }
                        Direction::Left => {
                            let entry = pair_counts.entry((next_pos, direction)).or_insert(0);
                            *entry += 1;
                            if *entry > 1 {
                                return true;
                            }
                            direction = Direction::Top
                        }
                    },
                    _ => guard_pos = next_pos,
                };
            }
            false
        })
        .count()
}

fn move_guard(
    guard_pos: Position,
    grid: &Grid<char>,
    direction: Direction,
) -> (Position, Option<char>) {
    let mut next_pos = guard_pos;
    match direction {
        Direction::Top => {
            next_pos.y -= 1;
        }
        Direction::Right => {
            next_pos.x += 1;
        }
        Direction::Down => {
            next_pos.y += 1;
        }
        Direction::Left => {
            next_pos.x -= 1;
        }
    };

    (next_pos.clone(), grid.get(next_pos.y, next_pos.x).copied())
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(super::part_01(test_input), 41);
    }
    #[test]
    fn part_02() {
        let test_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(super::part_02(test_input), 6);
    }
}
