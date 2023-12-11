use rstest::rstest;

#[derive(Debug, Copy, Clone)]
enum Tiles {
    Vertical(Tile),
    Horizontal(Tile),
    North2East(Tile),
    North2West(Tile),
    South2West(Tile),
    South2East(Tile),
    Ground(Tile),
    Start(Tile),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    position: Point,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let input = std::fs::read_to_string("puzzle").expect("Unable to open file");
    println!("Part 01: {}", part_01(&input));
}

fn part_01(input: &str) -> u64 {
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

    let end_position = path_a
        .zip(path_b)
        .position(|(a, b)| a.1 == b.1)
        .expect("No path found");
    end_position as u64 + 1
}

fn crate_grid(input: &str) -> Vec<Vec<Tiles>> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, tile)| match tile {
                    '|' => Tiles::Vertical(Tile {
                        position: Point { x, y },
                    }),
                    '-' => Tiles::Horizontal(Tile {
                        position: Point { x, y },
                    }),
                    'L' => Tiles::North2East(Tile {
                        position: Point { x, y },
                    }),
                    'J' => Tiles::North2West(Tile {
                        position: Point { x, y },
                    }),
                    '7' => Tiles::South2West(Tile {
                        position: Point { x, y },
                    }),
                    'F' => Tiles::South2East(Tile {
                        position: Point { x, y },
                    }),
                    '.' => Tiles::Ground(Tile {
                        position: Point { x, y },
                    }),
                    'S' => Tiles::Start(Tile {
                        position: Point { x, y },
                    }),
                    _ => panic!("Invalid tile"),
                })
                .collect()
        })
        .collect()
}

fn peek_neighbors(grid: &[Vec<Tiles>], pos: Point) -> Vec<&Tiles> {
    let mut neighbors = Vec::new();

    if pos.x > 0 && pos.y > 0 {
        if let Some(tile) = grid.get(pos.y).and_then(|line| line.get(pos.x + 1)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y).and_then(|line| line.get(pos.x - 1)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y + 1).and_then(|line| line.get(pos.x)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y - 1).and_then(|line| line.get(pos.x)) {
            neighbors.push(tile);
        }
    } else if pos.x == 0 && pos.y > 0 {
        if let Some(tile) = grid.get(pos.y).and_then(|line| line.get(pos.x + 1)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y + 1).and_then(|line| line.get(pos.x)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y - 1).and_then(|line| line.get(pos.x)) {
            neighbors.push(tile);
        }
    } else if pos.x > 0 && pos.y == 0 {
        if let Some(tile) = grid.get(pos.y).and_then(|line| line.get(pos.x + 1)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y).and_then(|line| line.get(pos.x - 1)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y + 1).and_then(|line| line.get(pos.x)) {
            neighbors.push(tile);
        }
    } else if pos.x == 0 && pos.y == 0 {
        if let Some(tile) = grid.get(pos.y).and_then(|line| line.get(pos.x + 1)) {
            neighbors.push(tile);
        }
        if let Some(tile) = grid.get(pos.y + 1).and_then(|line| line.get(pos.x)) {
            neighbors.push(tile);
        }
    }
    neighbors
}

#[rstest]
#[case(
    ".....
.S-7.
.|.|.
.L-J.
.....",
    4
)]
#[case(
    "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    8
)]
fn two_simple_cases(#[case] input: &str, #[case] expected: u64) {
    assert_eq!(part_01(input), expected);
}
