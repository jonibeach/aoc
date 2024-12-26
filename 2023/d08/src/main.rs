use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    label: [char; 3],
    left: [char; 3],
    right: [char; 3],
}

impl Node {
    fn str_to_chars_3(input: &str) -> [char; 3] {
        let mut output = [' '; 3];
        for (i, char) in input.char_indices() {
            if i <= 2 {
                output[i] = char;
            }
        }
        output
    }

    fn from_str(input: &str) -> Option<Self> {
        let trimmed = input.replace(" ", "").replace("(", "").replace(")", "");
        if let Some((label, rest)) = trimmed.split_once("=") {
            if let Some((left, right)) = rest.split_once(",") {
                let label = Self::str_to_chars_3(label);
                let left = Self::str_to_chars_3(left);
                let right = Self::str_to_chars_3(right);
                return Some(Self { label, left, right });
            }
        }
        None
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(input: char) -> Option<Self> {
        match input {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    first_node: Node,
    nodes: HashMap<[char; 3], Node>,
}

impl Map {
    fn from_str(input: &str) -> Option<Self> {
        let mut lines = input.split("\n");

        if let Some(instructions_line) = lines.next() {
            let mut instructions = Vec::with_capacity(instructions_line.len());

            for char in instructions_line.chars() {
                if let Some(ins) = Instruction::from_char(char) {
                    instructions.push(ins);
                }
            }

            let _ = lines.next(); // Skip empty line

            let mut nodes = HashMap::new();
            for line in lines {
                if let Some(node) = Node::from_str(line) {
                    nodes.insert(node.label, node);
                }
            }

            return Some(Self {
                first_node: nodes
                    .get(&['A', 'A', 'A'])
                    .expect("nodes should have AAA")
                    .to_owned(),
                nodes,
                instructions,
            });
        }
        None
    }

    fn reach_zzz(&self) -> usize {
        let mut total_move_count = 0;
        let mut current = &self.first_node;
        while current.label != ['Z', 'Z', 'Z'] {
            for ins in &self.instructions {
                let next = match ins {
                    Instruction::Left => &current.left,
                    Instruction::Right => &current.right,
                };

                let next_node = self.nodes.get(next).expect("next node should exist");
                println!(
                    "MOVING {:?} FROM {:?} TO {:?}",
                    ins, current.label, next_node.label
                );
                current = next_node;
                total_move_count += 1;
            }
        }
        total_move_count
    }
}

#[derive(Debug)]
struct MapP2 {
    instructions: Vec<Instruction>,
    first_nodes: Vec<Node>,
    nodes: HashMap<[char; 3], Node>,
}

impl MapP2 {
    fn from_str(input: &str) -> Option<Self> {
        let mut lines = input.split("\n");

        if let Some(instructions_line) = lines.next() {
            let mut instructions = Vec::with_capacity(instructions_line.len());

            for char in instructions_line.chars() {
                if let Some(ins) = Instruction::from_char(char) {
                    instructions.push(ins);
                }
            }

            let _ = lines.next(); // Skip empty line

            let mut nodes = HashMap::new();
            for line in lines {
                if let Some(node) = Node::from_str(line) {
                    nodes.insert(node.label, node);
                }
            }

            let mut first_nodes = Vec::new();

            for (label, node) in nodes.clone() {
                if label[2] == 'A' {
                    first_nodes.push(node)
                }
            }

            return Some(Self {
                first_nodes,
                nodes,
                instructions,
            });
        }
        None
    }

    fn reach_zzz(&mut self) -> usize {
        let mut total_move_count = 0;
        let current = &mut self.first_nodes;
        while let Some(_) = &current.iter().find(|val|val.label[2]!='Z') {
            for ins in &self.instructions {
                for current in current.iter_mut() {
                    let next = match ins {
                        Instruction::Left => &current.left,
                        Instruction::Right => &current.right,
                    };
    
  
                    let next_node = self.nodes.get(next).expect("next node should exist");
                    // println!("MOVING {:?} FROM {:?} TO {:?}", ins, current.label, next_node.label);
                    *current = next_node.clone();

                }
                total_move_count += 1;
                println!("TOTAL MOVE COUNT {}", total_move_count);
            }
        }
        total_move_count
    }
}

pub fn part1(input: String) -> usize {
    if let Some(map) = Map::from_str(&input) {

        return map.reach_zzz();
    }
    0
}

pub fn part2(input: String) -> usize {
    if let Some(mut map) = MapP2::from_str(&input) {
        return map.reach_zzz();
    }
    0
}

fn main() {}
