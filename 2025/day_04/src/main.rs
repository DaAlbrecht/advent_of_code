fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
    dbg!(part_02(&file));
}

static DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part_01(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != '@' {
                continue;
            }

            let mut adjacant = 0;

            for (dr, dc) in DIRS {
                let next_row = (row as isize + dr) as usize;
                let next_col = (col as isize + dc) as usize;

                if next_row < rows && next_col < cols && grid[next_row][next_col] == '@' {
                    adjacant += 1;
                }
            }

            if adjacant < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part_02(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    let mut found_rolle = false;

    loop {
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != '@' {
                    continue;
                }

                let mut adjacant = 0;

                for (dr, dc) in DIRS {
                    let next_row = (row as isize + dr) as usize;
                    let next_col = (col as isize + dc) as usize;

                    if next_row < rows && next_col < cols && grid[next_row][next_col] == '@' {
                        adjacant += 1;
                    }
                }

                if adjacant < 4 {
                    // found rolle yeei, remove it
                    found_rolle = true;
                    grid[row][col] = '.';
                    count += 1;
                }
            }
        }
        // no more rolles to remove :(
        if !found_rolle {
            return count;
        }
        found_rolle = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::{part_01, part_02};

    #[test]
    fn test_01() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(13, part_01(input));
    }

    #[test]
    fn test_02() {
        let input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(43, part_02(input));
    }
}
