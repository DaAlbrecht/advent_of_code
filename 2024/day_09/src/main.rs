use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum Position {
    Id(usize),
    Empty(usize),
}

#[derive(Debug, Clone, PartialEq)]
enum PositionType {
    File(usize),
    Empty(usize),
    FileWithId((usize, usize)),
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Position::Id(id) => write!(f, "{id}"),
            Position::Empty(_) => write!(f, "."),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part_01: {}", part_01(input.clone()));
    println!("part_02: {}", part_02(input));
}

fn part_01(mut input: String) -> usize {
    input = input.trim().to_string();
    if input.len() % 2 != 0 {
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
            disk_map_with_id.push(Position::Empty(*space as usize));
        }
    }
    return compress(disk_map_with_id)
        .iter()
        .enumerate()
        .map(|(position, file_id)| position * file_id)
        .sum();
}
#[allow(clippy::too_many_lines)]
fn part_02(mut input: String) -> usize {
    input = input.trim().to_string();
    if input.len() % 2 != 0 {
        input.push('0');
    }

    let disk_map = input
        .chars()
        .chunks(2)
        .into_iter()
        .flat_map(|mut chunk| {
            vec![
                PositionType::File(chunk.next().unwrap().to_digit(10).unwrap() as usize),
                PositionType::Empty(chunk.next().unwrap().to_digit(10).unwrap() as usize),
            ]
        })
        .collect::<Vec<PositionType>>();

    let disk_map = disk_map
        .iter()
        .enumerate()
        .map(|(id, pos_type)| match pos_type {
            PositionType::File(files) => PositionType::FileWithId((id / 2, *files)),
            _ => pos_type.clone(),
        })
        .collect::<Vec<PositionType>>();

    let mut next_file_pos = disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, pos)| match pos {
            PositionType::FileWithId(_) => Some(i),
            _ => None,
        })
        .next_back()
        .unwrap();

    let mut compact_disk = disk_map.clone();

    loop {
        if let PositionType::FileWithId((level, number_of_files)) =
            compact_disk[next_file_pos].clone()
        {
            if let Some((first_free_spot, free_space)) =
                compact_disk
                    .iter()
                    .enumerate()
                    .find_map(|(i, pos)| match pos {
                        PositionType::Empty(space) => {
                            if *space >= number_of_files {
                                return Some((i, *space));
                            }
                            None
                        }
                        _ => None,
                    })
            {
                if first_free_spot < next_file_pos {
                    let left_free_after_move = free_space - number_of_files;
                    compact_disk[first_free_spot] = PositionType::Empty(left_free_after_move);
                    compact_disk[next_file_pos] = PositionType::Empty(number_of_files);
                    compact_disk.insert(
                        first_free_spot,
                        PositionType::FileWithId((level, number_of_files)),
                    );
                }
            }

            match compact_disk
                .iter()
                .enumerate()
                .filter_map(|(i, pos)| match pos {
                    PositionType::FileWithId(_) => {
                        if i < next_file_pos {
                            return Some(i);
                        }
                        None
                    }
                    _ => None,
                })
                .next_back()
            {
                Some(next) => {
                    next_file_pos = next;
                    continue;
                }

                None => {
                    break;
                }
            }
        }
        panic!("should always be number");
    }

    let mut disk_map_with_id = Vec::new();

    for position_type in &compact_disk {
        match position_type {
            PositionType::FileWithId((id, i)) => {
                for _ in 0..*i {
                    disk_map_with_id.push(Position::Id(*id));
                }
            }
            PositionType::Empty(i) => {
                for _ in 0..*i {
                    disk_map_with_id.push(Position::Empty(*i));
                }
            }
            PositionType::File(_) => panic!("should not hit"),
        }
    }
    let mut sums = Vec::new();
    for (i, pos) in disk_map_with_id.iter().enumerate() {
        match pos {
            Position::Id(id) => {
                sums.push(id * i);
            }
            Position::Empty(_) => continue,
        }
    }

    sums.iter().sum()
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
                Position::Empty(_) => Some(i),
            })
    {
        if earliest_empty_slot >= next_slot {
            break;
        }

        while matches!(compact_disk[next_slot], Position::Empty(_)) {
            next_slot -= 1;
        }

        compact_disk.swap(next_slot, earliest_empty_slot);
    }

    compact_disk
        .iter()
        .filter_map(|position| match position {
            Position::Id(id) => Some(id),
            Position::Empty(_) => None,
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
    #[test]
    fn part_02() {
        let test_input = "2333133121414131402";
        assert_eq!(super::part_02(test_input.to_string()), 2858);
    }
}
