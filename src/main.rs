extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;
use std::convert::TryInto;

use opengl_graphics::GlGraphics;
use piston_window::*;
use std::time::{Duration, Instant};

const SIZE_X: usize = 21;
const SIZE_Y: usize = 21;

#[derive(Copy, Clone)]
enum Color {
    Red,
    Blue,
}

struct Metrics {
    block_pixels: usize,
    board_x: usize,
    board_y: usize,
}

impl Metrics {
    fn resolution(&self) -> [u32; 2] {
        [
            (self.board_x * self.block_pixels) as u32,
            (self.board_y * self.block_pixels) as u32,
        ]
    }
}

type Cell = Option<Color>;

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<Cell>>,
    pattern: Vec<Vec<u8>>,
}

type Player = Board;

impl Board {
    fn empty(dim_x: usize, dim_y: usize) -> Self {
        let line: Vec<_> = (0..dim_x).map(|_| None).collect();
        let cells: Vec<_> = (0..dim_y).map(|_| line.clone()).collect();
        
        //===
        //let mut rng = rand::thread_rng();
        //let mut val: u8 = rng.gen();
        //val = val % 2;
        //=== 
        let pattern = vec![
        vec![0,1,0,0,0,1,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0], 
        vec![0,1,0,1,0,1,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0],  
        vec![0,1,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0],  
        vec![0,0,0,1,0,1,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0],  
        vec![0,1,1,1,0,1,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0],  
        vec![0,1,0,0,0,1,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0], 
        vec![1,1,1,0,1,1,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1], 
        vec![0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0], 
        vec![0,0,1,0,1,1,1,1,1,1,1,1,0,0,1,1,1,0,1,1,2], 
        vec![0,0,1,0,0,0,0,0,0,0,0,1,0,0,1,0,0,0,1,1,1], 
        vec![0,0,1,1,1,0,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1], 
        vec![0,0,0,0,1,0,1,0,0,1,0,0,1,1,0,0,0,1,0,0,0], 
        vec![0,0,0,0,1,0,1,0,0,1,0,0,0,0,0,1,1,1,0,0,0], 
        vec![1,1,1,1,1,0,1,0,0,1,0,1,1,1,1,1,1,0,0,0,0], 
        vec![0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0], 
        vec![1,0,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0], 
        vec![1,0,0,0,1,0,1,0,0,0,0,0,0,0,1,0,0,1,0,0,0], 
        vec![1,1,1,0,1,0,1,0,0,0,0,0,0,1,0,1,1,0,1,0,0], 
        vec![0,0,1,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,1,0,0], 
        vec![0,0,1,0,0,0,1,0,0,0,0,0,0,0,1,0,0,1,0,0,0], 
        vec![0,0,1,1,1,1,1,0,0,0,0,0,0,0,0,1,1,0,0,0,0], 
                            ];
        Board { cells , pattern}
    }
    fn valid(&self, offset: (isize, isize)) -> bool {
        if offset.0 >= 0  &&  offset.0 < self.dim_x() as isize {
            if offset.1 >= 0  &&  offset.1 < self.dim_y() as isize {
                return true;
            }
        }
        return false;
    }

    fn dim_x(&self) -> usize {
        self.cells[0].len()
    }
    fn dim_y(&self) -> usize {
        self.cells.len()
    }

    fn player(spec: &[[u8; 1]; 1], color: Color) -> Self {
        let mut board = Board::empty(spec[0].len(), spec.len());

        for x in 0..spec[0].len() {
            for y in 0..spec.len() {
                board.cells[y][x] = if spec[y][x] != 0 { Some(color) } else { None }
            }
        }

        board
    }

    fn as_merged(&self, offset: (isize, isize), board: &Board) -> Option<Board> {
        let mut copy = self.clone();

        for x in 0..board.dim_x() {
            for y in 0..board.dim_y() {
                let cell = board.cells[y][x];
                if cell.is_some() {
                    let x = x as isize + offset.0;
                    let y = y as isize + offset.1;
                    if !self.valid((x, y)) {
                        return None;
                    }
                    else if self.cells[y as usize][x as usize].is_none() {
                        copy.cells[y as usize][x as usize] = cell.clone();
                    } 
                }
            }
        }

        Some(copy)
    }

