use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Velocity {
  pub angle: f64, //rads
  pub speed: f64
}

impl Component for Velocity {
  type Storage = VecStorage<Velocity>;
}
