fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_01: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(r, l)| {
                (
                    r.trim().parse::<usize>().unwrap(),
                    l.trim().parse::<usize>().unwrap(),
                )
            })
        })
        .unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part_02(input: &str) -> usize {
    let (left, right): (Vec<usize>, Vec<usize>) = input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(r, l)| {
                (
                    r.trim().parse::<usize>().unwrap(),
                    l.trim().parse::<usize>().unwrap(),
                )
            })
        })
        .unzip();

    left.iter()
        .map(|entry| {
            let occurrence = right.iter().filter(|r| *r == entry).count();
            entry * occurrence
        })
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let test_input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(super::part_01(test_input), 11);
    }

    #[test]
    fn part_02() {
        let test_input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(super::part_02(test_input), 31);
    }
}
