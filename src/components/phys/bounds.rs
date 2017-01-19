use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Bounds {
  pub w: f64,
  pub h: f64
}

impl Component for Bounds {
  type Storage = VecStorage<Bounds>;
}
