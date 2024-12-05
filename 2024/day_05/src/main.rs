use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
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

    let valid_updates = updates
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

    valid_updates
        .iter()
        .filter_map(|update| update.get(update.len() / 2))
        .sum()
}

fn part_02(input: &str) -> usize {
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

    let invalid_updates = updates
        .lines()
        .filter_map(|line| {
            let update = line
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            if !valid_update(&update, ordering_rules.clone()) {
                return Some(update);
            }
            None
        })
        .collect::<Vec<Vec<usize>>>();
    let valid_updates = invalid_updates
        .iter()
        .map(|invalid_update| order_update(&invalid_update, ordering_rules.clone()))
        .collect::<Vec<Vec<usize>>>();

    valid_updates
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

fn order_update(update: &[usize], ordering_rules: HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    update.iter().for_each(|&x| {
        if let Some(rules) = ordering_rules.get(&x) {
            let filtered_rules = rules
                .iter()
                .filter(|&&y| update.contains(&y))
                .cloned()
                .collect::<Vec<usize>>();
            graph.insert(x, filtered_rules);
        }
    });

    let mut visited = HashMap::new();
    let mut stack = Vec::new();
    for &node in update {
        if !visited.get(&node).copied().unwrap_or(false) {
            topological_sort(node, &graph, &mut visited, &mut stack);
        }
    }

    stack.reverse();
    stack
}

fn topological_sort(
    node: usize,
    graph: &HashMap<usize, Vec<usize>>,
    visited: &mut HashMap<usize, bool>,
    stack: &mut Vec<usize>,
) {
    visited.insert(node, true);
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.get(&neighbor).copied().unwrap_or(false) {
                topological_sort(neighbor, graph, visited, stack);
            }
        }
    }
    stack.push(node);
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

    #[test]
    fn part_02() {
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
        assert_eq!(super::part_02(test_input), 123);
    }
}
