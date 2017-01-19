use graphics::types::Rectangle;
use specs::{Component, VecStorage};

pub enum ShapeType {
  Circle(Rectangle),
  Rectangle(Rectangle)
}

pub struct Shape {
  pub shape: ShapeType,
  pub color: [f32; 4]
}

impl Component for Shape {
  type Storage = VecStorage<Shape>;
}
