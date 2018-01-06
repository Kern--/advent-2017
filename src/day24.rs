use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::convert::TryFrom;
use std::collections::HashSet;

#[derive(Debug,PartialEq, Eq, Hash, Clone)]
pub struct Node {
    pub id: usize,
    pub left: u32,
    pub right: u32
}

impl Node {
    pub fn strength(&self) -> u32 {
        self.left + self.right
    }
}

impl <'a> TryFrom<&'a str> for Node {
    type Error = NodeParseError<'a>;
    fn try_from(input: &'a str) -> Result<Node, Self::Error> {
        let parts = input.split("/").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(NodeParseError::InvalidStructure(input));
        }
        let left = parts[0].parse::<u32>().map_err(|_| NodeParseError::InvalidLeftSide(input, parts[0]))?;
        let right = parts[1].parse::<u32>().map_err(|_| NodeParseError::InvalidRightSide(input, parts[1]))?;
        Ok(Node{id: 0, left, right})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeParseError<'a> {
    InvalidStructure(&'a str),
    InvalidLeftSide(&'a str, &'a str),
    InvalidRightSide(&'a str, &'a str)
}

impl <'a> Display for NodeParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            NodeParseError::InvalidStructure(input) => write!(f, "Invalid structure. Expected 'a/b' got {}", input),
            NodeParseError::InvalidLeftSide(input, left) => write!(f, "Invalid left hand side. Expected number, got {} in {}", left, input),
            NodeParseError::InvalidRightSide(input, right) => write!(f, "Invalid right hand side. Expected number, got {} in {}", right, input),
        }
    }
}

impl <'a> Error for NodeParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            NodeParseError::InvalidStructure(_) => "Invalid structure",
            NodeParseError::InvalidLeftSide(_, _) => "Invalid left hand side",
            NodeParseError::InvalidRightSide(_, _) => "Invalid right hand side",
        }
    }
}

fn search_for_strongest_bridge(nodes: &Vec<Node>, seen: HashSet<Node>, current_strength: u32, constraint: u32) -> u32 {
    nodes.iter()
        // Select nodes that can be connected to the current nodes
        .filter(|node| node.left == constraint || node.right == constraint)
        // Select nodes which haven't already been seen
        .filter(|node| !seen.contains(&node))
        // Calculate the max-strength of all extended bridges that can be made from the current bridge extended by one node
        .map(|sub_node| {
            let mut new_seen: HashSet<Node> = seen.iter().cloned().collect();
            new_seen.insert(sub_node.clone());
            let new_strength = current_strength + sub_node.strength();
            let new_constraint = if sub_node.left == constraint { sub_node.right } else { sub_node.left };
            search_for_strongest_bridge(nodes, new_seen, new_strength, new_constraint)})
        // Choose the maximum strength of all bridges that can be made by extending the current one
        .max_by(|a, b| a.cmp(b))
        // If no bridges can be made by extension, the max is the current strength;
        .unwrap_or(current_strength)
}

/// Computes the strength of the strongest bridge that can be made
///  from the input string.
pub fn compute_strongest_bridge(input: &str) -> Result<u32, NodeParseError> {
    // Parse the nodes
    let mut nodes = input.split("\n").map(Node::try_from).collect::<Result<Vec<Node>, NodeParseError>>()?;
    // Give them IDs so that if 2 nodes have the same left/right, they're distinguishable
    for (id, node) in nodes.iter_mut().enumerate() {
        node.id = id;
    }
    // Call a recursive method to find the strongest
    Ok(search_for_strongest_bridge(&nodes, HashSet::new(), 0, 0))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_node() {
        let mut input = "10/12";
        assert_eq!(Node::try_from(input), Ok(Node{id: 0, left: 10, right: 12}));

        input = "10-12";
        assert_eq!(Node::try_from(input), Err(NodeParseError::InvalidStructure(&input)));

        input = "a/12";
        assert_eq!(Node::try_from(input), Err(NodeParseError::InvalidLeftSide(&input, &"a")));

        input = "10/b";
        assert_eq!(Node::try_from(input), Err(NodeParseError::InvalidRightSide(&input, &"b")));
    }
}