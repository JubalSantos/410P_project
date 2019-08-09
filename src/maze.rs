extern crate rand;

use std::vec::Vec;

//tile can be either a wall of and empty space
pub enum Tile {
    Wall,
    path,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Tile>,
}

impl Maze {
    fn init(width: usize, height: usize) -> Maze {
        let mut maze = Maze{
            width: width,
            height: height,
            data: Vec::new(),
        };
        for y in 0..height {
            for x in 0..width {
                let value = if y == 0 || y == height - 1 { 
                    Tile::path
                }
                else if x == 0 || x == width - 1 {
                    Tile::path
                }
                else {
                    Tile::Wall
                };
                maze.data.push(value);
            }
        }
        maze
    }
    pub fn print(&self) {
        for y in 0..self.height{
            for x in 0..self.width{
                match self.data[y * self.width + x]{
                    Tile::Wall => print!("▉▉"),
                    Tile::path => print!("  "),
                }
            }
            println!()
        }
    }
    //checks to see if its a wall or path
    fn wall(&self, x: isize, y: isize) -> bool {
        let (ux, uy) = (x as usize, y as usize);
        match self.data[(uy * self.width + ux) as usize] {
            Tile::Wall => true,
            Tile::path => false,
        }
    }
    //this creates the path so there is and enterance and exit
    fn carve<R: rand::Rng>(&mut self, rng: &mut R, x: usize, y: usize) {
        let xd = [1, -1, 0, 0];
        let yd = [0, 0, 1, -1];
        self.data[y * self.width + x] = Tile::path;
        let d = rng.gen::<usize>() % 4;
        for i in 0..4{
            let dx: isize = xd[(d + i) % 4];
            let dy: isize = yd[(d + i) % 4];
            let x2 = (x as isize) + dx;
            let y2 = (y as isize) + dy;
            if self.wall(x2, y2){
                let nx = x2 + dx;
                let ny = y2 + dy;
                if self.wall(nx, ny){
                    let ndx = (y2 as usize) * self.width + (x2 as usize);
                    self.data[ndx] = Tile::path;
                    self.carve(rng, nx as usize, ny as usize);
                }
            }
        }
    }
    //generates the maze and returns it
    pub fn gen(width: usize, height: usize) -> Maze{
        let mut maze = Maze::init(width, height);
        let mut rn = rand::thread_rng();
        maze.carve(&mut rn, 2, 2);
        maze.data[1 * width + 2] = Tile::path;
        maze.data[(height -2) * width + (width - 3 )] = Tile::path;
        maze
    }

}