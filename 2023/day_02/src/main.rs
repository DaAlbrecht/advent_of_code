use std::fs;

#[derive(Debug, Clone)]
struct SubGame {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Clone)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

struct Inventory {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = fs::read_to_string("puzzle").expect("file not found");
    let result = part_01(&input);
    println!("Part 01: {}", result);
    let result = part_02(&input);
    println!("Part 02: {}", result);
}

fn part_01(input: &str) -> u32 {
    let inventory = Inventory {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = input
        .lines()
        .map(|game| parse_game(game))
        .collect::<Vec<Vec<SubGame>>>();
    possible_games(games, inventory)
}

fn part_02(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let game = parse_game(game);
            let min_inventory = min_cubes_needed(game);
            min_inventory.green * min_inventory.red * min_inventory.blue
        })
        .sum::<u32>()
}

fn min_cubes_needed(game: Vec<SubGame>) -> Inventory {
    let max_red = game.iter().map(|game| game.red).max().unwrap();
    let max_green = game.iter().map(|game| game.green).max().unwrap();
    let max_blue = game.iter().map(|game| game.blue).max().unwrap();
    Inventory {
        red: max_red,
        green: max_green,
        blue: max_blue,
    }
}

fn parse_game(game: &str) -> Vec<SubGame> {
    let (round, rest) = game.split_once(":").expect("Invalid game");
    let (_, id) = round.split_once(" ").expect("no id found");
    let id = id.parse::<u32>().ok().expect("id not a number");
    rest.split(";")
        .filter_map(|game| parse_subgame(game, id))
        .collect::<Vec<SubGame>>()
}

fn possible_games(games: Vec<Vec<SubGame>>, inventory: Inventory) -> u32 {
    let mut possible_games = Vec::new();
    for game in games {
        let mut possible = vec![false; game.len()];
        for (i, subgame) in game.iter().enumerate() {
            if subgame.red <= inventory.red
                && subgame.green <= inventory.green
                && subgame.blue <= inventory.blue
            {
                possible[i] = true;
            }
        }

        if possible.iter().all(|p| *p) {
            possible_games.push(game[0].id);
        }
    }
    possible_games.iter().map(|id| id).sum::<u32>()
}

fn parse_subgame(subgame: &str, id: u32) -> Option<SubGame> {
    let color = subgame
        .split(",")
        .map(|color| {
            let (counter, color) = color.trim().split_once(" ")?;
            let counter = counter.parse::<u32>().ok()?;
            match color {
                "red" => Some(Color::Red(counter)),
                "green" => Some(Color::Green(counter)),
                "blue" => Some(Color::Blue(counter)),
                _ => return None,
            }
        })
        .collect::<Vec<Option<Color>>>();
    let blue = color
        .iter()
        .filter_map(|color| match color {
            Some(Color::Blue(counter)) => Some(counter),
            _ => None,
        })
        .sum::<u32>();
    let red = color
        .iter()
        .filter_map(|color| match color {
            Some(Color::Red(counter)) => Some(counter),
            _ => None,
        })
        .sum::<u32>();
    let green = color
        .iter()
        .filter_map(|color| match color {
            Some(Color::Green(counter)) => Some(counter),
            _ => None,
        })
        .sum::<u32>();
    Some(SubGame {
        id,
        red,
        green,
        blue,
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(super::part_01(input), 8);
    }
    #[test]
    fn part_02() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(super::part_02(input), 2286);
    }
}