    fn draw(&self, c: &Context, gl: &mut GlGraphics, metrics: &Metrics) {
        let mut draw = |color, rect: [f64; 4]| {
            Rectangle::new(color).draw(rect, &DrawState::default(), c.transform, gl);
        };

        for x in 0..self.dim_x() {
            for y in 0..self.dim_y() {
                if self.pattern[y][x] == 0 {
                    // leave this alone
                    let block_pixels = metrics.block_pixels as f64;
                    let border_size = block_pixels / 20.0;
                    let outer = [
                        block_pixels * (x as f64),
                        block_pixels * (y as f64),
                        block_pixels,
                        block_pixels,
                    ];
                    let inner = [
                        outer[0] + border_size,
                        outer[1] + border_size,
                        outer[2] - border_size * 2.0,
                        outer[3] - border_size * 2.0,
                    ];

                    draw([0.0, 0.0, 0.0, 1.0], outer);
                    draw([0.1, 0.2, 0.3, 1.0], inner);

                    if let Some(color) = self.cells[y][x] {
                        let code = match color {
                            Color::Red => [1.0, 0.0, 0.0, 1.0],
                            Color::Blue => [0.0, 0.0, 0.0, 0.0],
                        };
                        draw(code, outer);
                    }
                }
                if self.pattern[y][x] == 1 {
                    let block_pixels = metrics.block_pixels as f64;
                    let border_size = block_pixels / 20.0;
                    let outer = [
                        block_pixels * (x as f64),
                        block_pixels * (y as f64),
                        block_pixels,
                        block_pixels,
                    ];
                    let inner = [
                        outer[0] + border_size,
                        outer[1] + border_size,
                        outer[2] - border_size * 2.0,
                        outer[3] - border_size * 2.0,
                    ];

                    draw([0.0, 0.0, 0.0, 0.0], outer);
                    draw([0.1, 0.0, 1.0, 1.0], inner);

                    if let Some(color) = self.cells[y][x] {
                        let code = match color {
                            Color::Red => [1.0, 0.0, 0.0, 1.0],
                            Color::Blue => [0.0, 0.0, 0.0, 0.0],
                        };
                        draw(code, outer);
                    }
                }
                if self.pattern[y][x] == 2 {
                    let block_pixels = metrics.block_pixels as f64;
                    let border_size = block_pixels / 20.0;
                    let outer = [
                        block_pixels * (x as f64),
                        block_pixels * (y as f64),
                        block_pixels,
                        block_pixels,
                    ];
                    let inner = [
                        outer[0] + border_size,
                        outer[1] + border_size,
                        outer[2] - border_size * 2.0,
                        outer[3] - border_size * 2.0,
                    ];

                    draw([0.0, 0.0, 0.0, 0.0], outer);
                    draw([0.5, 0.5, 0.5, 0.5], inner);

                    if let Some(color) = self.cells[y][x] {
                        let code = match color {
                            Color::Red => [1.0, 0.0, 0.0, 1.0],
                            Color::Blue => [0.0, 0.0, 0.0, 0.0],
                        };
                        draw(code, outer);
                    }
                }
            }
        }
    }

    fn without_line(&self, idx: usize) -> Self {
        let mut board = self.clone();

        board.cells.remove(idx);
        board
    }

    fn with_trimmed_lines(&self) -> Self {
        let mut board = self.clone();

        while board.cells[0].iter().all(Cell::is_none) {
            board = board.without_line(0);
        }

        while board.cells[board.dim_y() - 1].iter().all(Cell::is_none) {
            board = board.without_line(board.dim_y() - 1);
        }

        board
    }

    fn transposed(&self) -> Self {
        let mut board = Self::empty(self.dim_y(), self.dim_x());

        for x in 0..self.dim_x() {
            for y in 0..self.dim_y() {
                board.cells[x][y] = self.cells[y][x];
            }
        }

        board
    }

    fn with_trim_sides(&self) -> Self {
        self.with_trimmed_lines()
            .transposed()
            .with_trimmed_lines()
            .transposed()
    }
}

struct Moving {
    offset: (isize, isize),
    player: Player,
    time_since_move: Instant,
}

enum State {
    Moving(Moving),
    //GameOver,
}

struct Game {
    board: Board,
    metrics: Metrics,
    state: State,
    //maze: Maze
}

