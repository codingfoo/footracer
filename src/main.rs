use std::ops::Add;

struct Tuple {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}

impl Tuple {
  fn is_point(&self) -> bool {
    self.w == 1.0
  }
  fn is_vector(&self) -> bool {
    self.w == 0.0
  }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple{ x:self.x+other.x, y: self.y+other.y, z: self.z+other.z, w:self.w+other.w }
    }
}

fn main() {
    println!("Ray Tracing!");
}

#[test]
fn test_add_two_tuples() {
  let t1 = Tuple {x:3.0, y:-2.0, z:5.0, w:1.0};
  let t2 = Tuple {x:-2.0, y:3.0, z:1.0, w:0.0};
  assert!(t1+t2==Tuple{x:1.0, y:1.0, z:6.0, w:1.0});
}

#[test]
fn test_tuple_is_a_point() {
  let t = Tuple {x:4.0, y:-4.0, z:3.0, w:1.0};
  assert!(t.x == 4.0);
  assert!(t.y == -4.0);
  assert!(t.z == 3.0);
  assert!(t.w == 1.0);
  assert!(t.is_point() == true);
  assert!(t.is_vector() == false);
}

#[test]
fn test_tuple_is_a_vector() {
  let t = Tuple {x:4.0, y:-4.0, z:3.0, w:0.0};
  assert!(t.x == 4.0);
  assert!(t.y == -4.0);
  assert!(t.z == 3.0);
  assert!(t.w == 0.0);
  assert!(t.is_point() == false);
  assert!(t.is_vector() == true);
}
