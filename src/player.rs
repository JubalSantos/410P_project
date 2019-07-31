extern crate piston;
extern crate opengl_graphics;

use piston::input::*;
use opengl_graphics::GlGraphics;

pub enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

pub struct Player {
    pub x: u32,
    pub y: u32,
    pub dir: Direction,
}

impl Player {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let sq =
            graphics::rectangle::square(f64::from(self.x * 20), f64::from(self.y * 20), 20_f64);
        gl.draw(args.viewport(), |c, gl| {
            let trans = c.transform;
            graphics::rectangle(BLACK, sq, trans, gl);
        });

    }

    pub fn update(&mut self) {
        match self.dir {
            Direction::RIGHT => self.x += 1,
            Direction::LEFT => self.x -= 1,
            Direction::UP => self.y -= 1,
            Direction::DOWN => self.y += 1,
        }
    }
}

pub struct Enemie {
    pub x: u32,
    pub y: u32,
}

impl Enemie {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
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

    pub fn update(&mut self, p: &Player) -> bool {
        if p.x == self.x && p.y == self.y {
            true
        } else {
            false
        }
    }
}
