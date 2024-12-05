use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let mut ordering_rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let (page_ordering_rules, updates) = input.split_once("\n\n").unwrap();

    page_ordering_rules.lines().for_each(|line| {
        let (x, y) = line.split_once('|').unwrap();
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        match ordering_rules.get_mut(&x) {
            Some(numbers_to_print_after_x) => {
                numbers_to_print_after_x.push(y);
            }
            None => {
                ordering_rules.insert(x, vec![y]);
            }
        }
    });

    let updates = updates
        .lines()
        .filter_map(|line| {
            let update = line
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            if valid_update(&update, ordering_rules.clone()) {
                return Some(update);
            }
            None
        })
        .collect::<Vec<Vec<usize>>>();

    updates
        .iter()
        .filter_map(|update| update.get(update.len() / 2))
        .sum()
}

fn valid_update(update: &[usize], ordering_rules: HashMap<usize, Vec<usize>>) -> bool {
    update.iter().enumerate().all(|(x_index, x)| {
        if let Some(rules) = ordering_rules.get(x) {
            let idx = update
                .iter()
                .enumerate()
                .filter_map(|(i, y)| rules.contains(y).then_some(i))
                .collect::<Vec<usize>>();

            return idx.iter().all(|y_index| &x_index < y_index);
        }
        return true;
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(super::part_01(test_input), 143);
    }
}
