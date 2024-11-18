#![allow(dead_code)]
use petgraph::graph::{DiGraph, NodeIndex};
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use std::collections::HashMap;

use hex;
use std::error::Error;

fn hash_label(label: &str, digest_size: usize) -> Result<String, Box<dyn Error>> {
    let mut hasher = Blake2bVar::new(digest_size)?;
    hasher.update(label.as_bytes());
    let mut buf = vec![0u8; digest_size];
    hasher.finalize_variable(&mut buf)?;
    Ok(hex::encode(buf))
}


fn init_node_labels(graph: &DiGraph<String, ()>, node_attr: Option<&str>) -> HashMap<NodeIndex, String> {
    graph.node_indices().map(|node| {
        let label = match node_attr {
            Some(_attr) => graph[node].clone(),
            None => graph.neighbors_undirected(node).count().to_string(),
        };
        (node, label)
    }).collect()
}

fn neighborhood_aggregate(graph: &DiGraph<String, ()>, node: NodeIndex, node_labels: &HashMap<NodeIndex, String>) -> Result<String, Box<dyn Error>> {
    let error_message = "{} label for {:?} not found";

    let mut label_list: Vec<String> = graph.neighbors(node)
        .map(|neighbor| -> Result<String, Box<dyn Error>> {
            node_labels.get(&neighbor)
                .ok_or_else(|| format!("{}{}{:?}", error_message, "Neighbor", neighbor).into())
                .map(|label| label.clone())
        })
        .collect::<Result<Vec<String>, Box<dyn Error>>>()?;

    label_list.sort();
    let node_label = node_labels.get(&node)
        .ok_or_else(|| -> String { format!("{}{}{:?}", error_message, "Node", node).into()} )?;

    Ok(node_label.clone() + &label_list.concat())
}

fn weisfeiler_lehman_step(graph: &DiGraph<String, ()>, labels: &HashMap<NodeIndex, String>, digest_size: usize) -> Result<HashMap<NodeIndex, String>, Box<dyn Error>> {
    graph.node_indices().map(|node| {
        let label = neighborhood_aggregate(graph, node, labels)?;
        hash_label(&label, digest_size).map(|hashed_label| (node, hashed_label))
    }).collect::<Result<HashMap<_, _>, _>>()
}

pub fn weisfeiler_lehman_graph_hash(graph: &DiGraph<String, ()>, node_attr: Option<&str>, iterations: usize, digest_size: usize) -> Result<String, Box<dyn Error>> {
    let mut node_labels = init_node_labels(graph, node_attr);

    let mut subgraph_hash_counts = Vec::new();
    for _ in 0..iterations {
        node_labels = weisfeiler_lehman_step(graph, &node_labels, digest_size)?;
        let mut counter = HashMap::new();
        for label in node_labels.values() {
            *counter.entry(label.clone()).or_insert(0) += 1;
        }
        let mut sorted_counts: Vec<_> = counter.into_iter().collect();
        sorted_counts.sort_by_key(|k| k.0.clone());
        subgraph_hash_counts.extend(sorted_counts);
    }

    let final_label = format!("{:?}", subgraph_hash_counts);
    hash_label(&final_label, digest_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;
    use std::collections::HashMap;

    #[test]
    fn test_hash_label() {
        let label = "Teststring";
        let digest_size = 16;
        match hash_label(label, digest_size) {
            Ok(hash) => println!("{}", hash),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    #[test]
    fn test_weisfeiler_lehman_graph_hash() {
        let mut graph = DiGraph::new();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());

        let hash = weisfeiler_lehman_graph_hash(&graph, None, 3, 16).unwrap();
        println!("{}",hash);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_weisfeiler_lehman_graph_hash_isomorphic() {
        let mut g1 = DiGraph::new();
        let n1 = g1.add_node("1".to_string());
        let n2 = g1.add_node("2".to_string());
        let n3 = g1.add_node("3".to_string());
        let n4 = g1.add_node("4".to_string());

        g1.add_edge(n1, n2, ());
        g1.add_edge(n2, n3, ());
        g1.add_edge(n3, n1, ());
        g1.add_edge(n1, n4, ());

        let mut g2 = DiGraph::new();
        let n5 = g2.add_node("5".to_string());
        let n6 = g2.add_node("6".to_string());
        let n7 = g2.add_node("7".to_string());
        let n8 = g2.add_node("8".to_string());

        g2.add_edge(n5, n6, ());
        g2.add_edge(n6, n7, ());
        g2.add_edge(n7, n5, ());
        g2.add_edge(n7, n8, ());

        let hash1 = weisfeiler_lehman_graph_hash(&g1, None, 3, 16).unwrap();
        let hash2 = weisfeiler_lehman_graph_hash(&g2, None, 3, 16).unwrap();

        println!("Isomorphic");
        println!("Hash1: {}", hash1);
        println!("Hash2: {}", hash2);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_weisfeiler_lehman_graph_hash_non_isomorphic() {
        let mut g1 = DiGraph::new();
        let n1 = g1.add_node("1".to_string());
        let n2 = g1.add_node("2".to_string());
        let n3 = g1.add_node("3".to_string());
        let n4 = g1.add_node("4".to_string());

        g1.add_edge(n1, n2, ());
        g1.add_edge(n2, n3, ());
        g1.add_edge(n3, n1, ());
        g1.add_edge(n1, n4, ());

        let mut g2 = DiGraph::new();
        let n5 = g2.add_node("5".to_string());
        let n6 = g2.add_node("6".to_string());
        let n7 = g2.add_node("7".to_string());
        let n8 = g2.add_node("8".to_string());

        g2.add_edge(n5, n6, ());
        g2.add_edge(n6, n7, ());
        g2.add_edge(n7, n5, ());
        g2.add_edge(n7, n8, ());

        let hash1 = weisfeiler_lehman_graph_hash(&g1, Some("label"), 3, 16).unwrap();
        let hash2 = weisfeiler_lehman_graph_hash(&g2, Some("label"), 3, 16).unwrap();

        println!("Non-Isomorphic");
        println!("Hash1: {}", hash1);
        println!("Hash2: {}", hash2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_init_node_labels() {

        let mut graph = DiGraph::new();

        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());

        let labels_with_attr = init_node_labels(&graph, Some("label"));
        assert_eq!(labels_with_attr.get(&a).unwrap(), "A");
        assert_eq!(labels_with_attr.get(&b).unwrap(), "B");
        assert_eq!(labels_with_attr.get(&c).unwrap(), "C");

        let labels_without_attr = init_node_labels(&graph, None);
        assert_eq!(labels_without_attr.get(&a).unwrap(), "1");
        assert_eq!(labels_without_attr.get(&b).unwrap(), "2");
        assert_eq!(labels_without_attr.get(&c).unwrap(), "1");
    }

    #[test]
    fn test_neighborhood_aggregate() {
        // Create a new directed graph
        let mut graph = DiGraph::new();

        // Add nodes to the graph
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());

        // Add edges to the graph
        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());

        // Initialize node labels
        let mut node_labels = HashMap::new();
        node_labels.insert(a, "A".to_string());
        node_labels.insert(b, "B".to_string());
        node_labels.insert(c, "C".to_string());

        // Test neighborhood aggregation for node 'a'
        let result = neighborhood_aggregate(&graph, a, &node_labels);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "A".to_string() + "B");

        // Test neighborhood aggregation for node 'b'
        let result = neighborhood_aggregate(&graph, b, &node_labels);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "B".to_string() + "C");

        // Test neighborhood aggregation for node 'c'
        let result = neighborhood_aggregate(&graph, c, &node_labels);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "C".to_string());
    }
}