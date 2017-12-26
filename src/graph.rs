use std::fmt;
use std::error;
use std::num;
use std::collections::VecDeque;
use std::collections::HashSet;

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
      GraphError::GraphError => "A graph error occured",
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
      GraphError::NodeParseError(ref node) => {
        f.write_str(&format!("Can't parse graph node: {}", &node))
      }
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
      None => None,
    }
  }
}

pub struct NodeIterator<'a, T: 'a> {
  graph: &'a Graph<T>,
  index: usize,
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
  type Item = Node;
  fn next(&mut self) -> Option<Node> {
    if self.graph.nodes.len() > self.index {
      let node = Node { index: self.index };
      self.index += 1;
      return Some(node);
    }
    None
  }
}

pub struct NodeSearchIterator<'a, T: 'a> {
  graph: &'a Graph<T>,
  seen: HashSet<usize>,
  queue: VecDeque<usize>,
  index: Option<usize>,
}

impl<'a, T> NodeSearchIterator<'a, T> {
  fn new(index: usize, graph: &'a Graph<T>) -> NodeSearchIterator<'a, T> {
    let mut iter = NodeSearchIterator {
      graph: graph,
      seen: HashSet::new(),
      queue: VecDeque::new(),
      index: Some(index),
    };
    iter.seen.insert(index);
    return iter;
  }

  fn check(&mut self, index: usize) {
    if !self.seen.contains(&index) {
      self.seen.insert(index);
      self.queue.push_back(index);
    }
  }
}

impl<'a, T> Iterator for NodeSearchIterator<'a, T> {
  type Item = Node;

  fn next(&mut self) -> Option<Node> {
    match self.index {
      None => None,
      Some(index) => {
        let node = Node { index: index };
        if let Some(pnode) = node.parent(self.graph) {
          self.check(pnode.index)
        }
        for child in node.children(self.graph) {
          self.check(child.index)
        }
        self.index = self.queue.pop_front();
        return Some(node);
      }
    }
  }
}

impl Node {
  pub fn append<T>(&self, node: &Node, graph: &mut Graph<T>) -> Result<(), GraphError> {
    {
      let node_container = &mut graph.nodes[node.index];
      node_container.parent = Some(*self);
    }
    {
      let self_container = &mut graph.nodes[self.index];
      self_container.children.push(*node);
    }
    Ok(())
  }

  pub fn add_children<T>(&self, children: &[Node], graph: &mut Graph<T>) -> Result<(), GraphError> {
    for node in children.iter() {
      self.append(node, graph)?
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

  pub fn connected<T>(self, graph: &Graph<T>) -> NodeSearchIterator<T> {
    NodeSearchIterator::new(self.index, graph)
  }
}

#[derive(Debug)]
pub struct Graph<T> {
  nodes: Vec<NodeContainer<T>>,
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

  pub fn len(&self) -> usize {
    self.nodes.len()
  }

  pub fn count_groups(&self) -> usize {
    let mut ngroups = 0;
    let mut seen: HashSet<Node> = HashSet::new();
    for node in self.iter() {
      if !seen.contains(&node) {
        ngroups += 1;
        seen.insert(node);
        for cnode in node.connected(self) {
          seen.insert(cnode);
        }
      }
    }
    ngroups
  }

  /// Finds the root node, but assumes all nodes are connected.
  pub fn root(&self) -> Option<Node> {
    self.first().and_then(|x| x.ancestors(self).last())
  }

  pub fn node(&mut self, data: T) -> Node {
    let index = self.nodes.len();

    // Push the node into the arena
    self.nodes.push(NodeContainer {
      parent: None,
      children: Vec::new(),
      data: data,
    });

    // Return the node identifier
    Node { index: index }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn create_graph() {
    let mut g: Graph<String> = Graph::new();
    let node = g.node("Hello".to_string());
    assert_eq!(g.get_data(&node), "Hello")
  }

}
