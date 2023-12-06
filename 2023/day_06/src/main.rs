fn main() {
    let input = std::fs::read_to_string("puzzle").expect("could not read input file");
    println!("part 01: {}", part_01(&input));
    println!("part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u64 {
    let games = input
        .lines()
        .next()
        .expect("expect puzzle input to have two lines")
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .zip(
            input
                .lines()
                .nth(1)
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap()),
        )
        .collect::<Vec<(u64, u64)>>();

    let winning_moves = games
        .iter()
        .map(|x| winning_moves(*x))
        .collect::<Vec<u64>>();

    let mut total = 1;
    for i in winning_moves {
        total *= i;
    }
    total
}

fn part_02(input: &str) -> u64 {
    let time = input
        .lines()
        .next()
        .expect("expect puzzle input to have two lines")
        .strip_prefix("Time:")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = input
        .lines()
        .nth(1)
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    winning_moves((time, distance))
}

fn winning_moves(game: (u64, u64)) -> u64 {
    let time = game.0;
    let distance = game.1;
    let mut wins = 0;
    for i in 1..time {
        let possible_distance = i * (time - i);
        if possible_distance > distance {
            wins += 1;
        }
    }
    wins
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(crate::part_01(&input), 288);
    }
    #[test]
    fn part_02() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(crate::part_02(&input), 71503);
    }
}
