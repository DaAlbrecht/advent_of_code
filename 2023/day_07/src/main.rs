#[derive(Debug, PartialEq, Eq, Hash)]
enum Hands {
    FiveOfAKind(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    TwoPairs(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl From<Vec<Card>> for Hands {
    fn from(value: Vec<Card>) -> Self {
        let mut counts = std::collections::HashMap::new();
        for card in value.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts = counts.into_iter().collect::<Vec<_>>();

        if value.contains(&Card::Joker) {
            let jokers = value.iter().filter(|c| **c == Card::Joker).count();

            if jokers == 5 {
                return Hands::FiveOfAKind(value);
            }

            counts.retain(|(card, _)| **card != Card::Joker);

            counts.sort_by(|a, b| b.1.cmp(&a.1));
            counts[0].1 += jokers;
        }
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        match counts[0].1 {
            5 => Hands::FiveOfAKind(value),
            4 => Hands::FourOfAKind(value),
            3 => {
                if counts[1].1 == 2 {
                    Hands::FullHouse(value)
                } else {
                    Hands::ThreeOfAKind(value)
                }
            }
            2 => {
                if counts[1].1 == 2 {
                    Hands::TwoPairs(value)
                } else {
                    Hands::OnePair(value)
                }
            }
            1 => Hands::HighCard(value),
            _ => {
                panic!("invalid Hand")
            }
        }
    }
}

impl PartialOrd for Hands {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hands {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Hands::FiveOfAKind(self_hand), Hands::FiveOfAKind(other_hand)) => {
                for (self_card, other_card) in self_hand.iter().zip(other_hand.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hands::FiveOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, Hands::FiveOfAKind(_)) => std::cmp::Ordering::Less,
            (Hands::FourOfAKind(self_hand), Hands::FourOfAKind(other_hand)) => {
                for (self_card, other_card) in self_hand.iter().zip(other_hand.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hands::FourOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, Hands::FourOfAKind(_)) => std::cmp::Ordering::Less,
            (Hands::FullHouse(self_hand), Hands::FullHouse(other_hand)) => {
                for (self_card, other_card) in self_hand.iter().zip(other_hand.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hands::FullHouse(_), _) => std::cmp::Ordering::Greater,
            (_, Hands::FullHouse(_)) => std::cmp::Ordering::Less,
            (Hands::ThreeOfAKind(self_hand), Hands::ThreeOfAKind(other_hand)) => {
                for (self_card, other_card) in self_hand.iter().zip(other_hand.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hands::ThreeOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, Hands::ThreeOfAKind(_)) => std::cmp::Ordering::Less,
            (Hands::TwoPairs(self_hand), Hands::TwoPairs(other_hand)) => {
                for (self_card, other_card) in self_hand.iter().zip(other_hand.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hands::TwoPairs(_), _) => std::cmp::Ordering::Greater,
            (_, Hands::TwoPairs(_)) => std::cmp::Ordering::Less,
            (Hands::OnePair(self_hand), Hands::OnePair(other_hand)) => {
                for (self_card, other_card) in self_hand.iter().zip(other_hand.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hands::OnePair(_), _) => std::cmp::Ordering::Greater,
            (_, Hands::OnePair(_)) => std::cmp::Ordering::Less,
            (Hands::HighCard(self_hand), Hands::HighCard(other_hand)) => {
                for (self_card, other_card) in self_hand.iter().zip(other_hand.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("invalid card"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle").expect("file not found");
    println!("Part 01: {}", part_01(&input));
    println!("Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u64 {
    let mut games = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").expect("invalid input");
            let bid = bid.parse::<u64>().expect("invalid input");
            let cards = cards.chars().map(|c| c.into()).collect::<Vec<_>>();
            let hands: Hands = cards.into();
            (hands, bid)
        })
        .collect::<Vec<(Hands, u64)>>();

    games.sort_by(|a, b| a.0.cmp(&b.0));
    games
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum::<u64>()
}

fn part_02(input: &str) -> u64 {
    let mut games = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").expect("invalid input");
            let bid = bid.parse::<u64>().expect("invalid input");
            let cards = cards.chars().map(|c| c.into()).collect::<Vec<Card>>();
            let cards = cards
                .iter()
                .map(|c| if *c == Card::J { Card::Joker } else { *c })
                .collect::<Vec<_>>();
            let hands: Hands = cards.into();
            (hands, bid)
        })
        .collect::<Vec<(Hands, u64)>>();

    games.sort_by(|a, b| a.0.cmp(&b.0));
    games
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1) * bid)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = std::fs::read_to_string("test").unwrap();
        assert_eq!(super::part_01(&input), 6440);
    }

    #[test]
    fn part_02() {
        let input = std::fs::read_to_string("test").unwrap();
        assert_eq!(super::part_02(&input), 5905);
    }
}
