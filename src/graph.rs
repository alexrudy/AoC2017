//! A graph manipulation library.
//!
//! Implements a directional graph
//! on top of some data type held
//! at each node.

use std::num;
use std::collections::VecDeque;
use std::collections::HashSet;

/// Error returned when the graph
/// has a problem.
#[derive(Fail, Debug)]
pub enum GraphError {
  
  #[fail(display="Graph doesn't make sense.")]
  GraphError,
  
  #[fail(display="Can't parse graph node: {}", _0)]
  NodeParseError(String),
  
  #[fail(display="Can't parse integer.")]
  GraphParseWeightError(#[cause] num::ParseIntError),
}


/// A representation of a node,
/// which does not reference it's parent
/// graph, and so can be used to link
/// and manipulate the structure of the graph.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Node {
  index: usize,
}

/// A container for node data.
#[derive(Debug)]
pub struct NodeContainer<T> {
  data: T,
  parent: Option<Node>,
  children: Vec<Node>,
}

/// An iterator over graph ancestors.
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

/// An iterator over individual nodes.
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

/// An iterator searching for a particular node.
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
  
  /// Append the given node to this node as child.
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
  
  /// Add children to this node.
  pub fn add_children<T>(&self, children: &[Node], graph: &mut Graph<T>) -> Result<(), GraphError> {
    for node in children.iter() {
      self.append(node, graph)?
    }
    Ok(())
  }
  
  /// Iterate over the ancestors of this graph.
  pub fn ancestors<T>(self, graph: &Graph<T>) -> Ancestors<T> {
    Ancestors {
      graph: graph,
      node: graph.nodes[self.index].parent,
    }
  }
  
  /// Return a reference to the vector containing
  /// the direct children of this node.
  pub fn children<T>(self, graph: &Graph<T>) -> &Vec<Node> {
    &graph.nodes[self.index].children
  }

  /// Return the parent of this node.
  pub fn parent<T>(self, graph: &Graph<T>) -> Option<Node> {
    graph.nodes[self.index].parent
  }
  
  /// Iterate over all connected nodes to this node.
  pub fn connected<T>(self, graph: &Graph<T>) -> NodeSearchIterator<T> {
    NodeSearchIterator::new(self.index, graph)
  }
}


/// A group of possibly connected nodes.
#[derive(Debug)]
pub struct Graph<T> {
  nodes: Vec<NodeContainer<T>>,
}

impl<T> Graph<T> {
  
  /// Make a new, empty graph.
  pub fn new() -> Graph<T> {
    Graph { nodes: Vec::new() }
  }
  
  /// Get the data belonging to a given node.
  pub fn get_data(&self, node: &Node) -> &T {
    &self.nodes[node.index].data
  }

  /// Get the first node in the graph (by insertion order)
  pub fn first(&self) -> Option<Node> {
    if self.nodes.len() > 0 {
      Some(Node { index: 0 })
    } else {
      None
    }
  }
  
  /// Iterate over all nodes in the graph.
  pub fn iter(&self) -> NodeIterator<T> {
    NodeIterator {
      graph: self,
      index: 0,
    }
  }
  
  /// Number of nodes in the graph.
  pub fn len(&self) -> usize {
    self.nodes.len()
  }
  
  /// Number of groups in the graph.
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
  
  /// Create a new node, with some data.
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
