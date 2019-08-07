extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod game;
mod player;
mod maze;

use crate::game::Game;
use crate::player::Enemie;
use crate::player::{Player, Direction};

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
