fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
}

fn part_01(input: &str) -> usize {
    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    (0..cols)
        .map(|col| {
            let op = grid[rows - 1][col];

            let numbers = grid[..rows - 1]
                .iter()
                .map(|row| row[col].parse::<usize>().unwrap());

            match op {
                "*" => numbers.product::<usize>(),
                "+" => numbers.sum::<usize>(),
                _ => unreachable!(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part_01;

    #[test]
    fn test_01() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(4277556, part_01(input));
    }
}
