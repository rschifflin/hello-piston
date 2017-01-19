use specs::{Component, NullStorage};
#[derive(Default)]
pub struct AI;

impl Component for AI {
  type Storage = NullStorage<AI>;
}
