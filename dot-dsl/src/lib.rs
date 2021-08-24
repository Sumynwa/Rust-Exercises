pub mod graph {
    use crate::graph::graph_items::node::*;
    use crate::graph::graph_items::edge::*;

    use std::collections::HashMap;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph { nodes: Vec::new(),
                    edges: Vec::new(),
                    attrs: HashMap::new(),}
        }

        pub fn with_nodes(mut self, node_list: &[Node]) -> Graph {
            self.nodes = node_list.to_vec();
            self
        }

        pub fn with_edges(mut self, edge_list: &[Edge]) -> Graph {
            self.edges = edge_list.to_vec();
            self
        }

        pub fn with_attrs(mut self, attr_list: &[(&str, &str)]) -> Graph {
           for (key, value) in attr_list {
               self.attrs.insert(String::from(key.clone()), String::from(value.clone()));
           }
           self
        }

        pub fn get_node(&self, node: &str) -> Option<&Node> {
            self.nodes.iter().find(|&t| t.name == String::from(node))
        }
    }

    pub mod graph_items {
       pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                 pub name: String,
                 attr : HashMap<String, String>,
            }

            impl Node {
                pub fn new(a: &str) -> Node {
                    Node {  name: String::from(a),
                            attr: HashMap::new(),
                         }
                }
                
                pub fn with_attrs(mut self, tup: &[(&str, &str)]) -> Node {
                     for t in tup {
                         let (value1, value2) = t;
                         self.attr.insert(String::from(value1.clone()), String::from(value2.clone()));
                     }
                     self
                }

               pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attr.get(key).map(|s| s.as_str())
               }
            }
        }

        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone,Debug, PartialEq)]
            pub struct Edge {
                node_pair_a: String,
                node_pair_b: String,
                attr: HashMap<String, String>,
            }
        
            impl Edge {
                pub fn new(a: &str, b: &str) -> Edge {
                    Edge {  node_pair_a: String::from(a),
                            node_pair_b: String::from(b),
                            attr: HashMap::new(),
                         }
                }

                pub fn with_attrs(mut self, tup: &[(&str, &str)]) -> Edge {
                     for t in tup {
                         let (value1, value2) = t;
                         self.attr.insert(String::from(value1.clone()), String::from(value2.clone()));
                     }
                     self
                }
            }
        }

    }
}
