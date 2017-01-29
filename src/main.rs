#![feature(duration_checked_ops)]

extern crate piston;
extern crate float;
extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate specs;
extern crate cpal;
extern crate futures;
#[macro_use] extern crate conrod;

mod components;
mod systems;
mod colors;
mod sound;
mod screen;
mod world;
mod context;
mod ui;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use specs::{RunArg, Join, Entity, World};
use glium_graphics::{Glium2d, GliumGraphics, OpenGL, GliumWindow};
use glium::{Surface, Frame};
use glium::backend::Facade;
use context::Context;
use conrod::render::{Primitive, PrimitiveKind, PrimitiveWalker};
use futures::sync::mpsc::channel;
use sound::{SoundEvent, spawn_audio_thread};

pub struct App {
  ui: ui::Ui, //Conrod drawing context
  gl: Glium2d, // OpenGL drawing backend.
  context: Context, //Screen size, important entities
  planner: specs::Planner<Context>
}

impl App {
  fn render(&mut self, args: &RenderArgs, window: &mut glium_graphics::GliumWindow) {
    let world = self.planner.mut_world();
    let &mut ui::Ui {
      ref mut ui,
      ref mut primitives,
      ref mut text_texture_cache,
      ref mut glyph_cache,
      ref image_map,
      ..
    } = &mut self.ui;


    if let Some(mut new_primitives) = ui.draw_if_changed() {
      *primitives = Some(new_primitives.owned());
    }

    let mut frame = window.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    self.gl.draw(&mut frame, args.viewport(), |mut c, g| {
      if let &mut Some(ref ps) = primitives {
        conrod::backend::piston::draw::primitives(
          ps.walk(),
          c,
          g,
          text_texture_cache,
          glyph_cache,
          image_map,
          ui::Ui::cache_queued_glyphs,
          ui::Ui::texture_from_image
        );
      };
      Self::render_gfx(c, g, world);
    });

    frame.finish().unwrap();
  }

  fn render_gfx(c: graphics::Context, g: &mut GliumGraphics<Frame>, world: &mut World) {
    use graphics::{Graphics, ellipse, rectangle, Transformed};

    let pos = world.read::<components::phys::Position>();
    let shapes = world.read::<components::visual::Shape>();
    for (pos, shape) in (&pos, &shapes).iter() {
      let xform = c.transform.trans(pos.x, pos.y);
      match shape.shape {
        components::visual::ShapeType::Circle(circ) => {
          ellipse(shape.color, circ, xform, g);
        },
        components::visual::ShapeType::Rectangle(rect) => {
          rectangle(shape.color, rect, xform, g);
        },
      };
    };
  }

  fn input(&mut self, args: &Input) {
    self.ui.ui.handle_event(args.clone());

    use components::control::player::Direction;

    let Context { p1_paddle, p2_paddle, .. } = self.context.clone();

    let p1_direction = match *args {
      Input::Press(Button::Keyboard(keyboard::Key::S)) => Some(Direction::Down),
      Input::Press(Button::Keyboard(keyboard::Key::W)) => Some(Direction::Up),
      Input::Release(Button::Keyboard(keyboard::Key::S)) => Some(Direction::Neutral),
      Input::Release(Button::Keyboard(keyboard::Key::W)) => Some(Direction::Neutral),
      _ => None
    };

    let p2_direction = match *args {
      Input::Press(Button::Keyboard(keyboard::Key::Down)) => Some(Direction::Down),
      Input::Press(Button::Keyboard(keyboard::Key::Up)) => Some(Direction::Up),
      Input::Release(Button::Keyboard(keyboard::Key::Down)) => Some(Direction::Neutral),
      Input::Release(Button::Keyboard(keyboard::Key::Up)) => Some(Direction::Neutral),
      _ => None
    };

    self.planner.run_custom(move |arg: RunArg| {
      let mut players = arg.fetch(|w| {
        w.write::<::components::control::Player>()
      });

      p1_direction.and_then(|dir| {
        players.get_mut(p1_paddle).map(|mut p1| {
          p1.direction = dir;
        })
      });

      p2_direction.and_then(|dir| {
        players.get_mut(p2_paddle).map(|mut p2| {
          p2.direction = dir;
        })
      });
    })
  }

  fn update(&mut self, _: &UpdateArgs) {
    self.planner.dispatch(self.context.clone());
  }
}

fn main() {
  let (sound_tx, sound_rx) = channel::<SoundEvent>(0);
  spawn_audio_thread(sound_rx);

  let opengl = OpenGL::V3_2;
  let mut window: GliumWindow = WindowSettings::new(
    "pawng",
    [screen::WIDTH, screen::HEIGHT]
  )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

  // Create a new game and run it.
  let mut world = specs::World::new();
  world::register(&mut world);
  let (p1, p2, ball) = world::create_initial_entities(&mut world);
  let context = Context {
    p1_paddle: p1,
    p2_paddle: p2,
    ball: ball,
    sound_tx: sound_tx
  };
  let mut planner = specs::Planner::<Context>::new(world, 4);
  systems::plan_system(&mut planner, systems::Physics, 0);
  systems::plan_system(&mut planner, systems::control::Player, 0);
  let mut ui = ui::Ui::new(&mut window);
  ui.update();

  let mut app = App {
    ui: ui,
    gl: Glium2d::new(opengl, &window),
    context: context,
    planner: planner,
  };

  while let Some(e) = window.next() {
    if let &Event::Input(ref i) = &e {
      app.input(i);
    }
    e.update(|args| app.update(&args));
    e.render(|args| {
      app.render(&args, &mut window);
    });
  }
}
