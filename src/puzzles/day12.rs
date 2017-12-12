
use super::super::graph;

impl graph::Graph<String> {
  
  pub fn has_node(&self, name: &str) -> bool {
    self.iter().any(|x| { self.get_data(&x) == name })
  }
  
  pub fn find_node(&self, name: &str) -> Option<graph::Node> {
    self.iter().find(|x| { self.get_data(&x) == name})
  }
  
  pub fn parse_node(&mut self, line: &str) -> Result<(), graph::GraphError> {
    let mut parts = line.split("<->");
    let name = parts.next().unwrap().trim();
    
    let node = match self.find_node(&name) {
      Some(n) => n,
      None => self.new_node(name.to_string())
    };
    
    let children = parts.next().unwrap().split(',');
    for childname in children {
      let child = self.new_node(childname.trim().to_string());
      node.append(&child, self)?;
    }
    
    Ok(())
  }
  
}


#[cfg(test)]
mod tests {
  
  use super::*;
  
  #[test]
  fn try_parse_nodes() {
    let mut g : graph::Graph<String> = graph::Graph::new();
    g.parse_node("0 <-> 2").unwrap();
    assert_eq!(g.len(), 2);
  }
  
}