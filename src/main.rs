extern crate piston;
extern crate float;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate specs;

mod components;
mod systems;
mod colors;
mod screen;
mod world;
mod context;

use components::phys::{Position, Bounds, Velocity};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use specs::{RunArg, Join};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use context::Context;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    context: Context, //Screen size, important entities
    planner: specs::Planner<Context>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let world = self.planner.mut_world();
        let pos = world.read::<components::phys::Position>();
        let shapes = world.read::<components::visual::Shape>();

        self.gl.draw(args.viewport(), |c, gl| {
          clear(colors::BLACK, gl);
          for (pos, shape) in (&pos, &shapes).iter() {
            let xform = c.transform.trans(pos.x, pos.y);
            match shape.shape {
              components::visual::ShapeType::Circle(circ) => {
                ellipse(shape.color, circ, xform, gl);
              },
              components::visual::ShapeType::Rectangle(rect) => {
                rectangle(shape.color, rect, xform, gl);
              },
            };
          };
        });
    }

    fn input(&mut self, args: &Input) {
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

    fn update(&mut self, args: &UpdateArgs) {
      self.planner.dispatch(self.context.clone());
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
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
    let context = world::create_initial_entities(&mut world);
    let mut planner = specs::Planner::<Context>::new(world, 4);
    systems::plan_system(&mut planner, systems::Physics, 0);
    systems::plan_system(&mut planner, systems::control::Player, 0);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        context: context,
        planner: planner
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
      if let &Event::Input(ref i) = &e { app.input(i); }
      e.update(|args| app.update(&args));
      e.render(|args| app.render(&args));
    }
}
