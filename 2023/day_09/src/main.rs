fn main() {
    let input = std::fs::read_to_string("puzzle").expect("Unable to read file");
    println!("Part 01: {}", part_01(&input));
    println!("Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> i64 {
    let ranges = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|range| range.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut new_elements = Vec::new();
    for range in ranges.iter() {
        let predictions = predict_next(range, &mut Vec::new());
        let new_element = range.last().unwrap() + predictions.last().unwrap();
        new_elements.push(new_element);
    }

    new_elements.iter().sum()
}

fn part_02(input: &str) -> i64 {
    let ranges = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|range| range.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut new_elements = Vec::new();
    for range in ranges.iter() {
        let range_rev = range.iter().rev().cloned().collect::<Vec<_>>();
        let predictions = predict_next(&range_rev, &mut Vec::new());
        let new_element = range_rev.last().unwrap() + predictions.last().unwrap();
        new_elements.push(new_element);
    }

    new_elements.iter().sum()
}

fn predict_next(range: &[i64], differences: &mut Vec<i64>) -> Vec<i64> {
    if range.iter().all(|x| *x == 0) {
        return differences.clone();
    }
    let mut sub_range = Vec::new();
    for i in 0..range.len() - 1 {
        sub_range.push(range[i + 1] - range[i]);
    }
    if differences.is_empty() {
        differences.push(*sub_range.last().unwrap());
    } else {
        differences.push(sub_range.last().unwrap() + differences.last().unwrap());
    }
    predict_next(&sub_range, differences)
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = std::fs::read_to_string("test").expect("Unable to read file");
        assert_eq!(crate::part_01(&input), 114);
    }

    #[test]
    fn part_02() {
        let input = std::fs::read_to_string("test").expect("Unable to read file");
        assert_eq!(crate::part_02(&input), 2);
    }
}
