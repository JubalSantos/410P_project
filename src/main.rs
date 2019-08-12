extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;

use opengl_graphics::GlGraphics;
use piston_window::*;
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
enum Color {
    Red,
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
}

type Player = Board;

enum DrawEffect<'a> {
    None,
    Flash(&'a Vec<usize>),
    Darker,
}

impl Board {
    fn empty(dim_x: usize, dim_y: usize) -> Self {
        let line: Vec<_> = (0..dim_x).map(|_| None).collect();
        let cells: Vec<_> = (0..dim_y).map(|_| line.clone()).collect();
        Board { cells }
    }

    fn dim_x(&self) -> usize {
        self.cells[0].len()
    }
    fn dim_y(&self) -> usize {
        self.cells.len()
    }

    fn player(spec: &[[u8; 4]; 4], color: Color) -> Self {
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
                    if self.cells[y as usize][x as usize].is_none() {
                        copy.cells[y as usize][x as usize] = cell.clone();
                    } else {
                        // Collision
                        return None;
                    }
                }
            }
        }

        Some(copy)
    }

    fn draw<'a>(
        &self,
        c: &Context,
        gl: &mut GlGraphics,
        effect: DrawEffect<'a>,
        metrics: &Metrics,
    ) {
        let mut draw = |color, rect: [f64; 4]| {
            Rectangle::new(color).draw(rect, &DrawState::default(), c.transform, gl);
        };

        for x in 0..self.dim_x() {
            for y in 0..self.dim_y() {
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
                    };
                    draw(code, outer);
                }

                match effect {
                    DrawEffect::None => {}
                    DrawEffect::Flash(lines) => {
                        if lines.contains(&(y as usize)) {
                            draw([1.0, 1.0, 1.0, 0.5], outer);
                        }
                    }
                    DrawEffect::Darker => {
                        draw([0.0, 0.0, 0.0, 0.9], outer);
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

    fn get_full_lines_indicts(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, line)| line.iter().all(|cell| !cell.is_none()))
            .map(|(idx, _)| idx)
            .collect()
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
    Flashing(isize, Instant, Vec<usize>),
    GameOver,
}

struct Game {
    board: Board,
    metrics: Metrics,
    piece: Vec<Board>,
    state: State,
}

impl Game {
    fn new(metrics: Metrics) -> Self {
        let __ = 0;
        let xx = 01;
        let piece = vec![Board::player(
            &[
                [__, __, __, __],
                [__, __, __, __],
                [__, __, xx, __],
                [__, __, __, __],
            ],
            Color::Red,
        )]
        .into_iter()
        .map(|x| x.with_trim_sides())
        .collect();

        Game {
            board: Board::empty(metrics.board_x, metrics.board_y),
            state: State::Moving(Self::new_move(&piece)),
            piece,
            metrics,
        }
    }

    fn new_move(piece: &Vec<Board>) -> Moving {
        let idx = rand::random::<usize>() % piece.len();

        Moving {
            offset: (0, 0),
            player: piece[idx].clone(),
            time_since_move: Instant::now(),
        }
    }

    fn move_piece(&mut self, change: (isize, isize)) {
        let opt_new_state = match &mut self.state {
            State::GameOver | State::Flashing(_, _, _) => None,
            State::Moving(moving) => {
                let new_offset = {
                    let (x, y) = moving.offset;
                    ((x as isize + change.0), (y as isize + change.1))
                };

                let is_down = change == (0, 1);

                if self.board.as_merged(new_offset, &moving.player).is_none() {
                    // There were collisions
                    if is_down {
                        match self.board.as_merged(moving.offset, &moving.player) {
                            None => Some(State::GameOver),
                            Some(merged_board) => {
                                let completed = merged_board.get_full_lines_indicts();
                                self.board = merged_board;

                                *moving = Self::new_move(&self.piece);
                                if !completed.is_empty() {
                                    Some(State::Flashing(0, Instant::now(), completed))
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        None
                    }
                } else {
                    moving.offset = new_offset;
                    moving.time_since_move = Instant::now();
                    None
                }
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
            State::GameOver => return,
            State::Flashing(stage, last_stage_switch, _lines) => {
                if last_stage_switch.elapsed() <= Duration::from_millis(50) {
                    return;
                }
                if *stage < 18 {
                    *stage += 1;
                    *last_stage_switch = Instant::now();
                    return;
                } else {
                    Disposition::Mm
                }
            }
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
            State::Flashing(stage, _, lines) => {
                let effect = {
                    if *stage % 2 == 0 {
                        DrawEffect::None
                    } else {
                        DrawEffect::Flash(&lines)
                    }
                };
                self.board.draw(c, gl, effect, &self.metrics);
            }
            State::Moving(moving) => {
                if let Some(merged) = self.board.as_merged(moving.offset, &moving.player) {
                    merged.draw(c, gl, DrawEffect::None, &self.metrics);
                }
            }
            State::GameOver => {
                self.board.draw(c, gl, DrawEffect::Darker, &self.metrics);
            }
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
        block_pixels: 20,
        board_x: 30,
        board_y: 30,
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