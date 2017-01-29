use components::phys::{Position, Velocity, Bounds};
use specs::{Allocator, System, RunArg, Join, Storage, MaskedStorage};
use systems::NamedSystem;
use float::Radians;
use std::ops::Neg;
use screen;
use sound::SoundEvent;
use context::Context;
use futures::{Future, Sink};

pub struct Physics;

impl System<Context> for Physics {
  fn run(&mut self, arg: RunArg, context: Context) {
    let (mut s_pos, mut s_vel, s_bounds) = arg.fetch(|w| {
      let s_pos = w.write::<Position>();
      let s_vel = w.write::<Velocity>();
      let s_bounds = w.read::<Bounds>();
      (s_pos, s_vel, s_bounds)
    });

    type JType<'a> = (
      &'a mut Storage<Position, ::std::sync::RwLockReadGuard<'a, Allocator>, ::std::sync::RwLockWriteGuard<'a, MaskedStorage<Position>>>,
      &'a mut Storage<Velocity, ::std::sync::RwLockReadGuard<'a, Allocator>, ::std::sync::RwLockWriteGuard<'a, MaskedStorage<Velocity>>>,
      &'a Storage<Bounds, ::std::sync::RwLockReadGuard<'a, Allocator>, ::std::sync::RwLockReadGuard<'a, MaskedStorage<Bounds>>>
    );
    let (_, mut join_values) = (&mut s_pos, &mut s_vel, &s_bounds).open();

    unsafe {
      let (ref mut p1_pos, ref mut p1_vel, p1_bounds) = <JType as Join>::get(&mut join_values, context.p1_paddle.get_id());
      let (ref mut p2_pos, ref mut p2_vel, p2_bounds) = <JType as Join>::get(&mut join_values, context.p2_paddle.get_id());
      let (ref mut ball_pos, ref mut ball_vel, ball_bounds) = <JType as Join>::get(&mut join_values, context.ball.get_id());

      update_pos(p1_pos, p1_vel);
      update_pos(p2_pos, p2_vel);
      clamp_pos(p1_pos, p1_bounds);
      clamp_pos(p2_pos, p2_bounds);
      update_pos(ball_pos, ball_vel);

      if is_oob_y(ball_pos, ball_bounds) {
        ball_vel.angle = ball_vel.angle.neg();
      }
      if is_oob_x(ball_pos, ball_bounds) {
        ball_pos.x = screen::WIDTH as f64/2.0;
        ball_pos.y = screen::HEIGHT as f64/2.0;
        ball_vel.angle = (ball_vel.angle.signum() * 10.0).deg_to_rad();
        ball_vel.speed = 4.0;
      } else if is_colliding(ball_pos, ball_bounds, p1_pos, p1_bounds) {
        context.sound_tx.send(SoundEvent::Beep).wait().map_err(|e| println!("error emitting beep: {:?}", e));
        if is_upper_corner(ball_pos, p1_pos) {
          let corner_ratio = (p1_pos.y - ball_pos.y) / ball_bounds.h;
          ball_vel.angle = (45.0 + (30.0 * corner_ratio)).deg_to_rad();
          ball_vel.speed += 3.0 * corner_ratio;
        } else if is_lower_corner(ball_pos, ball_bounds, p1_pos, p1_bounds) {
          let corner_ratio = ((ball_pos.y + ball_bounds.h) - (p1_pos.y + p1_bounds.h)) / ball_bounds.h;
          ball_vel.angle = (-45.0 + (30.0 * corner_ratio)).deg_to_rad();
          ball_vel.speed += 3.0 * corner_ratio;
        } else {
          ball_vel.angle += 90.0.deg_to_rad();
          ball_vel.angle = ball_vel.angle.neg();
          ball_vel.angle -= 90.0.deg_to_rad();
        }
        ball_pos.x = p1_pos.x + p1_bounds.w;
      } else if is_colliding(ball_pos, ball_bounds, p2_pos, p2_bounds) {
        context.sound_tx.send(SoundEvent::Boop).wait().map_err(|e| println!("error emitting boop: {:?}", e));
        if is_upper_corner(ball_pos, p2_pos) {
          let corner_ratio = (p2_pos.y - ball_pos.y) / ball_bounds.h;
          ball_vel.angle = (135.0 - (30.0 * corner_ratio)).deg_to_rad();
          ball_vel.speed += 3.0 * corner_ratio;
        } else if is_lower_corner(ball_pos, ball_bounds, p2_pos, p2_bounds) {
          let corner_ratio = ((ball_pos.y + ball_bounds.h) - (p2_pos.y + p2_bounds.h)) / ball_bounds.h;
          ball_vel.angle = (-135.0 + (30.0 * corner_ratio)).deg_to_rad();
          ball_vel.speed += 3.0 * corner_ratio;
        } else {
          ball_vel.angle += 90.0.deg_to_rad();
          ball_vel.angle = ball_vel.angle.neg();
          ball_vel.angle -= 90.0.deg_to_rad();
        }
        ball_pos.x = p2_pos.x - ball_bounds.w;
      }

      clamp_pos(ball_pos, ball_bounds);
    }
  }
}

impl NamedSystem<Context> for Physics {
  fn name(&self) -> &'static str {
    "physics"
  }
}

fn update_pos(pos: &mut Position, vel: &mut Velocity) {
  pos.x += vel.speed * vel.angle.cos();
  pos.y -= vel.speed * vel.angle.sin();
}

fn clamp_pos(pos: &mut Position, bounds: &Bounds) {
  pos.x = pos.x.min(screen::WIDTH as f64 - bounds.w);
  pos.x = pos.x.max(0.0);
  pos.y = pos.y.min(screen::HEIGHT as f64 - bounds.h);
  pos.y = pos.y.max(0.0);
}

fn is_oob_x(ball_pos: &Position, ball_bounds: &Bounds) -> bool {
  ball_pos.x < 0.0 || ball_pos.x + ball_bounds.w > screen::WIDTH as f64
}

fn is_oob_y(ball_pos: &Position, ball_bounds: &Bounds) -> bool {
  ball_pos.y < 0.0 || ball_pos.y + ball_bounds.h > screen::HEIGHT as f64
}

fn is_colliding(first_pos: &Position, first_bounds: &Bounds, second_pos: &Position, second_bounds: &Bounds) -> bool {
  (first_pos.x < second_pos.x + second_bounds.w && first_pos.x + first_bounds.w > second_pos.x) &&
  (first_pos.y < second_pos.y + second_bounds.h && first_pos.y + first_bounds.h > second_pos.y)
}

fn is_upper_corner(ball_pos: &Position, paddle_pos: &Position) -> bool {
  ball_pos.y < paddle_pos.y
}

fn is_lower_corner(ball_pos: &Position, ball_bounds: &Bounds, paddle_pos: &Position, paddle_bounds: &Bounds) -> bool {
  ball_pos.y + ball_bounds.h > paddle_pos.y + paddle_bounds.h
}
