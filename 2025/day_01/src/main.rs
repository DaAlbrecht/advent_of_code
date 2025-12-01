fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
}

fn part_01(file: &str) -> usize {
    let mut dial = 50;
    let mut count_zero = 0;

    for line in file.lines() {
        let (direction, number) = line.split_at(1);
        let number = number.parse::<i16>().unwrap();
        match direction {
            "R" => dial = (dial + number).rem_euclid(100),
            "L" => dial = (dial - number).rem_euclid(100),
            _ => unreachable!(),
        }
        if dial == 0 {
            count_zero += 1;
        }
    }
    count_zero
}

#[cfg(test)]
mod tests {
    use crate::part_01;

    #[test]
    fn test_01() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(3, part_01(input));
    }
}
