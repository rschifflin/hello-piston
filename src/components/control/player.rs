use specs::{Component, VecStorage};

#[derive(Copy, Clone)]
pub enum Direction {
  Up,
  Down,
  Neutral
}

pub struct Player {
  pub direction: Direction
}

impl Component for Player {
  type Storage = VecStorage<Player>;
}
