extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

struct Game {
    gl: GlGraphics,
    player: Player,
    enemie: Enemie,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            clear(WHITE, gl);
        });

        self.player.render(&mut self.gl, arg);
        self.enemie.render(&mut self.gl, arg);
    }

    fn update(&mut self) -> bool {
        self.player.update();
        if !self.enemie.update(&mut self.player) {
            return false;
        }
        true
    }

    fn input(&mut self, btn: &Button) {
        if btn == &Button::Keyboard(Key::Up) || btn == &Button::Keyboard(Key::W) {
            self.player.dir = Direction::UP;
        }
        if btn == &Button::Keyboard(Key::Down) || btn == &Button::Keyboard(Key::S) {
            self.player.dir = Direction::DOWN;
        }
        if btn == &Button::Keyboard(Key::Right) || btn == &Button::Keyboard(Key::D) {
            self.player.dir = Direction::RIGHT;
        } else if btn == &Button::Keyboard(Key::Left) || btn == &Button::Keyboard(Key::A) {
            self.player.dir = Direction::LEFT;
        }
    }
}

struct Player {
    x: u32,
    y: u32,
    dir: Direction,
}
impl Player {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let sq =
            graphics::rectangle::square(f64::from(self.x * 20), f64::from(self.y * 20), 20_f64);
        gl.draw(args.viewport(), |c, gl| {
            let trans = c.transform;
            graphics::rectangle(BLACK, sq, trans, gl);
        });
    }
    fn update(&mut self) {
        match self.dir {
            Direction::RIGHT => self.x += 1,
            Direction::LEFT => self.x -= 1,
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
        }
    }
}

struct Enemie {
    x: u32,
    y: u32,
}
impl Enemie {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        //makes a square to be drawn
        let square =
            graphics::rectangle::square(f64::from(self.x * 20), f64::from(self.y * 20), 20_f64);
        //draws the enemie on the window
        gl.draw(args.viewport(), |c, gl| {
            let trans = c.transform;
            graphics::rectangle(RED, square, trans, gl);
        });
    }
    fn update(&mut self, p: &Player) -> bool {
        if p.x == self.x && p.y == self.y {
            true
        } else {
            false
        }
    }
}
fn main() {
    let opengl = OpenGL::V3_2;
    //this is the setup of the window size and title
    let mut window: Window = WindowSettings::new("Mazemenia", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        player: Player {
            //starting position of snake
            x: 0,
            y: 0,
            //the direction that it starts moving
            dir: Direction::RIGHT,
        },
        enemie: Enemie {
            //starting postion of the enemie
            x: 10,
            y: 10,
        },
    };

    let mut events = Events::new(EventSettings::new().ups(4));
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(_u) = e.update_args() {
            if game.update() {
                println!("Game Over");
		println!("enemie found you");
                break;
            }
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.input(&k.button);
            }
        }
    }
}
