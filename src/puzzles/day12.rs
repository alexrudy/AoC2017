use super::super::graph;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Town {
  name: String,
}

impl From<String> for Town {
  fn from(name: String) -> Town {
    Town { name: name }
  }
}

impl graph::Graph<Town> {
  pub fn has_node(&self, name: &str) -> bool {
    self.iter().any(|x| self.get_data(&x).name == name)
  }

  pub fn find_node(&self, name: &str) -> Option<graph::Node> {
    self.iter().find(|x| self.get_data(&x).name == name)
  }

  pub fn parse_node(&mut self, line: &str) -> Result<(), graph::GraphError> {
    let mut parts = line.split("<->");
    let name = parts.next().unwrap().trim();

    let node = match self.find_node(&name) {
      Some(n) => n,
      None => self.node(Town {
        name: name.to_string(),
      }),
    };

    let children = parts.next().unwrap().split(',');
    for childname in children {
      let child = match self.find_node(childname.trim()) {
        Some(n) => n,
        None => self.node(Town {
          name: childname.trim().to_string(),
        }),
      };
      node.append(&child, self)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::io::BufRead;

  #[test]
  fn try_parse_nodes() {
    let mut g: graph::Graph<Town> = graph::Graph::new();
    g.parse_node("0 <-> 2").unwrap();
    assert_eq!(g.len(), 2);
  }

  #[test]
  fn try_search() {
    let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    let mut g: graph::Graph<Town> = graph::Graph::new();
    for line in input.as_bytes().lines() {
      g.parse_node(&line.unwrap()).unwrap();
    }
    let root = g.find_node("0").unwrap();

    let nodes: Vec<String> = root
      .connected(&g)
      .map(|x| g.get_data(&x).name.clone())
      .collect();
    println!("{:?}", nodes);

    assert_eq!(root.connected(&g).count(), 6)
  }

}
