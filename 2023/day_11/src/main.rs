#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Type {
    Galaxy,
    Empty,
    Old,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OldPoint {
    x: usize,
    y: usize,
    old_y: usize,
}

fn main() {
    let input = std::fs::read_to_string("puzzle").expect("Unable to read input file");
    println!("Part 01: {}", part_01(&input));
    println!("Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Type::Empty,
                    '#' => Type::Galaxy,
                    _ => panic!("Unknown type"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let input = expand(&input);

    let points = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &t)| t == Type::Galaxy)
                .map(move |(x, _)| Point { x, y })
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    for point in &points {
        let distances = shortest_paths(*point, &points);
        total += distances.iter().sum::<u64>();
    }
    total / 2
}

fn part_02(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Type::Empty,
                    '#' => Type::Galaxy,
                    _ => panic!("Unknown type"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let input = mark_old(&input);

    let points = exand_old(&input);

    let mut total = 0;

    for point in &points {
        let distances = shortest_paths(*point, &points);
        total += distances.iter().sum::<u64>();
    }
    total / 2
}

fn mark_old(input: &Vec<Vec<Type>>) -> Vec<Vec<Type>> {
    let mut new_input = input.clone();
    let old_row = vec![Type::Old; input[0].len()];
    for row in 0..input.len() {
        if input[row].iter().all(|&t| t == Type::Empty) {
            new_input[row] = old_row.clone();
        }
    }
    for col in 0..input[0].len() {
        if input.iter().all(|row| row[col] == Type::Empty) {
            new_input.iter_mut().for_each(|row| row[col] = Type::Old);
        }
    }
    new_input
}

fn exand_old(input: &[Vec<Type>]) -> Vec<Point> {
    let mut old_rows_count = 0;
    let mut old_cols_count = 0;

    let mut exanded_galaxies = Vec::new();

    for (row, item) in input.iter().enumerate() {
        if item.iter().all(|&t| t == Type::Old) {
            old_rows_count += 1;
        }
        for (col, item) in item.iter().enumerate() {
            if *item == Type::Galaxy {
                let new_row = row + (old_rows_count * (1000000 - 1));
                let point = OldPoint {
                    x: col,
                    y: row,
                    old_y: new_row,
                };
                exanded_galaxies.push(point);
            }
        }
    }

    for col in 0..input[0].len() {
        if input.iter().all(|row| row[col] == Type::Old) {
            old_cols_count += 1;
        }
        for (row, item) in input.iter().enumerate() {
            if item[col] == Type::Galaxy {
                let point = exanded_galaxies
                    .iter()
                    .find(|&p| p.x == col && p.y == row)
                    .unwrap();
                let new_col = col + (old_cols_count * (1000000 - 1));
                let new_point = OldPoint {
                    x: new_col,
                    y: row,
                    old_y: point.old_y,
                };

                exanded_galaxies.iter_mut().for_each(|p| {
                    if p.x == col && p.y == row {
                        *p = new_point;
                    }
                });
            }
        }
    }

    exanded_galaxies
        .iter()
        .map(|p| Point { x: p.x, y: p.old_y })
        .collect::<Vec<_>>()
}

fn expand(input: &[Vec<Type>]) -> Vec<Vec<Type>> {
    let mut new_input = input.to_vec();
    let mut shifted = 0;
    for (row, item) in input.iter().enumerate() {
        if item.iter().all(|&t| t == Type::Empty) {
            new_input.insert(row + shifted, input[row].clone());
            shifted += 1;
        }
    }
    shifted = 0;
    for col in 0..input[0].len() {
        if input.iter().all(|row| row[col] == Type::Empty) {
            new_input
                .iter_mut()
                .for_each(|row| row.insert(col + shifted, Type::Empty));
            shifted += 1;
        }
    }
    new_input
}

fn shortest_paths(position: Point, destinations: &[Point]) -> Vec<u64> {
    let mut paths = Vec::new();
    for destination in destinations {
        let x = (position.x as i64 - destination.x as i64).abs();
        let y = (position.y as i64 - destination.y as i64).abs();
        if x == 0 && y == 0 {
            continue;
        }
        paths.push(x as u64 + y as u64);
    }
    paths
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(super::part_01(&input), 374);
    }
}
