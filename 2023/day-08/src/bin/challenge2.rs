use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
struct NetworkDiagram<'a> {
    instructions: Vec<Direction>,
    tree: BTreeMap<&'a str, Node<'a>>,
    starting_node_keys: Vec<&'a str>,
}

impl<'a> NetworkDiagram<'a> {
    fn find_exit(&mut self) -> usize {
        let starting_nodes: Vec<Node<'a>> = self
            .starting_node_keys
            .iter()
            .map(|starting_node_key| *self.tree.get(starting_node_key).unwrap())
            .collect();
        println!("{:?}", starting_nodes);

        let smallest_cycle_len: Vec<(usize, usize)> = starting_nodes
            .iter()
            .map(|starting_node| {
                let mut first_end = (0, "");
                let step_size = self
                    .instructions
                    .iter()
                    .cycle()
                    .enumerate()
                    .scan(starting_node, |current_node, (index, instruction)| {
                        let key;
                        *current_node = match instruction {
                            Direction::Left => {
                                key = current_node.left;
                                self.tree.get(current_node.left).unwrap()
                            }
                            Direction::Right => {
                                key = current_node.right;
                                self.tree.get(current_node.right).unwrap()
                            }
                        };
                        if first_end.0 != 0 {
                            (key != first_end.1).then_some(())
                        } else {
                            if key.chars().last() == Some('Z') {
                                first_end.0 = index;
                                first_end.1 = key;
                            }
                            Some(())
                        }
                    })
                    .count();
                (first_end.0, step_size)
            })
            .collect();
        let smallest_cycle_len: Vec<usize> = smallest_cycle_len
            .iter()
            .map(|(first_end, step_size)| {
                let step = step_size - first_end - 1;
                assert!(first_end == &step);
                step_size + 1
            })
            .collect();
        println!("{:?}", smallest_cycle_len);
        let largest_step =
            smallest_cycle_len.iter().fold(
                usize::MIN,
                |acc, val| {
                    if *val > acc {
                        *val
                    } else {
                        acc
                    }
                },
            );
        vec![largest_step]
            .iter()
            .cycle()
            .scan(usize::MIN, |acc, step_size| {
                *acc = *acc + *step_size;
                Some(*acc)
            })
            .take_while(|current_step_count| {
                !smallest_cycle_len
                    .iter()
                    .map(|len| {
                        let rem = current_step_count % *len;
                        rem
                    })
                    .all(|len| len == 0)
            })
            .last()
            .unwrap()
    }
}

fn parse_node<'a>(input: &'a str) -> IResult<&str, (&'a str, Node<'a>)> {
    let (input, key) = terminated(alphanumeric1, tag(" = "))(input)?;
    let (input, (_, left, _, right, _)) =
        tuple((tag("("), alphanumeric1, tag(", "), alphanumeric1, tag(")")))(input)?;
    Ok((input, (key, Node { left, right })))
}

fn parse_instruction<'a>(input: &'a str) -> IResult<&str, Vec<Direction>> {
    let (input, instructions) = terminated(alpha1, tuple((line_ending, line_ending)))(input)?;
    let instructions = instructions
        .chars()
        .map(|char| match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            c => panic!("Malformed Instruction {c}"),
        })
        .collect();
    Ok((input, instructions))
}

fn parse_network_diagram<'a>(input: &'a str) -> IResult<&str, NetworkDiagram<'a>> {
    let (input, instructions) = parse_instruction(input)?;
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;
    let mut tree = BTreeMap::new();
    nodes.iter().for_each(|(key, node)| {
        tree.insert(*key, *node);
    });

    Ok((
        input,
        NetworkDiagram {
            instructions,
            tree,
            starting_node_keys: nodes
                .iter()
                .map(|(key, _)| *key)
                .filter(|key| key.chars().last() == Some('A'))
                .collect(),
        },
    ))
}

fn main() {
    let input = include_str!("./input2.txt");
    let (_input, mut network_diagram) = parse_network_diagram(input).expect("Should parse diagram");
    println!("{:?}", network_diagram.find_exit());
}
