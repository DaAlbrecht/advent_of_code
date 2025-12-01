fn main() {
    let file = std::fs::read_to_string("input").unwrap();
    dbg!(part_01(&file));
    dbg!(part_02(&file));
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

fn part_02(file: &str) -> usize {
    let mut dial: i32 = 50;
    let mut count_zero = 0;

    for line in file.lines() {
        let (direction, number) = line.split_at(1);
        let number = number.parse::<i32>().unwrap();

        match direction {
            "R" => {
                for i in 1..=number {
                    if (dial + i).rem_euclid(100) == 0 {
                        count_zero += 1;
                    }
                }
                dial = (dial + number).rem_euclid(100);
            }
            "L" => {
                for i in 1..=number {
                    if (dial - i).rem_euclid(100) == 0 {
                        count_zero += 1;
                    }
                }
                dial = (dial - number).rem_euclid(100);
            }
            _ => unreachable!(),
        }
    }

    count_zero
}

#[cfg(test)]
mod tests {
    use crate::{part_01, part_02};

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

    #[test]
    fn test_02() {
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
        assert_eq!(6, part_02(input));
    }
}
