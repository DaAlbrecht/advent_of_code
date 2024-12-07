use itertools::{repeat_n, Itertools};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    const OPERANDS: [char; 2] = ['*', '+'];
    process(input, &OPERANDS)
}

fn part_02(input: &str) -> usize {
    const OPERANDS: [char; 3] = ['*', '+', '|'];
    process(input, &OPERANDS)
}

fn process(input: &str, operators: &[char]) -> usize {
    let test_result = input
        .lines()
        .map(|line| {
            let (test_result, values) = line.split_once(':').unwrap();
            (
                test_result.parse::<usize>().unwrap(),
                values
                    .split_whitespace()
                    .map(|value| value.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(usize, Vec<usize>)>>();

    test_result
        .iter()
        .filter_map(|test| {
            let operator_combinations =
                repeat_n(operators.iter(), test.1.len() - 1).multi_cartesian_product();
            for operators in operator_combinations {
                if evaluate_expression(&test.1, &operators) == test.0 {
                    return Some(test.0);
                }
            }
            None::<usize>
        })
        .sum()
}

fn evaluate_expression(numbers: &[usize], operators: &[&char]) -> usize {
    let mut result = numbers[0];
    for (i, &operator) in operators.iter().enumerate() {
        match operator {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => {
                result = format!("{}{}", result, numbers[i + 1])
                    .parse::<usize>()
                    .unwrap();
            }
            _ => panic!("Invalid operator"),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(super::part_01(test_input), 3749);
    }
    #[test]
    fn part_02() {
        let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(super::part_02(test_input), 113_837_497);
    }
}
