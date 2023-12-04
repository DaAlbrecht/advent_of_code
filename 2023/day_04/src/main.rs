use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("puzzle").expect("could not read input file");
    println!("Part 01: {}", part_01(&input));
    println!("Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(':').expect("not a valid game line");

            let (winning_numbers, my_numbers) =
                line.split_once('|').expect("not a valid game line");

            let winning_numbers: Vec<u32> = winning_numbers
                .split_whitespace()
                .map(|n| n.parse().expect("not a valid number"))
                .collect();

            let my_numbers: Vec<u32> = my_numbers
                .split_whitespace()
                .map(|n| n.parse().expect("not a valid number"))
                .collect();

            let wins = my_numbers
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .count();
            calculate_points(wins)
        })
        .sum()
}

fn calculate_points(wins: usize) -> u32 {
    match wins {
        0 => 0,
        _ => 2u32.pow(wins as u32 - 1),
    }
}

fn part_02(input: &str) -> u32 {
    let game = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, line) = line.split_once(':').expect("not a valid game line");

            let (winning_numbers, my_numbers) =
                line.split_once('|').expect("not a valid game line");

            let winning_numbers: Vec<u32> = winning_numbers
                .split_whitespace()
                .map(|n| n.parse().expect("not a valid number"))
                .collect();

            let my_numbers: Vec<u32> = my_numbers
                .split_whitespace()
                .map(|n| n.parse().expect("not a valid number"))
                .collect();
            (i + 1, my_numbers, winning_numbers)
        })
        .collect::<Vec<(usize, Vec<u32>, Vec<u32>)>>();

    let mut won_cards = vec![];
    for (round, my_numbers, winning_numbers) in game {
        count_wins(round, &my_numbers, &winning_numbers, &mut won_cards);
    }

    let occurrences_map: HashMap<_, _> = won_cards.iter().fold(HashMap::new(), |mut map, &num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    occurrences_map
        .iter()
        .map(|(_, &occurrences)| occurrences)
        .sum()
}

fn count_wins(round: usize, my_numbers: &[u32], winning_numbers: &[u32], won_cards: &mut Vec<u32>) {
    let wins = winning_numbers
        .iter()
        .filter(|n| my_numbers.contains(n))
        .count() as u32;

    let cards = (1..=wins).map(|n| n + round as u32).collect::<Vec<u32>>();

    for card in won_cards.clone() {
        if card == round as u32 {
            won_cards.extend(cards.clone());
        }
    }

    won_cards.extend(cards);
    won_cards.push(round as u32);
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
Card 7: 74 77 10 23 35 67 36 11 | 74 77 10 23 35 67 36 11";
        assert_eq!(super::part_01(input), 141);
    }

    #[test]
    fn part_02() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(super::part_02(input), 30);
    }
}
