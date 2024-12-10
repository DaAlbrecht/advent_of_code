use std::collections::HashSet;

use grid::Grid;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn part_01(input: &str) -> usize {
    let cols = input.lines().next().unwrap().chars().count();

    let mut starting_pos = Vec::new();
    let mut end_positions = Vec::new();

    let grid_vec = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '0' {
                        starting_pos.push((row as isize, col as isize));
                    }
                    if c == '9' {
                        end_positions.push((row as isize, col as isize));
                    }
                    c.to_digit(10).unwrap() as isize
                })
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<isize>>();

    let grid = Grid::from_vec(grid_vec, cols);
    starting_pos
        .into_iter()
        .map(|start_pos| {
            end_positions
                .iter()
                .filter(|&&end_pos| walk(start_pos, end_pos, &grid, 0))
                .count()
        })
        .sum()
}

fn part_02(input: &str) -> usize {
    let cols = input.lines().next().unwrap().chars().count();

    let mut starting_pos = Vec::new();

    let grid_vec = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '0' {
                        starting_pos.push((row as isize, col as isize));
                    }
                    c.to_digit(10).unwrap() as isize
                })
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<isize>>();

    let grid = Grid::from_vec(grid_vec, cols);

    let mut distinct_paths: HashSet<Vec<(isize, isize)>> = HashSet::new();
    for start_pos in starting_pos {
        walk_all(
            start_pos,
            &mut vec![start_pos],
            &grid,
            &mut distinct_paths,
            0,
        );
    }
    distinct_paths.len()
}

fn walk(
    current_pos: (isize, isize),
    end_pos: (isize, isize),
    grid: &Grid<isize>,
    current_height: isize,
) -> bool {
    //base case
    if current_pos == end_pos {
        return true;
    }

    for direction in DIRECTIONS {
        let next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        if let Some(&next_height) = grid.get(next_pos.0, next_pos.1) {
            if next_height == current_height + 1 && walk(next_pos, end_pos, grid, next_height) {
                return true;
            }
        }
    }

    false
}

fn walk_all(
    current_pos: (isize, isize),
    current_path: &mut Vec<(isize, isize)>,
    grid: &Grid<isize>,
    visited_trails: &mut HashSet<Vec<(isize, isize)>>,
    current_height: isize,
) {
    for direction in DIRECTIONS {
        let next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);

        if let Some(&next_height) = grid.get(next_pos.0, next_pos.1) {
            if next_height == current_height + 1 {
                current_path.push(next_pos);

                if next_height == 9 {
                    visited_trails.insert(current_path.clone());
                } else {
                    walk_all(next_pos, current_path, grid, visited_trails, next_height);
                }
                current_path.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(super::part_01(test_input), 36);
    }
    #[test]
    fn part_02() {
        let test_input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(super::part_02(test_input), 81);
    }
}
