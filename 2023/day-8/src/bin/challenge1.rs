use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, not_line_ending},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
struct NetworkDiagram<'a> {
    instructions: &'a str,
    tree: BTreeMap<&'a str, Node<'a>>,
}

impl<'a> NetworkDiagram<'a> {
    fn find_exit(&mut self) -> usize {
        let mut current_node = self.tree.get("AAA").unwrap();
        self.instructions
            .chars()
            .cycle()
            .take_while(|instruction| {
                let mut new_node_key = "";
                match instruction {
                    'L' => {
                        new_node_key = current_node.left;
                    }
                    'R' => {
                        new_node_key = current_node.right;
                    }
                    _ => (),
                }
                current_node = self.tree.get(new_node_key).unwrap();
                new_node_key != "ZZZ"
            })
            .count()
            + 1
    }
}

fn parse_node<'a>(input: &'a str) -> IResult<&str, (&'a str, Node<'a>)> {
    let (input, key) = terminated(alpha1, tag(" = "))(input)?;
    let (input, (_, left, _, right, _)) =
        tuple((tag("("), alpha1, tag(", "), alpha1, tag(")")))(input)?;
    Ok((input, (key, Node { left, right })))
}

fn parse_network_diagram<'a>(input: &'a str) -> IResult<&str, NetworkDiagram<'a>> {
    let (input, instructions) = terminated(alpha1, tuple((line_ending, line_ending)))(input)?;
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;
    let mut tree = BTreeMap::new();
    nodes.iter().for_each(|(key, node)| {
        tree.insert(*key, *node);
    });

    Ok((input, NetworkDiagram { instructions, tree }))
}

fn main() {
    let input = include_str!("./input1.txt");
    let (_input, mut network_diagram) = parse_network_diagram(input).expect("Should parse diagram");
    println!("{:?}", network_diagram.find_exit());
}
