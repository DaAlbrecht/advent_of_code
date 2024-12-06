use std::collections::HashSet;

use grid::Grid;

#[derive(Debug, Clone)]
enum Direction {
    TOP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
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

    let mut direction = Direction::TOP;
    while let (next_pos, Some(next_char)) = move_guard(guard_pos.clone(), &grid, direction.clone())
    {
        distinct_positions.insert(guard_pos.clone());
        match next_char {
            '#' => match direction {
                Direction::TOP => direction = Direction::RIGHT,
                Direction::RIGHT => direction = Direction::DOWN,
                Direction::DOWN => direction = Direction::LEFT,
                Direction::LEFT => direction = Direction::TOP,
            },
            _ => guard_pos = next_pos,
        }
    }
    distinct_positions.insert(guard_pos.clone());
    distinct_positions.iter().count()
}

fn move_guard(
    guard_pos: Position,
    grid: &Grid<char>,
    direction: Direction,
) -> (Position, Option<char>) {
    let mut next_pos = guard_pos;
    match direction {
        Direction::TOP => {
            next_pos.y -= 1;
        }
        Direction::RIGHT => {
            next_pos.x += 1;
        }
        Direction::DOWN => {
            next_pos.y += 1;
        }
        Direction::LEFT => {
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
}
