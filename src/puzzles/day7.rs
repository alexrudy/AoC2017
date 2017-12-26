#[allow(unused_imports)]
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;

use super::super::graph;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
  pub name: String,
  pub weight: u32,
}

impl graph::Graph<Program> {
  pub fn parse_node(
    &mut self,
    text: &str,
  ) -> Result<(graph::Node, Vec<String>), graph::GraphError> {
    let mut parts = text.trim().split(char::is_whitespace);
    let name = parts
      .next()
      .ok_or(graph::GraphError::NodeParseError(text.to_string()))?
      .to_string();


    let weight = {
      let weight_text = parts
        .next()
        .ok_or(graph::GraphError::NodeParseError(text.to_string()))?;
      let wsize = weight_text.len() - 1;
      weight_text[1..wsize].parse::<u32>().unwrap()
    };

    let node = self.node(Program {
      name: name,
      weight: weight,
    });

    let arrow = parts.next();
    let mut children = Vec::new();
    if arrow.is_some() {
      children.extend(parts.map(|part| part.trim_matches(',').to_string()));
    }
    Ok((node, children))
  }

  pub fn parse_nodes<Q: io::BufRead>(
    &mut self,
    lines: io::Lines<Q>,
  ) -> Result<(), graph::GraphError> {
    // Store the children, so we can map them later.
    let mut children_map = HashMap::new();
    let mut node_map = HashMap::new();

    // Collect all the nodes
    for line in lines {
      let (node, children) = self.parse_node(&line.unwrap())?;
      children_map.insert(node, children);
      node_map.insert(self.get_data(&node).name.clone(), node);
    }

    // Set up the graph
    for (node, children) in children_map {
      let child_nodes: Vec<graph::Node> = children
        .iter()
        .map(|x| node_map.get(&x.to_string()).unwrap().clone())
        .collect();
      node.add_children(&child_nodes, self)?;
    }

    Ok(())
  }

  pub fn find_node(&self, name: &str) -> Option<graph::Node> {
    for node in self.iter() {
      if self.get_data(&node).name == name {
        return Some(node);
      }
    }
    None
  }
}

impl graph::Node {
  pub fn weight(&self, nodes: &graph::Graph<Program>) -> u32 {
    let csum: u32 = self.children(nodes).iter().map(|x| x.weight(nodes)).sum();
    nodes.get_data(self).weight + csum
  }

  pub fn balanced(&self, nodes: &graph::Graph<Program>) -> bool {
    let mut child_weight = None;
    for cw in self.children(nodes).iter().map(|x| x.weight(nodes)) {
      child_weight = match child_weight {
        None => Some(cw),
        Some(ew) => Some(ew),
      };
      if child_weight != Some(cw) {
        return false;
      }
    }
    true
  }

  fn target_weight(&self, nodes: &graph::Graph<Program>) -> Option<u32> {
    let mut weights = HashMap::new();
    for sibling in self.parent(nodes)?.children(nodes) {
      match weights.entry(sibling.weight(nodes)) {
        Entry::Occupied(mut e) => {
          *e.get_mut() += 1;
        }
        Entry::Vacant(k) => {
          k.insert(1);
        }
      };
    }
    let cmax = weights.values().max();
    let mut result = None;
    for (weight, count) in &weights {
      if Some(count) == cmax {
        result = Some(*weight);
      }
    }
    result
  }

  pub fn badweight(&self, nodes: &graph::Graph<Program>) -> bool {
    let parent = self.parent(nodes);
    if (!parent.map(|x| x.balanced(nodes)).unwrap_or(true)) & (self.balanced(nodes)) {
      return self.target_weight(nodes) != Some(self.weight(nodes));
    } else {
      return false;
    }
  }

  pub fn fixed_weight(&self, nodes: &graph::Graph<Program>) -> Option<u32> {
    self.target_weight(nodes).map(|x| {
      x
        - self
          .children(nodes)
          .iter()
          .map(|x| x.weight(nodes))
          .sum::<u32>()
    })
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn parse_programs_test() {
    let program_input = "pbga (66)
  xhth (57)
  ebii (61)
  havc (66)
  ktlj (57)
  fwft (72) -> ktlj, cntj, xhth
  qoyq (66)
  padx (45) -> pbga, havc, qoyq
  tknk (41) -> ugml, padx, fwft
  jptl (61)
  ugml (68) -> gyxo, ebii, jptl
  gyxo (61)
  cntj (57)";

    let mut g: graph::Graph<Program> = graph::Graph::new();
    g.parse_nodes(program_input.as_bytes().lines()).unwrap();
  }

  #[test]
  fn find_root_node_test() {
    let program_input = "pbga (66)
  xhth (57)
  ebii (61)
  havc (66)
  ktlj (57)
  fwft (72) -> ktlj, cntj, xhth
  qoyq (66)
  padx (45) -> pbga, havc, qoyq
  tknk (41) -> ugml, padx, fwft
  jptl (61)
  ugml (68) -> gyxo, ebii, jptl
  gyxo (61)
  cntj (57)";
    let mut g: graph::Graph<Program> = graph::Graph::new();
    g.parse_nodes(program_input.as_bytes().lines()).unwrap();

    let node_expecterd = Program {
      name: "tknk".to_string(),
      weight: 41,
    };
    assert_eq!(g.root().map(|x| g.get_data(&x)), Some(&node_expecterd));
  }

  #[test]
  fn weigh_node_test() {
    let program_input = "pbga (66)
  xhth (57)
  ebii (61)
  havc (66)
  ktlj (57)
  fwft (72) -> ktlj, cntj, xhth
  qoyq (66)
  padx (45) -> pbga, havc, qoyq
  tknk (41) -> ugml, padx, fwft
  jptl (61)
  ugml (68) -> gyxo, ebii, jptl
  gyxo (61)
  cntj (57)";
    let mut g: graph::Graph<Program> = graph::Graph::new();
    g.parse_nodes(program_input.as_bytes().lines()).unwrap();

    assert_eq!(g.first().unwrap().weight(&g), 66);
    assert_eq!(g.first().unwrap().balanced(&g), true);

    let node = g.find_node("tknk").unwrap();
    assert_eq!(node.weight(&g), 778);
    assert_eq!(node.balanced(&g), false);
    assert_eq!(node.target_weight(&g), None);
    assert_eq!(node.badweight(&g), false);

    let bnode = g.find_node("ugml").unwrap();
    assert_eq!(bnode.badweight(&g), true);
    assert_eq!(bnode.fixed_weight(&g), Some(60));
  }

  #[test]
  fn parse_program_test() {
    let mut g: graph::Graph<Program> = graph::Graph::new();
    let text = "fwft (72) -> ktlj, cntj, xhth";
    let children_expected: Vec<String> = vec!["ktlj", "cntj", "xhth"]
      .iter()
      .map(|s| s.to_string())
      .collect();
    let (node, children) = g.parse_node(text).unwrap();
    // assert_eq!(prog, Program::new("fwft".to_string(), 72));
    assert_eq!(children, children_expected);

    let node_expected = Program {
      name: "fwft".to_string(),
      weight: 72,
    };
    assert_eq!(g.get_data(&node), &node_expected)
  }


}
