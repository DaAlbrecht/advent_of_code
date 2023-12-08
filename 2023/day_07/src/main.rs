#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hands {
    FiveOfAKind(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    TwoPairs(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
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

impl From<Vec<Card>> for Hands {
    fn from(cards: Vec<Card>) -> Self {
        let mut cards = cards;
        cards.sort();
        let mut counts = std::collections::HashMap::new();
        for card in cards.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        let cards = counts.iter().map(|(card, _)| **card).collect::<Vec<_>>();
        match counts[0].1 {
            5 => Hands::FiveOfAKind(cards),
            4 => Hands::FourOfAKind(cards),
            3 => {
                if counts[1].1 == 2 {
                    Hands::FullHouse(cards)
                } else {
                    Hands::ThreeOfAKind(cards)
                }
            }
            2 => {
                if counts[1].1 == 2 {
                    Hands::TwoPairs(cards)
                } else {
                    Hands::OnePair(cards)
                }
            }
            1 => Hands::HighCard(cards),
            _ => panic!("invalid card"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle").expect("file not found");
    println!("Part 01: {}", part_01(&input));
}

fn part_01(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").expect("invalid input");
            let bid = bid.parse::<u64>().expect("invalid input");
            let cards = cards
                .chars()
                .map(|card| Card::from(card))
                .collect::<Vec<Card>>();
            (Hands::from(cards), bid)
        })
        .count() as u64;

    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = std::fs::read_to_string("test").unwrap();
        assert_eq!(super::part_01(&input), 6440);
    }
}
