fn main() {
    let input = std::fs::read_to_string("puzzle").expect("Unable to read file");
    println!("Part 01: {}", part_01(&input));
}

fn part_01(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (springs, condition_recipes) = line.split_once(' ').unwrap();
        })
        .collect::<Vec<_>>();
    todo!()
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
}