impl Game {
    fn new(metrics: Metrics) -> Self {
        let piece = vec![Board::player(&[[1]], Color::Red)]
            .into_iter()
            .map(|x| x.with_trim_sides())
            .collect();

        Game {
            board: Board::empty(metrics.board_x, metrics.board_y),
            state: State::Moving(Self::new_move(&piece)),
            metrics,
        }
    }

    fn new_move(piece: &Vec<Board>) -> Moving {
        Moving {
            offset: (0, 0),
            player: piece[0].clone(),
            time_since_move: Instant::now(),
        }
    }

    fn move_piece(&mut self, change: (isize, isize)) {
        let opt_new_state = match &mut self.state {
            //State::GameOver => None,
            State::Moving(moving) => {
                let mut new_offset = {
                    let (x, y) = moving.offset;
                    ((x as isize + change.0), (y as isize + change.1))
                };

                if new_offset.0 < 1 {
                    new_offset.0 = 0;
                }
                if new_offset.0 > (SIZE_X-1).try_into().unwrap() {
                    new_offset.0 = (SIZE_X-1).try_into().unwrap();
                }
                if new_offset.1 < 1 {
                    new_offset.1 = 0;
                }
                if new_offset.1 > (SIZE_Y-1).try_into().unwrap() {
                    new_offset.1 = (SIZE_Y-1).try_into().unwrap();
                }

                let x: usize = (new_offset.0).try_into().unwrap();
                let y: usize = (new_offset.1).try_into().unwrap();

                if self.board.pattern[y][x] == 0 {
                    moving.offset = new_offset;
                    //println!("\nValid move!");
                }
                else if self.board.pattern[y][x] == 1 {
                    //println!("\nInvalid move!");
                }
                else if self.board.pattern[y][x] == 2 {
                    println!("YOU WIN!!!");
                    std::process::exit(1);
                }
                //======
                // check for out of bounds move
                if moving.offset.0 < 1 {
                    moving.offset.0 = 0;
                }
                if moving.offset.0 > (SIZE_X-1).try_into().unwrap() {
                    moving.offset.0 = (SIZE_X-1).try_into().unwrap();
                }
                if moving.offset.1 < 1 {
                    moving.offset.1 = 0;
                }
                if moving.offset.1 > (SIZE_Y-1).try_into().unwrap() {
                    moving.offset.1 = (SIZE_Y-1).try_into().unwrap();
                }
                //======
                moving.time_since_move = Instant::now();
                None
            }
        };

        if let Some(new_state) = opt_new_state {
            self.state = new_state;
        }
    }

    fn progress(&mut self) {
        enum Disposition {
            Mm,
        }

        let disp = match &mut self.state {
            //State::GameOver => return,
            State::Moving(moving) => {
                if moving.time_since_move.elapsed() <= Duration::from_millis(700) {
                    return;
                }
                Disposition::Mm
            }
        };

        match disp {
            Disposition::Mm => self.move_piece((0, 0)),
        }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let res = self.metrics.resolution();
        let c = &Context::new_abs(f64::from(res[0]), f64::from(res[1]));

        gl.draw(args.viewport(), |_, gl| match &self.state {
            State::Moving(moving) => {
                if let Some(merged) = self.board.as_merged(moving.offset, &moving.player) {
                    merged.draw(c, gl, &self.metrics);
                }
            } //State::GameOver
        });
    }

    fn on_press(&mut self, args: &Button) {
        if let Button::Keyboard(key) = args {
            self.on_key(*key);
        }
    }

    fn on_key(&mut self, key: Key) {
        let movement = match key {
            Key::Right => Some((1, 0)),
            Key::Left => Some((-1, 0)),
            Key::Down => Some((0, 1)),
            Key::Up => Some((0, -1)),
            _ => None,
        };

        if let Some(movement) = movement {
            self.move_piece(movement);
            return;
        }
    }
}

fn main() {
    let metrics = Metrics {
        block_pixels: 30,
        board_x: SIZE_X,
        board_y: SIZE_Y,
    };

    let mut window: PistonWindow = WindowSettings::new("Mazemania", metrics.resolution())
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed: {}", e));

    let mut gl = GlGraphics::new(OpenGL::V3_2);
    let mut game = Game::new(metrics);

    while let Some(e) = window.next() {
        game.progress();

        if let Some(args) = e.render_args() {
            game.render(&mut gl, &args);
        }

        if let Some(args) = e.press_args() {
            game.on_press(&args);
        }
    }
}
