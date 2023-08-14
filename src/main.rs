extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, ButtonState, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent,
};
use piston::window::WindowSettings;
use piston::{ButtonEvent, CursorEvent};
use std::process;

struct Coordinate {
    x: f64,
    y: f64,
}
impl Coordinate {
    fn add(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }
    fn multiply(&mut self, other: Coordinate) {
        self.x *= other.x;
        self.y *= other.y;
    }
    fn multiply_by_scalar(&mut self, scalar: f64) -> Coordinate {
        return Coordinate {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }
    fn opposite(&mut self) -> Coordinate {
        return Coordinate {
            x: -self.x,
            y: -self.y,
        };
    }
    fn magnitude(&mut self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
    fn unit_size(&mut self) -> Coordinate {
        let mut dv = Coordinate {
            x: self.x,
            y: self.y,
        };
        let length = 1.0 / dv.magnitude();
        dv.multiply_by_scalar(length);
        return dv;
    }
}
pub struct Game {
    gl: GlGraphics,
    character: Character,
    controls: Controls,
    gravity: f64,
}
struct Controls {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
pub struct Character {
    coordinates: Coordinate,
    velocity: Coordinate,
    acceleration: Coordinate,
    friction: f64,
    mass: f64,
}

impl Character {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const CHARACTER_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let char = rectangle::square(0.0, 0.0, 100.0);

        let x = self.coordinates.x;
        let y = self.coordinates.y;

        gl.draw(args.viewport(), |c, gl| {
            rectangle(CHARACTER_COLOR, char, c.transform.trans(x, y), gl);
        });
    }
    fn update(&mut self, args: &UpdateArgs, controls: &mut Controls, gravity: f64) {
        if controls.up {
            self.acceleration.y -= 1.0;
        }
        if controls.down {
            self.acceleration.y += 1.0;
        }
        if controls.left {
            self.acceleration.x -= 1.0;
        }
        if controls.right {
            self.acceleration.x += 1.0;
        }

        self.velocity
            .add(self.acceleration.multiply_by_scalar(args.dt));

        self.coordinates
            .add(self.velocity.multiply_by_scalar(args.dt));

        let mut vel_opposite_unit = self.velocity.opposite().unit_size();
        let force_friction =
            vel_opposite_unit.multiply_by_scalar(gravity * self.mass * self.friction);
        println!("{:?}", force_friction.x);
        self.acceleration.add(force_friction);
    }
}
impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
        });
        self.character.render(&mut self.gl, args);
    }
    fn update(&mut self, args: &UpdateArgs) {
        self.character
            .update(args, &mut self.controls, self.gravity);
    }
    fn pressed(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::Up) => self.controls.up = true,
            Button::Keyboard(Key::Down) => self.controls.down = true,
            Button::Keyboard(Key::Left) => self.controls.left = true,
            Button::Keyboard(Key::Right) => self.controls.right = true,
            Button::Keyboard(Key::Escape) => process::exit(0),
            _ => {}
        }
    }
    fn released(&mut self, button: &Button) {
        match button {
            Button::Keyboard(Key::Up) => self.controls.up = false,
            Button::Keyboard(Key::Down) => self.controls.down = false,
            Button::Keyboard(Key::Left) => self.controls.left = false,
            Button::Keyboard(Key::Right) => self.controls.right = false,
            _ => {}
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("test", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        character: Character {
            coordinates: Coordinate {
                x: window.window.inner_size().width as f64 / 2.0,
                y: window.window.inner_size().height as f64 / 2.0,
            },
            velocity: Coordinate { x: 0.0, y: 0.0 },
            acceleration: Coordinate { x: 0.0, y: 0.0 },
            friction: 0.2,
            mass: 0.01,
        },
        controls: Controls {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        gravity: 9.8,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
        if let Some(args) = e.update_args() {
            game.update(&args);
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
            if k.state == ButtonState::Release {
                game.released(&k.button);
            }
        }
    }
}
