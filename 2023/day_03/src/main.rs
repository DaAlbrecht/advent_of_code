#[derive(Debug, Clone)]
struct MachinePart {
    serial_number: u32,
    x_start: usize,
    x_end: usize,
    y_pos: usize,
}

#[derive(Debug, Clone)]
struct Symbol {
    x: usize,
    y: usize,
}

fn main() {
    let input = std::fs::read_to_string("puzzle").unwrap();
    println!("part 01: {}", part_01(&input));
    println!("part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u32 {
    let available_symbols = create_available_symbols();
    println!("{:?}", available_symbols);

    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| parse_digits(i, line))
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .fold(vec![], |mut acc: Vec<Symbol>, (i, line)| {
            acc.append(&mut parse_symbols(i, line, &available_symbols));
            acc
        });

    parts
        .iter()
        .filter(|p| part_has_ajacent_symbol(p, &symbols))
        .map(|p| p.serial_number)
        .sum()
}

fn part_02(input: &str) -> u32 {
    let available_symbols = vec!['*'];

    let parts = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| parse_digits(i, line))
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .fold(vec![], |mut acc: Vec<Symbol>, (i, line)| {
            acc.append(&mut parse_symbols(i, line, &available_symbols));
            acc
        });

    symbols
        .iter()
        .map(|s| find_adjacent_parts_for_symbol(s, &parts))
        .filter(|p| p.len() == 2)
        .map(|p| p[0].serial_number * p[1].serial_number)
        .sum()
}

fn find_adjacent_parts_for_symbol(symbol: &Symbol, parts: &[MachinePart]) -> Vec<MachinePart> {
    parts
        .iter()
        .filter(|p| is_adjacent(p, symbol))
        .cloned()
        .collect::<Vec<_>>()
}

fn parse_digits(y_pos: usize, line: &str) -> Vec<MachinePart> {
    let mut iter = line.chars().enumerate().peekable();
    let mut machine_parts: Vec<MachinePart> = Vec::new();

    while let Some((i, c)) = iter.next() {
        if c.is_ascii_digit() {
            let x_start = i;
            let x_end = x_start
                + iter
                    .by_ref()
                    .take_while(|(_, next_char)| next_char.is_ascii_digit())
                    .count();

            let number_str: String = line[x_start..=x_end].chars().collect();
            let number: u32 = number_str.parse().unwrap();

            let machine_part = MachinePart {
                serial_number: number,
                x_start,
                x_end,
                y_pos,
            };

            machine_parts.push(machine_part);
        }
    }
    machine_parts
}

fn parse_symbols(y_pos: usize, input: &str, available_symbols: &[char]) -> Vec<Symbol> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| available_symbols.contains(c))
        .map(|(x, _)| Symbol { x, y: y_pos })
        .collect::<Vec<_>>()
}

fn part_has_ajacent_symbol(part: &MachinePart, symbols: &[Symbol]) -> bool {
    symbols.iter().any(|s| is_adjacent(part, s))
}

fn is_adjacent(part: &MachinePart, symbol: &Symbol) -> bool {
    let dx = (symbol.x as isize) - (part.x_start as isize);
    let dy = (symbol.y as isize) - (part.y_pos as isize);

    (dy.abs() <= 1)
        && ((dx.abs() <= 1)
            || (symbol.x as isize >= part.x_start as isize - 1
                && symbol.x as isize <= part.x_end as isize + 1))
}

fn create_available_symbols() -> Vec<char> {
    (33..127)
        .filter_map(|x| {
            if (x <= 47 && x != 46)
                || (x >= 58 && x <= 64)
                || (x >= 91 && x <= 96)
                || (x >= 123 && x <= 126)
            {
                Some(x as u8 as char)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(crate::part_01(input), 4361);
    }

    #[test]
    fn larger_test() {
        let input = std::fs::read_to_string("test").unwrap();
        assert_eq!(crate::part_01(&input), 2781);
    }

    #[test]
    fn part_02() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(crate::part_02(input), 467835);
    }
}
