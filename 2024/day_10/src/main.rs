use grid::Grid;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
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
}
