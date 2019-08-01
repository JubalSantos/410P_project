use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::player::{Player, Direction};
use crate::player::Enemie;

pub struct Game {
	pub gl: GlGraphics,
	pub player: Player,
	pub enemie: Enemie,
}

impl Game {
//generates a white gameboard
    pub fn render(&mut self, arg: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            clear(WHITE, gl);
        });
	//generates the player and enemie in the game
        self.player.render(&mut self.gl, arg);
        self.enemie.render(&mut self.gl, arg);
    }

    pub fn update(&mut self) -> bool {
        self.player.update();
        if !self.enemie.update(&mut self.player) {
            return false;
        }
        true
    }

    pub fn input(&mut self, btn: &Button) {
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
