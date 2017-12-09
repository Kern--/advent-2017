// This whole implementation is a bit awkward.
// I think restructuring the tree to separate IDs from Nodes might make this a little less awkward.
// A similar implementation to something the following might help:
// https://github.com/SimonSapin/rust-forest

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
    pub children: Vec<&'a str>,
    weight: Cell<Option<u32>>,
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
                children = cap.get(4).map_or(Vec::<&'a str>::new(), |m| m.as_str().split(", ").collect::<Vec<&'a str>>());
            }
            return Some(Node {
                name: name,
                value: value,
                parent: Cell::new(None),
                children: children,
                weight: Cell::new(None),
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
    
    /// Gets the weight of of a subtower of a tower
    pub fn calculate_weight(&self, node_name: &str) -> u32 {
        let node = &self.nodes[node_name];
        if let Some(weight) = node.weight.get() {
            return weight;
        }

        let children = &node.children;
        let weight = node.value + children.into_iter().map(|c| self.calculate_weight(c)).sum::<u32>();
        node.weight.set(Some(weight));
        weight
    }

    fn get_node(&self, node_name: &'a str) -> &Node<'a> {
        &self.nodes[node_name]
    }

    /// Calculates the correct value for the single node in a tower which is causing the tower to be imbalanced.
    /// Imbalanced is defined as a node who's wegiht is not equal to all of its siblings.
    /// 
    /// ## Algorithm
    /// We know that:
    /// for a node n, weight w is calculated as:
    /// w = n.value + sum(n.children.value)
    /// 
    /// In a balanced node, we know that:
    ///  n.child[0].value = n.child[1].value = n.childe[2].value ...
    /// 
    /// If the node's parent is not the root, then the properly balanced node has another constraint:
    /// w = parent.sibling.weight
    /// 
    /// The algorithm is set up so that n is in the imbalanced subtree
    /// Assume n is balanced. Then, from above,
    /// parent.sibling.weight = n.value + child_value * num_children or
    /// child_value = (parent.sibling.weight - n.value) / num_children.
    /// 
    /// If n if not balanced, then there exists exactly 1 j for which n.child[j].value != child_value.
    ///  in which case, n.child[j] is incorrect, and the expected value is child_value, so we can recurse.
    /// 
    /// If n is balanced, then n.value is incorrect.
    /// We can detect this because there is no j for which n.child[j] == child_value. This is distinct from the previous case 
    /// since we would have found num_children - i nodes for which n.child[j] holds. In this case, we've found the bad node, 
    /// and the correct value is given by correct_weight = w - num_children * correct_child_value
    /// where correct_child_value is the value that each child actually holds
    fn calculate_corected_weight_with_constraints(&'a self, current_node: &'a Node, expected_value: u32) -> u32 {
        
        let num_children = current_node.children.len() as u32;
        let expected_child_value = (expected_value as i32 - current_node.value as i32).abs() as u32 / num_children;
        let mut actual_child_value = 0;
        let mut bad_child: &'a str = "";
        let mut found_expected_child_value: bool = false;
        for (child, value) in (&current_node.children).into_iter().map(|c| (c, self.calculate_weight(c))) {
            if value != expected_child_value {
                // If we found a child which has a value other than the expected value,
                // tentatively remember it as the base of a bad subtower
                bad_child = child;
                actual_child_value = value;
            } else {
                // If we found a child which does have a value == expected value
                //  keep track of that as it implies the current node is not the node with the problem
                found_expected_child_value = true;
            }
        }

        if found_expected_child_value && bad_child != "" {
            // If we found nodes with the expected value and the bad_child is not empty
            //  then we found a single child for which the value was unexpected and we shoudl recurse on that subtower
            self.calculate_corected_weight_with_constraints(self.get_node(bad_child), expected_child_value)
        } else {
            // If we didn't find the expected subchild, or all of the children were expected,
            // then the current node is balanced and the problem is caused by the current node's value
            //
            // Note: if all of the children were expected, then this means that the difference in the node's incorrect value from the correct value
            //  is less than num_children which would cause the floored division in the expected_child_value calculation to = actual_value
            expected_value - (actual_child_value * num_children)
        }
    }

    /// Calculates the correct value for the single node in a tower which is causing the tower to be imbalanced.
    /// Imbalanced is defined as a node who's wegiht is not equal to all of its siblings
    /// 
    /// The return value is the correct weight of the node, there is no indication of which node was imbalanced.
    pub fn calculate_corrected_weight(&'a self) -> u32 {
        let base = self.get_node(self.base);
        let num_children = base.children.len();
        // If the number of children of the base < 3, it's possible to have a configuration with multiple solutions,
        //  so instead of trying to detect that, we'll just reject the input.
        // If the root had 2 children with different values, but are otherwise balanced,
        //  either child could be modified to have the other's value and it would be valid.
        // Technically, if the base had only 1 child, the algorithm should work by treating that child as the base.
        //  A full solution should probably handle that.
        if num_children < 3 {
            return 0;
        }

        // Find the child with the different weight.
        //  Since the base has no constraints about it's value, get_incorrect_subtower won't work.
        //  For this case we'll brute force it - check all other value to see if there's another that matches
        //  If there is, the expected values for all children is the current child's value
        //  If there  isn't, the current child is the bad node
        let child_weights = (&base.children).into_iter().map(|c| (c, self.calculate_weight(c))).collect::<Vec<(&&str, u32)>>();
        let mut _ignored = "";
        let mut expected_value = 0;
        let mut bad_node: &str = "";
        for i in 0..child_weights.len() {
            let mut is_unique = true;
            for j in 0..child_weights.len() {
                let (_, i_value) = child_weights[i];
                let (_, j_value) = child_weights[j];
                // Obviously a node's value is equal to it's own weight, ignore it
                if i == j { continue; }
                if i_value == j_value {
                    is_unique = false;
                    // if the node is not unique, then the current value is the value that all nodes should be as only 1 node is allowed to be incorrect
                    expected_value = i_value;
                    break;
                }
            }
            // if  the node is unique, then it must be the bad value. 
            if is_unique {
                let (node, _) = child_weights[i];
                bad_node = node;
            }
        }
        let node = self.get_node(bad_node);
        // Recurse on an algorithm now that we have constraints on the node's value
        self.calculate_corected_weight_with_constraints(node, expected_value)
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
        assert_eq!(60, tower.calculate_corrected_weight());
    }
}