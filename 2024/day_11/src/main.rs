use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.as_str()));
    println!("part_02: {}", part_02(input.as_str()));
}

fn part_01(input: &str) -> usize {
    process(input, 24)
}
fn part_02(input: &str) -> usize {
    process(input, 74)
}

fn process(input: &str, step: usize) -> usize {
    let stones = input
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut stone_map: HashMap<u64, u64> = HashMap::new();

    for stone in stones {
        stone_map
            .entry(stone)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    usize::try_from(
        std::iter::from_fn(|| {
            let mut temp_map: HashMap<u64, u64> = HashMap::new();
            stone_map.iter().for_each(|(stone, count)| match stone {
                0 => {
                    temp_map
                        .entry(1)
                        .and_modify(|v| {
                            *v += count;
                        })
                        .or_insert(*count);
                }
                i if (u64::ilog10(*i) + 1) % 2 == 0 => {
                    let temp = i.to_string();
                    let left = &temp[0..temp.len() / 2].parse::<u64>().unwrap();
                    let right = &temp[temp.len() / 2..temp.len()].parse::<u64>().unwrap();
                    temp_map
                        .entry(*left)
                        .and_modify(|v| *v += count)
                        .or_insert(*count);

                    temp_map
                        .entry(*right)
                        .and_modify(|v| *v += count)
                        .or_insert(*count);
                }
                _ => {
                    temp_map
                        .entry(stone * 2024)
                        .and_modify(|v| *v += count)
                        .or_insert(*count);
                }
            });
            stone_map.clone_from(&temp_map);
            Some(temp_map)
        })
        .nth(step)
        .unwrap()
        .values()
        .sum::<u64>(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "125 17";
        assert_eq!(super::part_01(test_input), 55312);
    }
    #[test]
    fn part_02() {
        let test_input = "125 17";
        assert_eq!(super::process(test_input, 24), 55312);
    }
}
