use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle").expect("Unable to read file");
    println!("day 1 part 1 {}", part_01(&input));
    println!("day 1 part 2 {}", part_02(&input));
}

fn part_01(input: &String) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|numbers| numbers[0] * 10 + numbers[numbers.len() - 1])
        .sum()
}

fn part_02(input: &String) -> u32 {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "on1e")
                .replace("two", "tw2o")
                .replace("three", "thr3ee")
                .replace("four", "fou4r")
                .replace("five", "fiv5e")
                .replace("six", "si6x")
                .replace("seven", "se7ven")
                .replace("eight", "eigh8t")
                .replace("nine", "ni9ne");
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|numbers| numbers[0] * 10 + numbers[numbers.len() - 1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01() {
        let input = String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(part_01(&input), 142);
        let input = String::from("2abc222222222\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(part_01(&input), 152);
    }

    #[test]
    fn test_day_02() {
        let input = String::from("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        assert_eq!(part_02(&input), 281);
    }
}
