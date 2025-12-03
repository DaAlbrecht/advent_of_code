fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
}

fn part_01(file: &str) -> usize {
    file.lines()
        .map(|line| {
            let banks = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let max_first = banks[..banks.len() - 1].iter().max().unwrap();
            let first_pos = banks.iter().position(|v| v == max_first).unwrap();

            let max_second = banks[first_pos + 1..banks.len()].iter().max().unwrap();

            format!("{max_first}{max_second}").parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::part_01;

    #[test]
    fn test_01() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(357, part_01(input));
    }
}
