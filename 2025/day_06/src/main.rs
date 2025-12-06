fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
    dbg!(part_02(&file));
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

fn part_02(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let columns: Vec<Vec<char>> = (0..cols)
        .map(|c| lines.iter().map(|row| row[c]).collect())
        .collect();

    let mut total = 0;
    let mut chunk: Vec<usize> = Vec::new();

    let mut collumn = cols.checked_sub(1);
    while let Some(col) = collumn {
        let column = &columns[col];

        if column.iter().all(|c| *c == ' ') {
            if !chunk.is_empty() {
                let op = columns[col + 1][rows - 1];
                let val = match op {
                    '*' => chunk.iter().product::<usize>(),
                    '+' => chunk.iter().sum::<usize>(),
                    _ => unreachable!(),
                };
                total += val;
                chunk.clear();
            }
        } else {
            let num: usize = column[..rows - 1]
                .iter()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            chunk.push(num);
        }

        collumn = col.checked_sub(1);
    }

    if !chunk.is_empty() {
        let op = columns[0][rows - 1];
        let val = match op {
            '*' => chunk.iter().product::<usize>(),
            '+' => chunk.iter().sum::<usize>(),
            _ => unreachable!(),
        };
        total += val;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::{part_01, part_02};

    #[test]
    fn test_01() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(4277556, part_01(input));
    }

    #[test]
    fn test_02() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(3263827, part_02(input));
    }
}
