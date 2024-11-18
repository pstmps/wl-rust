mod wl;

#[cfg(test)]
mod tests {
    use super::wl::weisfeiler_lehman_graph_hash;
    use petgraph::graph::DiGraph;

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

}