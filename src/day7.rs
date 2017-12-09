use regex::Regex;
use std::collections::HashMap;
use std::cell::Cell;

/// A node in a tower
pub struct Node<'a> {
    pub name: &'a str,
    pub value: u32,
    // The parent might not be instantiated when a node is created so we need
    //  interior mutability so we can make a second pass to set parents
    pub parent: Cell<Option<&'a str>>,
    pub children: Vec<&'a str>
}

impl <'a> Node<'a> {
    /// Parses a str into a Node
    /// 
    /// returns None if the string cannot be parsed
    pub fn from_str(input: &'a str) -> Option<Node<'a>> {
        let re = Regex::new(r"(\pL+) \((\d+)\)( -> ([\pL ,]+))?").unwrap();
        let captures = re.captures(input);
        if let Some(cap) = captures {
            let name = &cap.get(1).map_or("", |m| m.as_str());
            let value = cap.get(2).map_or("", |m| m.as_str()).parse::<u32>().unwrap();
            let mut children = Vec::<&'a str>::new();
            if cap.len() > 3 {
                children = cap.get(4).map_or("", |m| m.as_str()).split(", ").collect::<Vec<&'a str>>();
            }
            return Some(Node {
                name: name,
                value: value,
                parent: Cell::new(None),
                children: children
            });
        }
        None
    }
}

/// A tower of nodes where each node has 0 or more children and 0 or 1 parent.
/// 
/// There must be exactly 1 node with 0 parents which is the base of the tower
#[allow(dead_code)]
pub struct Tower<'a> {
    /// A map of node name -> Node
    nodes: HashMap<&'a str, Node<'a>>,
    /// The name of the base node
    pub base: &'a str
}

impl <'a> Tower<'a> {
    pub fn from_str(input: &'a str) -> Option<Tower<'a>> {
        // Parse all lines as nodes
        let nodes = input.split("\n").map(Node::from_str).collect::<Option<Vec<Node>>>();
        if let Some(nodes) = nodes {
            // A map of node name -> node 
            let mut mapped_nodes = HashMap::<&'a str, Node<'a>>::new();
            // Insert all nodes
            for node in nodes {
                mapped_nodes.insert(node.name, node);
            }
            // For each node n, set each child's parent to n.name
            for node in mapped_nodes.values() {
                for child in &node.children {
                    let child_node = mapped_nodes.get(*child);
                    if let Some(child_node) = child_node {
                        child_node.parent.set(Some(&node.name));
                    }
                }
            }
            // Find the base node (the node with no parent)
            let base = mapped_nodes.values().find(|&n| n.parent.get() == None).unwrap().name;
            return Some(Tower { nodes: mapped_nodes, base: base })
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn test_parse_node() {
        let input = "fwft (72) -> ktlj, cntj, xhth"; 
        let node = Node::from_str(input).unwrap();
        assert_eq!("fwft", node.name);
        assert_eq!(72, node.value);
        assert_eq!(3, node.children.len());
        assert_eq!("ktlj", node.children[0]);
        assert_eq!("cntj", node.children[1]);
        assert_eq!("xhth", node.children[2]);
    }

    #[test]
    fn test_parse_tower() {
        let input ="pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)";
        let tower = Tower::from_str(input).unwrap();
        assert_eq!("tknk", tower.base);
    }
}