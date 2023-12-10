use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid instruction"),
        }
    }
}

#[derive(Debug, Clone)]
struct Node<'a> {
    val: &'a str,
    left: &'a str,
    right: &'a str,
}

struct Cursor<'a> {
    instructions: Vec<Instruction>,
    current_instruction: usize,
    current_node: usize,
    nodes: Vec<Node<'a>>,
}

struct GhostCursor<'a> {
    instructions: Vec<Instruction>,
    current_instruction: usize,
    current_node: usize,
    nodes: Vec<Node<'a>>,
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nodes[self.current_node].val == "ZZZ" {
            return None;
        }

        match self.instructions[self.current_instruction] {
            Instruction::Left => {
                if self.current_instruction == self.instructions.len() - 1 {
                    self.current_instruction = 0;
                } else {
                    self.current_instruction += 1;
                }

                let next_node = self
                    .nodes
                    .par_iter()
                    .find_first(|node| node.val == self.nodes[self.current_node].left)
                    .cloned();

                self.current_node = self
                    .nodes
                    .par_iter()
                    .position_first(|node| node.val == self.nodes[self.current_node].left)
                    .unwrap();

                next_node
            }
            Instruction::Right => {
                if self.current_instruction == self.instructions.len() - 1 {
                    self.current_instruction = 0;
                } else {
                    self.current_instruction += 1;
                }
                let next_node = self
                    .nodes
                    .par_iter()
                    .find_first(|node| node.val == self.nodes[self.current_node].right)
                    .cloned();
                self.current_node = self
                    .nodes
                    .par_iter()
                    .position_first(|node| node.val == self.nodes[self.current_node].right)
                    .unwrap();
                next_node
            }
        }
    }
}

impl<'a> Iterator for GhostCursor<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nodes[self.current_node].val.ends_with('Z') {
            return None;
        }

        match self.instructions[self.current_instruction] {
            Instruction::Left => {
                if self.current_instruction == self.instructions.len() - 1 {
                    self.current_instruction = 0;
                } else {
                    self.current_instruction += 1;
                }

                let next_node = self
                    .nodes
                    .par_iter()
                    .find_first(|node| node.val == self.nodes[self.current_node].left)
                    .cloned();

                self.current_node = self
                    .nodes
                    .par_iter()
                    .position_first(|node| node.val == self.nodes[self.current_node].left)
                    .unwrap();

                next_node
            }
            Instruction::Right => {
                if self.current_instruction == self.instructions.len() - 1 {
                    self.current_instruction = 0;
                } else {
                    self.current_instruction += 1;
                }
                let next_node = self
                    .nodes
                    .par_iter()
                    .find_first(|node| node.val == self.nodes[self.current_node].right)
                    .cloned();
                self.current_node = self
                    .nodes
                    .par_iter()
                    .position_first(|node| node.val == self.nodes[self.current_node].right)
                    .unwrap();
                next_node
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle").expect("Unable to read file");
    println!("Part 01: {}", part_01(&input));
    println!("Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> i32 {
    let (instructions, map) = input
        .split_once("\n\n")
        .expect("invalid input, no new line found after instructions");

    let instructions = instructions
        .chars()
        .map(Instruction::from)
        .collect::<Vec<_>>();

    let map = map
        .lines()
        .map(|line| {
            let (id, nodes) = line.split_once('=').expect("invalid input, no = found");
            let id = id.trim();
            let (left, right) = nodes.split_once(',').expect("invalid input, no , found");
            let left: String = left.trim().chars().skip(1).collect();
            let right: String = right.trim().chars().take_while(|c| *c != ')').collect();
            (id, left, right)
        })
        .collect::<Vec<_>>();

    let nodes = map
        .par_iter()
        .map(|(id, left, right)| Node {
            val: id,
            left,
            right,
        })
        .collect::<Vec<_>>();

    let starting_node_pos = nodes
        .par_iter()
        .position_first(|node| node.val == "AAA")
        .unwrap();

    let cursor = Cursor {
        instructions,
        current_instruction: 0,
        current_node: starting_node_pos,
        nodes,
    };

    cursor.count() as i32
}

fn part_02(input: &str) -> i64 {
    let (instructions, map) = input
        .split_once("\n\n")
        .expect("invalid input, no new line found after instructions");

    let instructions = instructions
        .chars()
        .map(Instruction::from)
        .collect::<Vec<_>>();

    let map = map
        .lines()
        .map(|line| {
            let (id, nodes) = line.split_once('=').expect("invalid input, no = found");
            let id = id.trim();
            let (left, right) = nodes.split_once(',').expect("invalid input, no , found");
            let left: String = left.trim().chars().skip(1).collect();
            let right: String = right.trim().chars().take_while(|c| *c != ')').collect();
            (id, left, right)
        })
        .collect::<Vec<_>>();

    let nodes = map
        .par_iter()
        .map(|(id, left, right)| Node {
            val: id,
            left,
            right,
        })
        .collect::<Vec<_>>();

    let starting_nodes_pos = nodes
        .par_iter()
        .positions(|node| node.val.ends_with('A'))
        .collect::<Vec<_>>();

    //count the cycles for each starting node to reach a node that ends with Z
    let cycles = starting_nodes_pos
        .par_iter()
        .map(|&starting_node_pos| {
            let cursor = GhostCursor {
                instructions: instructions.clone(),
                current_instruction: 0,
                current_node: starting_node_pos,
                nodes: nodes.clone(),
            };

            cursor.count() as i64
        })
        .collect::<Vec<_>>();

    lcm(&cycles)
}

fn lcm(nums: &[i64]) -> i64 {
    let mut result = nums[0];
    for &num in &nums[1..] {
        result = result * num / gcd(result, num);
    }
    result
}

fn gcd(result: i64, num: i64) -> i64 {
    if num == 0 {
        result
    } else {
        gcd(num, result % num)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = std::fs::read_to_string("test").expect("Unable to read file");
        assert_eq!(crate::part_01(&input), 6);
    }

    #[test]
    fn part_02() {
        let input = std::fs::read_to_string("test").expect("Unable to read file");
        assert_eq!(crate::part_02(&input), 6);
    }
}
