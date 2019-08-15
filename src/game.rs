use opengl_graphics::GlGraphics;
use piston::input::*;
use std;
use std::cell::RefCell;
use std::rc::Rc;

use crate::maze::Maze;
use crate::player::Enemie;
use crate::player::{Direction, Player};

const SIZE_X = 21;
const SIZE_Y = 21;

pub struct Game {
    pub gl: GlGraphics,
    pub maze: Maze,
    pub player: Player,
    pub enemie: Enemie,
}

impl Game {
    pub fn new(gl: GlGraphics) -> Game {
        let maze = Maze::gen(SIZE_X, SIZE_Y);
        Game {
            gl,
            maze,
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
        }
    }
    //generates a white gameboard
    pub fn render(&mut self, arg: &RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
        self.gl.draw(arg.viewport(), |mut c, gll| {
            clear(BLUE, gll);
            //c.transform = c.transform.trans(cx, cy);
        });
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
