//use ndarray::prelude::*;
use std::fmt;

#[derive(Clone)]
enum Tile {
    Empty,
    WallVert,
    WallHoriz,
    Enemy,
    Event,
}

// https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Tile::Empty => write!(f, "▉"),
           Tile::WallVert => write!(f, "▌"),
           Tile::WallHoriz => write!(f, "▔"),
           Tile::Enemy => write!(f, "😈"),
           Tile::Event => write!(f, "?"),
       }
    }
}

pub struct Maze(Vec<Vec<Tile>>);

impl Maze {

    pub fn new(x: usize, y: usize) -> Self {
        let mut test = Tile::Empty;
        Maze(vec![vec![test; y]; x])
    }

    pub fn create_test(x: usize, y: usize) -> Self {
        let mut data = Self::new(x,y);
        for (_i, row) in data.0.iter().enumerate() {
            for (_j, mut col) in row.iter().enumerate() {
                col = &Tile::Empty;
            }
        }
        data
    }

    pub fn print(&mut self) {
        let mut data = &self.0;
        for (_i, row) in data.iter().enumerate() {
            for (_j, mut col) in row.iter().enumerate() {
                print!("{}", col);
            }
            println!("");
        }
    }

}