use grid::Grid;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let length = input.lines().count();
    let input_grid = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let grid = Grid::from_vec(input_grid, length);
    let mut x_start = Vec::new();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid.get(row, col) == Some(&'X') {
                x_start.push((row as isize, col as isize));
            }
        }
    }

    x_start
        .iter()
        .map(|start| solve_part_01(*start, &grid))
        .sum()
}

fn part_02(input: &str) -> usize {
    let length = input.lines().count();
    let input_grid = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<char>>();

    let grid = Grid::from_vec(input_grid, length);
    let mut a_start = Vec::new();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid.get(row, col) == Some(&'A') {
                a_start.push((row as isize, col as isize));
            }
        }
    }

    a_start
        .iter()
        .filter(|start| solve_part_02(**start, &grid))
        .count()
}

fn solve_part_01(start_pos: (isize, isize), grid: &Grid<char>) -> usize {
    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut found_words = 0;

    for &direction in &DIRECTIONS {
        let mut current_pos = start_pos;
        let mut matches = true;

        for &c in &WORD {
            if grid.get(current_pos.0, current_pos.1) != Some(&c) {
                matches = false;
                break;
            }
            current_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        }
        if matches {
            found_words += 1;
        }
    }
    found_words
}

fn solve_part_02(start: (isize, isize), grid: &Grid<char>) -> bool {
    let top_left = grid.get(start.0 - 1, start.1 + 1);
    let bottom_right = grid.get(start.0 + 1, start.1 - 1);
    let top_right = grid.get(start.0 + 1, start.1 + 1);
    let bottom_left = grid.get(start.0 - 1, start.1 - 1);

    const WORD: [char; 2] = ['S', 'M'];

    if top_right.is_none() || top_left.is_none() || bottom_left.is_none() || bottom_right.is_none()
    {
        return false;
    }

    if top_left == bottom_right
        || top_right == bottom_left
        || !WORD.contains(top_right.unwrap())
        || !WORD.contains(top_left.unwrap())
        || !WORD.contains(bottom_right.unwrap())
        || !WORD.contains(bottom_left.unwrap())
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(super::part_01(test_input), 18);
    }
    #[test]
    fn part_02() {
        let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(super::part_02(test_input), 9);
    }
}
