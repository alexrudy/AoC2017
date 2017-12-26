use std::str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
  x: i32,
  y: i32,
  z: i32,
}

impl Point {
  fn origin() -> Point {
    Point { x: 0, y: 0, z: 0 }
  }

  fn distance(&self, other: &Point) -> u32 {
    ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u32
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Particle {
  p: Point,
  v: Point,
  a: Point,
}

type ParseResult<T> = Result<T, <i32 as str::FromStr>::Err>;

impl Particle {
  pub fn distance(&self) -> u32 {
    return self.p.distance(&Point::origin());
  }

  fn advance(&mut self) {
    self.v.x += self.a.x;
    self.v.y += self.a.y;
    self.v.z += self.a.z;
    self.p.x += self.v.x;
    self.p.y += self.v.y;
    self.p.z += self.v.z;
  }

  pub fn settled(&self) -> bool {
    let xps = self.p.x.signum();
    let yps = self.p.y.signum();
    let zps = self.p.z.signum();
    let xvs = self.v.x.signum();
    let yvs = self.v.y.signum();
    let zvs = self.v.z.signum();
    let xas = self.a.x.signum();
    let yas = self.a.y.signum();
    let zas = self.a.z.signum();

    xps * xvs >= 0 && xvs * xas >= 0 && xps * xas >= 0 && yps * yvs >= 0 && yvs * yas >= 0
      && yps * yas >= 0 && zps * zvs >= 0 && zvs * zas >= 0 && zps * zas >= 0
  }

  pub fn parse(text: &str) -> ParseResult<Particle> {
    let mut vectors = text
      .split(">")
      .map(|x| parse_vector(x.trim_left_matches(",").trim()));

    let (_p, pv) = vectors.next().unwrap()?;
    let (_v, vv) = vectors.next().unwrap()?;
    let (_a, av) = vectors.next().unwrap()?;

    Ok(Particle {
      p: pv,
      v: vv,
      a: av,
    })
  }
}

pub struct ParticleSimulationIterator<'a> {
  particles: Vec<Particle>,
  step: usize,
  check: Option<&'a Fn(&[Particle]) -> bool>,
}

impl<'a> Iterator for ParticleSimulationIterator<'a> {
  type Item = (usize, Vec<Particle>);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      for particle in self.particles.iter_mut() {
        particle.advance();
      }
      self.step += 1;
      if let Some(check) = self.check {
        if check(&self.particles) {
          return Some((self.step, self.particles.clone()));
        }
      } else {
        return Some((self.step, self.particles.clone()));
      }
    }
  }
}

pub fn simulate<'a>(
  particles: Vec<Particle>,
  check: Option<&'a Fn(&[Particle]) -> bool>,
) -> ParticleSimulationIterator<'a> {
  ParticleSimulationIterator {
    particles: particles,
    step: 0,
    check: check,
  }
}

fn parse_vector(text: &str) -> ParseResult<(char, Point)> {
  let kind = text
    .chars()
    .nth(0)
    .expect("Vector stirng has no characters!");
  let vec: Vec<i32> = text.trim()[3..]
    .split(",")
    .map(|x| x.trim().parse::<i32>())
    .collect::<ParseResult<Vec<i32>>>()?;
  let point = Point {
    x: vec[0],
    y: vec[1],
    z: vec[2],
  };
  Ok((kind, point))
}

pub fn find_closest(particles: Vec<Particle>) -> usize {
  let (_n, pf) = simulate(
    particles,
    Some(&|p: &[Particle]| -> bool { p.iter().all(|pi| pi.settled()) }),
  ).nth(0)
    .unwrap();
  let (id, _pm) = pf.iter()
    .enumerate()
    .min_by_key(|&(_i, pi)| pi.distance())
    .unwrap();
  id
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parsing() {
    let p = Particle::parse(&"p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>").unwrap();
    let ep = Particle {
      p: Point { x: 3, y: 0, z: 0 },
      v: Point { x: 2, y: 0, z: 0 },
      a: Point { x: -1, y: 0, z: 0 },
    };
    assert_eq!(p, ep);
  }

  #[test]
  fn test_settled() {
    let p1 = Particle::parse(&"p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>").unwrap();
    assert!(!p1.settled());
    let p2 = Particle::parse(&"p=< 4,0,0>, v=< 0,0,0>, a=< 2,0,0>").unwrap();
    assert!(p2.settled());
  }

  #[test]
  fn test_simulate() {
    let mut ps = Vec::new();
    ps.push(Particle::parse(&"p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>").unwrap());
    ps.push(Particle::parse(&"p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>").unwrap());
    let (n, pf) = simulate(
      ps.clone(),
      Some(&|p: &[Particle]| -> bool { p.iter().all(|pi| pi.settled()) }),
    ).nth(0)
      .unwrap();
    let (id, _pm) = pf.iter()
      .enumerate()
      .min_by_key(|&(_i, pi)| pi.distance())
      .unwrap();
    assert_eq!(id, 0);
    assert_eq!(n, 5);
    assert_eq!(find_closest(ps.clone()), 0)
  }
}
