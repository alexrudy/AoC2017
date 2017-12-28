use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Connector(usize, usize);

impl Connector {
  
  fn new(a: usize, b: usize) -> Self {
    Connector(a, b)
  }
  
  fn other(&self, pins: usize) -> usize {
    if pins == self.0 {
      self.1
    } else {
      self.0
    }
  }
  
  fn iter(&self) -> ConnectorIterator {
    ConnectorIterator {
      connector: self,
      position: 0
    }
  }
}

impl PartialEq for Connector {
  fn eq(&self, other: &Connector) -> bool {
    self.0 == other.0 && self.1 == other.1 ||
    self.0 == other.1 && self.1 == other.0
  }
}

impl PartialEq<(usize, usize)> for Connector {
  fn eq(&self, other: &(usize, usize)) -> bool {
    self.0 == other.0 && self.1 == other.1 ||
    self.0 == other.1 && self.1 == other.0
  }
}

impl FromStr for Connector {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s.trim()
      .split('/')
      .map(|p| p.parse::<usize>())
      .collect::<Result<Vec<_>, _>>()?;
    
    Ok(Connector(parts[0], parts[1]))
  }
}

impl From<(usize, usize)> for Connector {
  fn from(parts: (usize, usize)) -> Self {
    Self::new(parts.0, parts.1)
  }
}

struct ConnectorIterator<'a> {
  connector: &'a Connector,
  position: usize,
}

impl<'a> Iterator for ConnectorIterator<'a> {
  type Item = usize;
  
  fn next(&mut self) -> Option<usize> {
    let result = match self.position {
      0 => Some(self.connector.0),
      1 => Some(self.connector.1),
      _ => None,
    };
    self.position += 1;
    result
  }
}

pub trait Bridge {
  fn contains_connector(&self, connector: &Connector) -> bool;
  
  fn strength(&self) -> usize;
}

impl Bridge for VecDeque<Connector> {
  
  fn contains_connector(&self, connector: &Connector) -> bool {
    self.contains(&connector)
  }
  
  fn strength(&self) -> usize {
    self.iter().map(|c| c.0 + c.1).sum()
  }
}

fn connector_mapping(connectors: &[Connector]) -> HashMap<usize, Vec<Connector>> {
  let mut connector_mapping = HashMap::new();
  for connector in connectors {
    for part in connector.iter() {
      let v = connector_mapping.entry(part).or_insert(Vec::new());
      (*v).push((part, connector.other(part)).into());
    }
  }
  connector_mapping
}

pub fn bridges(connectors: &[Connector]) -> BridgeIterator {
  let mut bridges = VecDeque::new();
  bridges.push_front(VecDeque::new());
  
  BridgeIterator {
    bridges : bridges,
    connectors: connector_mapping(connectors),
  }
}

pub struct BridgeIterator {
  bridges: VecDeque<VecDeque<Connector>>,
  connectors : HashMap<usize, Vec<Connector>>
}

impl Iterator for BridgeIterator {
  
  type Item = VecDeque<Connector>;
  
  
  fn next(&mut self) -> Option<VecDeque<Connector>> {
    let mut bridge;
    loop {
      bridge = self.bridges.pop_back()?;
      let target = bridge.back().map(|c| c.1).unwrap_or(0);
      let n = self.bridges.len();
      let connections = self.connectors.entry(target).or_insert(Vec::new());
      for connector in connections {
        if !bridge.contains_connector(connector) {
          let mut new_bridge = bridge.clone();
          new_bridge.push_back(*connector);
          self.bridges.push_front(new_bridge);
        }
      }
      
      // We didn't generate any more bridges, this must
      // be a terminal bridge, which we should yield.
      if n == self.bridges.len() {
        break
      }
    }
    Some(bridge)
  }
}

#[cfg(test)]
mod test {

  use super::*;
  
  const COMPONENTS: &str = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10";

  #[test]
  fn parse_connectors() {
    let c: Connector = "0/4".parse().unwrap();
    assert_eq!(c, Connector(0, 4));

    let c: Connector = "4/0".parse().unwrap();
    assert_eq!(c, Connector(0, 4));

    let c: Connector = "52/20".parse().unwrap();
    assert_eq!(c, Connector(20, 52));
  }
  
  #[test]
  fn connector_behavior() {
    let c: Connector = "4/0".parse().unwrap();
    assert_eq!(c.iter().collect::<Vec<_>>(), vec![4, 0]);
  }
  
  #[test]
  fn iterate_bridges() {
    let cs : Vec<Connector> = COMPONENTS.lines().map(|l| l.parse()).collect::<Result<_,_>>().unwrap();
    
    let b = bridges(&cs);
    assert_eq!(b.count(), 7);
    
    let b = bridges(&cs);
    let strongest_bridge = b.max_by_key(|b| b.strength()).unwrap();
    assert_eq!(strongest_bridge.strength(), 31);
    assert_eq!(strongest_bridge, vec![(0, 1), (1, 10), (10, 9)]);
  }
  use test::Bencher;
  
  #[bench]
  fn iterate_bridges_bench(b: &mut Bencher) {
    let cs : Vec<Connector> = COMPONENTS.lines().map(|l| l.parse()).collect::<Result<_,_>>().unwrap();
    b.iter(|| {bridges(&cs).map(|b| b.strength()).max();});
  }

}
