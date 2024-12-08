use std::collections::{HashMap, HashSet};

use grid::Grid;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    process(input, false)
}

fn process(input: &str, calc_antinode_many: bool) -> usize {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    let col_lenght = input.lines().next().unwrap().chars().count();
    let grid_vec = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let grid = Grid::from_vec(grid_vec, col_lenght);

    for col in 0..grid.cols() {
        for row in 0..grid.rows() {
            let cel = grid.get(row, col).unwrap();
            if cel != &'.' {
                if let Some(entry) = antennas.get_mut(cel) {
                    entry.push((row, col));
                } else {
                    antennas.insert(*cel, vec![(row, col)]);
                }
            };
        }
    }

    for key in antennas.keys() {
        for a in antennas.get(key).unwrap() {
            for b in antennas.get(key).unwrap() {
                if a != b {
                    if calc_antinode_many {
                        let many_possible_antinodes =
                            calc_antinode_part_2(a, b, grid.rows(), grid.cols());
                        for possible_antinode in many_possible_antinodes {
                            if grid.get(possible_antinode.0, possible_antinode.1).is_some() {
                                antinodes.insert(possible_antinode);
                            }
                        }
                    } else {
                        let possible_antinode = calc_antinode(a, b);
                        if grid.get(possible_antinode.0, possible_antinode.1).is_some() {
                            antinodes.insert(possible_antinode);
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part_02(input: &str) -> usize {
    process(input, true)
}

#[allow(clippy::cast_possible_wrap)]
fn calc_antinode(a: &(usize, usize), b: &(usize, usize)) -> (isize, isize) {
    let ax = a.0 as isize;
    let ay = a.1 as isize;
    let bx = b.0 as isize;
    let by = b.1 as isize;

    let cx = 2 * bx - ax;
    let cy = 2 * by - ay;
    (cx, cy)
}

#[allow(clippy::cast_possible_wrap)]
fn calc_antinode_part_2(
    a: &(usize, usize),
    b: &(usize, usize),
    row_size: usize,
    col_size: usize,
) -> Vec<(isize, isize)> {
    let mut antinodes = Vec::new();

    let ax = a.0 as isize;
    let ay = a.1 as isize;
    let bx = b.0 as isize;
    let by = b.1 as isize;

    let dx = bx - ax;
    let dy = by - ay;

    for x in 1..=row_size as isize * col_size as isize {
        let cx = bx + x * dx;
        let cy = by + x * dy;

        let cx_reverse = bx - x * dx;
        let cy_reverse = by - x * dy;

        antinodes.push((cx, cy));
        antinodes.push((cx_reverse, cy_reverse));
    }

    antinodes
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(super::part_01(test_input), 14);
    }

    #[test]
    fn part_02() {
        let test_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(super::part_02(test_input), 34);
    }
}
