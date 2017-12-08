use std::fmt;
use std::error;
use std::num;

#[derive(Debug)]
pub enum GraphError { 
  GraphError,
  NodeParseError(String),
  GraphParseWeightError(num::ParseIntError),
}

impl error::Error for GraphError {  
    fn description(&self) -> &str {
        match *self {
            GraphError::NodeParseError(_) => "Can't parse graph node",
            GraphError::GraphParseWeightError(_) => "Integer parsing error",
            GraphError::GraphError => "A graph error occured"
        }
    }
    
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GraphError::GraphParseWeightError(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }

}

impl fmt::Display for GraphError {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GraphError::NodeParseError(ref node) => f.write_str(&format!("Can't parse graph node: {}", &node)),
            GraphError::GraphParseWeightError(ref e) => e.fmt(f),
            GraphError::GraphError => f.write_str("The graph doesn't make sense!"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Node {
    index: usize,
}

#[derive(Debug)]
pub struct NodeContainer<T> {
  data: T,
  parent: Option<Node>,
  children: Vec<Node>,
}

pub struct Ancestors<'a, T: 'a> {
  graph: &'a Graph<T>,
  node: Option<Node>,
}

impl<'a, T> Iterator for Ancestors<'a, T> {
  type Item = Node;
  
  fn next(&mut self) -> Option<Node> {
    match self.node.take() {
      Some(node) => {
        self.node = self.graph.nodes[node.index].parent;
        Some(node)
      }
      None => None
    }
  }
}

pub struct NodeIterator<'a, T: 'a> {
  graph: &'a Graph<T>,
  index: usize
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
  type Item = Node;
  fn next(&mut self) -> Option<Node> {
    if self.graph.nodes.len() > self.index {
      let node = Node { index: self.index };
      self.index += 1;
      return Some(node)
    }
    None
  }
}

impl Node {
  
  pub fn add_children<T>(self, children: &[Node], graph: &mut Graph<T>) -> Result<(), GraphError> {
    for node in children.iter() {
      {
        let node_container = &mut graph.nodes[node.index];
        node_container.parent = Some(self);
      }
      {
        let self_container = &mut graph.nodes[self.index];
        self_container.children.push(*node);        
      }
    }
    Ok(())
  }
  
  pub fn ancestors<T>(self, graph: &Graph<T>) -> Ancestors<T> {
    Ancestors {
      graph: graph,
      node: graph.nodes[self.index].parent,
    }
  }
  
  pub fn children<T>(self, graph: &Graph<T>) -> &Vec<Node> {
    &graph.nodes[self.index].children
  }
  
  pub fn parent<T>(self, graph: &Graph<T>) -> Option<Node> {
    graph.nodes[self.index].parent
  }
}

#[derive(Debug)]
pub struct Graph<T> {
  nodes: Vec<NodeContainer<T>>
}

impl<T> Graph<T> {
  
  pub fn new() -> Graph<T> {
    Graph { nodes: Vec::new() }
  }
  
  pub fn get_data(&self, node: &Node) -> &T {
    &self.nodes[node.index].data
  }
  
  // Return the first node.
  pub fn first(&self) -> Option<Node> {
    if self.nodes.len() > 0 {
      Some(Node { index: 0 })
    } else {
      None
    }
  }
  
  pub fn iter(&self) -> NodeIterator<T> {
    NodeIterator {
      graph: self,
      index: 0,
    }
  }
  
  pub fn find_root(&self) -> Option<Node> {
    self.first().and_then(|x| x.ancestors(self).last())
  }
  
  pub fn new_node(&mut self, data: T) -> Node {
    
    let index = self.nodes.len();
    
    // Push the node into the arena
    self.nodes.push(NodeContainer {
        parent: None,
        children: Vec::new(),
        data: data,
    });

    // Return the node identifier
    Node { index : index }
  }

}