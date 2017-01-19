use components::control::player::Direction;
use components::phys::Velocity;
use float::Radians;
use specs::{System, RunArg, Join};
use systems::NamedSystem;
use context::Context;

pub struct Player;

impl System<Context> for Player {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (mut vels, players) = arg.fetch(|w| {
      let vels = w.write::<Velocity>();
      let players = w.read::<::components::control::Player>();
      (vels, players)
    });

    for (mut vel, player) in (&mut vels, &players).iter() {
      match player.direction {
        Direction::Up => {
          vel.angle = 90.0.deg_to_rad();
          vel.speed = 2.0;
        },
        Direction::Neutral => {
          vel.speed = 0.0;
        },
        Direction::Down => {
          vel.angle = 270.0.deg_to_rad();
          vel.speed = 2.0;
        }
      };
    }
  }
}

impl NamedSystem<Context> for Player {
  fn name(&self) -> &'static str {
    "ControlPlayer"
  }
}
