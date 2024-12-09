use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Position {
    Id(usize),
    Empty,
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Position::Id(id) => write!(f, "{id}"),
            Position::Empty => write!(f, "."),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input));
}

fn part_01(mut input: String) -> usize {
    //(;
    input = input.trim().to_string();
    if input.len() % 2 != 0 {
        println!("foo");
        input.push('0');
    }

    let disk_map = input
        .chars()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            (
                chunk.next().unwrap().to_digit(10).unwrap(),
                chunk.next().unwrap().to_digit(10).unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>();

    let mut disk_map_with_id = Vec::new();

    for (id, (files, space)) in disk_map.iter().enumerate() {
        for _ in 0..*files {
            disk_map_with_id.push(Position::Id(id));
        }
        for _ in 0..*space {
            disk_map_with_id.push(Position::Empty);
        }
    }

    compress(disk_map_with_id)
        .iter()
        .enumerate()
        .map(|(position, file_id)| position * file_id)
        .sum()
}

fn compress(files: Vec<Position>) -> Vec<usize> {
    let mut compact_disk = files;
    let mut next_slot = compact_disk.len() - 1;

    while let Some(earliest_empty_slot) =
        compact_disk
            .iter()
            .enumerate()
            .find_map(|(i, file)| match file {
                Position::Id(_) => None,
                Position::Empty => Some(i),
            })
    {
        if earliest_empty_slot >= next_slot {
            break;
        }

        while matches!(compact_disk[next_slot], Position::Empty) {
            next_slot -= 1;
        }

        compact_disk.swap(next_slot, earliest_empty_slot);
    }

    compact_disk
        .iter()
        .filter_map(|position| match position {
            Position::Id(id) => Some(id),
            Position::Empty => None,
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_01() {
        let test_input = "2333133121414131402";
        assert_eq!(super::part_01(test_input.to_string()), 1928);
    }
}
