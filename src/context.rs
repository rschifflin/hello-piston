use futures::sync::mpsc::Sender;
use specs;
use sound::SoundEvent;

#[derive(Clone)]
pub struct Context {
  pub p1_paddle: specs::Entity,
  pub p2_paddle: specs::Entity,
  pub ball: specs::Entity,
  pub sound_tx: Sender<SoundEvent>
}
