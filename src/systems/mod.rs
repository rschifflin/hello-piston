use specs::{Planner, System};
pub mod control;
mod physics;

pub trait NamedSystem<C>: System<C> {
  fn name(&self) -> &'static str;
}

pub fn plan_system<C: 'static, S: 'static + NamedSystem<C>>(planner: &mut Planner<C>, system: S, priority: i32) {
  let name = system.name();
  planner.add_system(system, name, priority);
}

pub use self::physics::Physics;
