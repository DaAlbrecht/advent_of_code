use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy)]
enum Condition {
    Working,
    Broken,
    Unknown,
}

fn main() {
    let input = std::fs::read_to_string("puzzle").expect("Unable to read file");
    //println!("Part 01: {}", part_01(&input));
    println!("Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, condition_recipes) = line.split_once(' ').unwrap();
            let springs = springs
                .chars()
                .map(|c| match c {
                    '?' => Condition::Unknown,
                    '.' => Condition::Broken,
                    '#' => Condition::Working,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<_>>();

            let unknown_count = springs
                .iter()
                .filter(|c| match c {
                    Condition::Unknown => true,
                    _ => false,
                })
                .count();

            let condition_recipes = condition_recipes
                .split(',')
                .map(|c| c.parse::<u64>().expect("Unable to parse condition recipe"))
                .collect::<Vec<_>>();

            let possible_springs = generate_all_possible_springs(&unknown_count);

            possible_springs
                .into_iter()
                .filter(|s| check_possibility(&springs, &condition_recipes, s))
                .count()
        })
        .sum::<usize>() as u64
}

fn part_02(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, condition_recipes) = line.split_once(' ').unwrap();
            let mut springs = springs
                .chars()
                .map(|c| match c {
                    '?' => Condition::Unknown,
                    '.' => Condition::Broken,
                    '#' => Condition::Working,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<_>>();

            for _ in 0..5 {
                springs.push(Condition::Unknown);
                for condition in springs.clone() {
                    springs.push(condition);
                }
            }

            let unknown_count = springs
                .iter()
                .filter(|c| match c {
                    Condition::Unknown => true,
                    _ => false,
                })
                .count();

            let mut condition_recipes = condition_recipes
                .split(',')
                .map(|c| c.parse::<u64>().expect("Unable to parse condition recipe"))
                .collect::<Vec<_>>();

            for _ in 0..5 {
                for condition in condition_recipes.clone() {
                    condition_recipes.push(condition);
                }
            }

            let possible_springs = generate_all_possible_springs(&unknown_count);
            possible_springs
                .into_iter()
                .filter(|s| check_possibility(&springs, &condition_recipes, &s))
                .count()
        })
        .sum::<usize>() as u64
}

fn generate_all_possible_springs(unknown_count: &usize) -> impl Iterator<Item = String> {
    repeat_n([".", "#"].into_iter(), *unknown_count)
        .multi_cartesian_product()
        .map(|v| v.into_iter().join(""))
}

fn check_possibility(
    input: &[Condition],
    condition_recipes: &[u64],
    possible_springs: &str,
) -> bool {
    let mut possible_iter = possible_springs.chars();
    let filled_springs = input
        .iter()
        .map(|c| match c {
            Condition::Unknown => possible_iter.next().unwrap(),
            Condition::Broken => '.',
            Condition::Working => '#',
        })
        .collect::<String>();

    let replaced_count = filled_springs
        .chars()
        .group_by(|c| c == &'#')
        .into_iter()
        .filter_map(|(springs, group)| springs.then_some(group.into_iter().count() as u64))
        .collect::<Vec<_>>();

    replaced_count == condition_recipes
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(crate::part_01(&input), 21);
    }

    #[test]
    fn part_02() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(crate::part_02(&input), 21);
    }
}
