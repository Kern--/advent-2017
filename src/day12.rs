use std::collections::HashSet;

type Graph<'a> = Vec<Vec<u32>>;


/// Finds the group containing the node recursively
fn find_group_recursively(node: u32, graph: &Graph, group: &mut HashSet<u32>) {
    if group.contains(&node) {
       return;
    }
    group.insert(node);
    let siblings = &graph[node as usize];
    for sibling in siblings {
        find_group_recursively(*sibling, graph, group);
    }
}

/// Finds the group containing the specified start node
pub fn find_group(start: u32, graph: &Graph) -> HashSet<u32> {
    let mut group = HashSet::new();
    find_group_recursively(start, graph, &mut group);
    group
}

/// Finds all groups in the graph
pub fn find_all_groups(graph: &Graph) -> Vec<HashSet<u32>> {
    // This could definitely be simplified by starting with a list of all nodes and removing them as 
    //  they're found in find_group_recursively, but it seems odd that finding groups would have to 
    //  keep track of which nodes were not found. HashSet lookups should be fast enough that its
    //  probably find to do this less efficient, but conceptually simpler algorithm.
    let mut groups = Vec::<HashSet<u32>>::new();
    for n in 0..graph.len() as u32 {
        if !(&groups).into_iter().any(|g| g.contains(&n)) {
            groups.push(find_group(n, graph));
        }
    }
    groups
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

        assert_eq!(find_all_groups(&input).len(), 2);
    }
}