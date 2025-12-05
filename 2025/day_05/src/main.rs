use std::collections::HashSet;

fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
}

fn part_01(input: &str) -> usize {
    let mut fresh_ingridiants = HashSet::new();
    let (id_ranges, ingridiants) = input.split_once("\n\n").unwrap();

    for line in id_ranges.lines() {
        let (from, to) = line.split_once('-').unwrap();
        let from = from.parse::<usize>().unwrap();
        let to = to.parse::<usize>().unwrap();
        fresh_ingridiants.insert((from, to));
    }

    let ingridiants = ingridiants
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    ingridiants
        .iter()
        .filter(|i| {
            let mut is_in_range = false;
            for (from, to) in &fresh_ingridiants {
                if *i >= from && *i <= to {
                    is_in_range = true;
                    break;
                }
            }
            is_in_range
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::part_01;

    #[test]
    fn test_01() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(3, part_01(input));
    }
}
