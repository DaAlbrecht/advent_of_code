use grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let cols = input.lines().next().unwrap().chars().count();
    let grid_vec = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();
    let grid = Grid::from_vec(grid_vec, cols);
    let rows = grid.rows();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let (area, perimeter) = walk(&grid, &mut visited, row, col);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn part_02(input: &str) -> usize {
    let cols = input.lines().next().unwrap().chars().count();
    let grid_vec = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();
    let grid = Grid::from_vec(grid_vec, cols);
    let rows = grid.rows();
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let (area, corners) = walk_2(&grid, &mut visited, row, col);
                total_price += area * corners;
            }
        }
    }

    total_price
}

fn walk(
    grid: &Grid<char>,
    visited: &mut HashSet<(usize, usize)>,
    start_row: usize,
    start_col: usize,
) -> (usize, usize) {
    let mut stack = vec![(start_row, start_col)];
    let mut area = 0;
    let mut perimeter = 0;
    let rows = grid.rows();
    let cols = grid.cols();
    let region_type = grid.get(start_row, start_col);

    while let Some((row, col)) = stack.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));
        area += 1;

        for &(dr, dc) in &DIRECTIONS {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                perimeter += 1;
            } else {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid.get(nr, nc) == region_type {
                    if !visited.contains(&(nr, nc)) {
                        stack.push((nr, nc));
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
    }

    (area, perimeter)
}
fn walk_2(
    grid: &Grid<char>,
    visited: &mut HashSet<(usize, usize)>,
    start_row: usize,
    start_col: usize,
) -> (usize, usize) {
    let mut stack = vec![(start_row, start_col)];
    let mut group = Vec::new();
    let region_type = grid.get(start_row, start_col);
    let rows = grid.rows();
    let cols = grid.cols();

    while let Some((row, col)) = stack.pop() {
        if visited.contains(&(row, col)) {
            continue;
        }

        visited.insert((row, col));
        group.push((row, col));

        for &(dr, dc) in &DIRECTIONS {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if grid.get(nr, nc) == region_type && !visited.contains(&(nr, nc)) {
                    stack.push((nr, nc));
                }
            }
        }
    }

    let sides = group
        .iter()
        .map(|&(row, col)| count_corners((row as isize, col as isize), grid, *region_type.unwrap()))
        .sum();

    (group.len(), sides)
}

fn count_corners(n: (isize, isize), grid: &Grid<char>, group_id: char) -> usize {
    let mut count = 0;
    for ((x, y), (x1, y1)) in DIRECTIONS.iter().circular_tuple_windows() {
        let test_a = grid.get(x + n.0, y + n.1).is_some_and(|c| *c == group_id);
        let test_b = grid.get(x1 + n.0, y1 + n.1).is_some_and(|c| *c == group_id);
        if test_a
            && test_b
            && grid
                .get(x + x1 + n.0, y + y1 + n.1)
                .is_some_and(|c| *c != group_id)
            || !test_a && !test_b
        {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let test_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(super::part_01(test_input), 1930);
    }

    #[test]
    fn part_02() {
        let test_input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(super::part_02(test_input), 1206);
    }
}
