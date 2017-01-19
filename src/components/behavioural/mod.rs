use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Paddle;

#[derive(Default)]
pub struct Ball;

impl Component for Paddle {
  type Storage = NullStorage<Paddle>;
}

impl Component for Ball {
  type Storage = NullStorage<Ball>;
}
