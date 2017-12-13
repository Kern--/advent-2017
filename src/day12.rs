use std::collections::HashSet;

type Graph<'a> = Vec<Vec<u32>>;


/// Finds the group containing the node recursively
fn find_group_recursively(node: u32, graph: &Graph, visited: &mut HashSet<u32>, group: &mut Vec<u32>) {
    if visited.contains(&node) {
       return;
    }
    group.push(node);
    visited.insert(node);
    let siblings = &graph[node as usize];
    for sibling in siblings {
        find_group_recursively(*sibling, graph, visited, group);
    }
}

/// Finds the group containing the specified start node
pub fn find_group(start: u32, graph: &Graph) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut group = Vec::new();
    find_group_recursively(start, graph, &mut visited, &mut group);
    group
}

/// Parses an input string into a graph
pub fn parse_graph(input: &str) -> Option<Graph> {
    let mut graph = Graph::new();
    for row in input.split("\n") {
        let parts = row.split(" <-> ").collect::<Vec<&str>>();
        let children = parts[1].split(", ").map(|s| s.parse::<u32>().ok()).collect::<Option<Vec<u32>>>();
        if let Some(children) = children {
            graph.push(children);
        } else {
            return None;
        }
    }
    Some(graph)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_graph() {
        let input = vec![vec![2], vec![1], vec![0, 3, 4], vec![2, 4], vec![2, 3, 6], vec![6], vec![4, 5]];
        let group = find_group(0, &input);
        assert_eq!(group.len(), 6);
        assert!(group.contains(&0), "Group did not contain 0");
        assert!(group.contains(&2), "Group did not contain 2");
        assert!(group.contains(&3), "Group did not contain 3");
        assert!(group.contains(&4), "Group did not contain 4");
        assert!(group.contains(&5), "Group did not contain 5");
        assert!(group.contains(&6), "Group did not contain 6");

        assert!(!group.contains(&1), "Group contained 1");
    }
}