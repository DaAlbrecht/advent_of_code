use std::collections::HashSet;

use crate::{crate_grid, peek_neighbors, Direction, Point, Tiles};

enum Status {
    In,
    Out,
}

pub fn part_02(input: &str) -> u64 {
    let grid = crate_grid(input);

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, tile)| match tile {
                Tiles::Start(_) => Some(Point { x, y }),
                _ => None,
            })
        })
        .unwrap();

    //algorithm inspired by chris biscardi https://www.youtube.com/watch?v=_-QXvb8GJlg
    let north_position = peek_neighbors(&grid, start)
        .iter()
        .find_map(|tile| match tile {
            Tiles::Vertical(_) | Tiles::South2West(_) | Tiles::South2East(_) => match tile {
                Tiles::Vertical(tile) | Tiles::South2West(tile) | Tiles::South2East(tile) => {
                    if tile.position.y < start.y {
                        Some((Direction::South, tile.position))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        });

    let south_position = peek_neighbors(&grid, start)
        .iter()
        .find_map(|tile| match tile {
            Tiles::Vertical(_) | Tiles::North2West(_) | Tiles::North2East(_) => match tile {
                Tiles::Vertical(tile) | Tiles::North2West(tile) | Tiles::North2East(tile) => {
                    if tile.position.y > start.y {
                        Some((Direction::North, tile.position))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        });

    let east_position = peek_neighbors(&grid, start)
        .iter()
        .find_map(|tile| match tile {
            Tiles::Horizontal(_) | Tiles::North2West(_) | Tiles::South2West(_) => match tile {
                Tiles::Horizontal(tile) | Tiles::North2West(tile) | Tiles::South2West(tile) => {
                    if tile.position.x > start.x {
                        Some((Direction::West, tile.position))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        });

    let west_position = peek_neighbors(&grid, start)
        .iter()
        .find_map(|tile| match tile {
            Tiles::Horizontal(_) | Tiles::North2East(_) | Tiles::South2East(_) => match tile {
                Tiles::Horizontal(tile) | Tiles::North2East(tile) | Tiles::South2East(tile) => {
                    if tile.position.x < start.x {
                        Some((Direction::East, tile.position))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        });

    let mut iters =
        vec![north_position, south_position, east_position, west_position]
            .into_iter()
            .flatten()
            .map(|tuple| {
                std::iter::successors(Some(tuple), |(from_direction, position)| {
                    let tiles_type = grid[position.y][position.x];

                    let next_direction = match (from_direction, tiles_type) {
                        (Direction::North, Tiles::Vertical(_)) => Direction::South,
                        (Direction::North, Tiles::North2East(_)) => Direction::East,
                        (Direction::North, Tiles::North2West(_)) => Direction::West,
                        (Direction::South, Tiles::Vertical(_)) => Direction::North,
                        (Direction::South, Tiles::South2East(_)) => Direction::East,
                        (Direction::South, Tiles::South2West(_)) => Direction::West,
                        (Direction::East, Tiles::Horizontal(_)) => Direction::West,
                        (Direction::East, Tiles::North2East(_)) => Direction::North,
                        (Direction::East, Tiles::South2East(_)) => Direction::South,
                        (Direction::West, Tiles::Horizontal(_)) => Direction::East,
                        (Direction::West, Tiles::North2West(_)) => Direction::North,
                        (Direction::West, Tiles::South2West(_)) => Direction::South,
                        _ => panic!("Invalid direction"),
                    };

                    let next =
                        match next_direction {
                            Direction::North => peek_neighbors(&grid, *position).iter().find_map(
                                |tile| match tile {
                                    Tiles::Vertical(tile)
                                    | Tiles::South2East(tile)
                                    | Tiles::South2West(tile) => {
                                        if tile.position.y < position.y {
                                            Some((Direction::South, tile.position))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                },
                            ),
                            Direction::South => peek_neighbors(&grid, *position).iter().find_map(
                                |tile| match tile {
                                    Tiles::Vertical(tile)
                                    | Tiles::North2East(tile)
                                    | Tiles::North2West(tile) => {
                                        if tile.position.y > position.y {
                                            Some((Direction::North, tile.position))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                },
                            ),
                            Direction::East => peek_neighbors(&grid, *position).iter().find_map(
                                |tile| match tile {
                                    Tiles::Horizontal(tile)
                                    | Tiles::North2West(tile)
                                    | Tiles::South2West(tile) => {
                                        if tile.position.x > position.x {
                                            Some((Direction::West, tile.position))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                },
                            ),
                            Direction::West => peek_neighbors(&grid, *position).iter().find_map(
                                |tile| match tile {
                                    Tiles::Horizontal(tile)
                                    | Tiles::North2East(tile)
                                    | Tiles::South2East(tile) => {
                                        if tile.position.x < position.x {
                                            Some((Direction::East, tile.position))
                                        } else {
                                            None
                                        }
                                    }
                                    _ => None,
                                },
                            ),
                        };
                    next
                })
            });

    let path_a = iters.next().expect("No path found");
    let path_b = iters.next().expect("No path found");

    let mut combined_path_iter = path_a.zip(path_b);

    let mut tiles_location = HashSet::new();
    tiles_location.insert(start);
    while let Some((path_a_point, path_b_point)) = combined_path_iter.next() {
        tiles_location.insert(path_a_point.1);
        tiles_location.insert(path_b_point.1);

        if path_a_point.1 == path_b_point.1 {
            //all tiles are visited
            break;
        }
    }

    let result = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut status = Status::Out;
            line.chars()
                .enumerate()
                .filter(|(x, _)| {
                    let point = Point { x: *x, y };
                    let tiles_type = grid[point.y][point.x];
                    if tiles_location.contains(&point) {
                        let crossed = match tiles_type {
                            Tiles::Start(_)
                            | Tiles::Vertical(_)
                            | Tiles::South2East(_)
                            | Tiles::South2West(_) => true,
                            _ => false,
                        };

                        if crossed {
                            status = match status {
                                Status::In => Status::Out,
                                Status::Out => Status::In,
                            };
                        }
                        false
                    } else {
                        match status {
                            Status::In => true,
                            Status::Out => false,
                        }
                    }
                })
                .count()
        })
        .sum::<usize>();
    result as u64
}
