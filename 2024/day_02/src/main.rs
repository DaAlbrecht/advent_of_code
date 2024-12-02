use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_01: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    let reports = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|c| c.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    reports.iter().filter(|report| is_safe(report)).count()
}

fn part_02(input: &str) -> usize {
    let reports = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|c| c.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    reports
        .iter()
        .filter(|report| is_safe_with_damper(report.to_vec()))
        .count()
}

fn is_safe(report: &[usize]) -> bool {
    let mut increasing: Option<bool> = None;
    report.iter().tuple_windows().all(|(first, second)| {
        if first.abs_diff(*second) > 3 || first == second {
            return false;
        }

        match increasing {
            Some(v) => match v {
                true => first < second,
                false => first > second,
            },
            None => {
                if first > second {
                    increasing = Some(false);
                } else {
                    increasing = Some(true);
                }
                true
            }
        }
    })
}

fn is_safe_with_damper(mut report: Vec<usize>) -> bool {
    if report.len() < 3 {
        return true;
    }
    let last = report.len() - 1;
    for i in (0..last).rev() {
        if is_safe(&report[..last]) {
            return true;
        }
        report.swap(i, last);
    }
    //last swap
    is_safe(&report[..last])
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(super::part_01(test_input), 2);
    }

    #[test]
    fn part_02() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(super::part_02(test_input), 4);
    }
}
