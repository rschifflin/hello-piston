use specs;

#[derive(Clone)]
pub struct Context {
  pub p1_paddle: specs::Entity,
  pub p2_paddle: specs::Entity,
  pub ball: specs::Entity,
}
