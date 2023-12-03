#[derive(Debug)]
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
}

fn part_01(input: &str) -> u32 {
    let available_symbols = create_available_symbols();
    println!("{:?}", available_symbols);

    let parts = input
        .lines()
        .enumerate()
        .map(|(i, line)| parse_digits(i, line))
        .flatten()
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .fold(vec![], |mut acc: Vec<Symbol>, (i, line)| {
            acc.append(&mut parse_symbols(i, line, available_symbols.clone()));
            acc
        });

    parts
        .iter()
        .filter(|p| part_has_ajacent_symbol(p, symbols.clone()))
        .map(|p| p.serial_number)
        .sum()
}

fn parse_digits(y_pos: usize, line: &str) -> Vec<MachinePart> {
    line.split(|c: char| !c.is_numeric())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let serial_number = s.parse::<u32>().unwrap();
            let start = line.find(s).unwrap();
            let end = start + s.len() - 1;
            MachinePart {
                serial_number,
                x_start: start,
                x_end: end,
                y_pos,
            }
        })
        .collect::<Vec<_>>()
}

fn parse_symbols(y_pos: usize, input: &str, available_symbols: Vec<char>) -> Vec<Symbol> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| available_symbols.contains(c))
        .map(|(x, _)| Symbol { x, y: y_pos })
        .collect::<Vec<_>>()
}

fn part_has_ajacent_symbol(part: &MachinePart, symbols: Vec<Symbol>) -> bool {
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
}
