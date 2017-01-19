use components::phys::{Position, Velocity, Bounds};
use components::visual::{Shape, ShapeType};
use components::control::{Player, AI};
use components::control::player::Direction;
use specs::World;
use float::Radians;
use context::Context;
use screen;
use graphics;
use colors;

const DELTA: f64 = 1.0;
pub fn register(world: &mut World) {
  world.register::<Player>();
  world.register::<AI>();


  world.register::<Bounds>();
  world.register::<Position>();
  world.register::<Velocity>();
  world.register::<Shape>();
}

pub fn create_initial_entities(world: &mut World) -> Context {
  let p1 = world.create_now() // Player paddle
    .with::<Player>(Player { direction: Direction::Neutral })
    .with::<Velocity>(Velocity { angle: 270.0.deg_to_rad(), speed: 0.0 })
    .with::<Position>(Position { x: 0.0, y: 0.0 })
    .with::<Bounds>(Bounds { w: 25.0, h: 100.0 })
    .with::<Shape>(Shape {
      shape: ShapeType::Rectangle([0.0, 0.0, 25.0, 100.0]),
      color: colors::WHITE
    })
    .build();

  let p2 = world.create_now() // AI Paddle
    .with::<Player>(Player { direction: Direction::Neutral })
    .with::<Velocity>(Velocity { angle: 0.0, speed: 0.0 })
    .with::<Position>(Position { x: (screen::WIDTH - 25) as f64, y: 0.0 })
    .with::<Bounds>(Bounds { w: 25.0, h: 100.0 })
    .with::<Shape>(Shape {
      shape: ShapeType::Rectangle([0.0, 0.0, 25.0, 100.0]),
      color: colors::WHITE
    })
    .build();

  let ball = world.create_now() //Ball
    .with::<Velocity>(Velocity { angle: 45.0.deg_to_rad(), speed: 4.0 })
    .with::<Position>(Position { x: (screen::WIDTH/2) as f64, y: (screen::HEIGHT/2) as f64})
    .with::<Bounds>(Bounds { w: 20.0, h: 20.0 })
    .with::<Shape>(Shape {
      shape: ShapeType::Circle(graphics::ellipse::circle(10.0, 10.0, 10.0)),
      color: colors::WHITE
    })
    .build();

  Context {
    p1_paddle: p1,
    p2_paddle: p2,
    ball: ball
  }
}
