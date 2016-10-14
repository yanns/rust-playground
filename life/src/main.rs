extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;
use rand::Rng;

const MAX_X: u32 = 899;
const MAX_Y: u32 = 699;
const CELL_WIDTH: u32 = 1;
const CELL_HEIGHT: u32 = CELL_WIDTH;
const NCELLS: u32 = (MAX_X + 1) / CELL_WIDTH;

struct Grid {
    cells: [[bool; NCELLS as usize]; NCELLS as usize],
}

impl Grid {
    fn new() -> Grid {
        Grid { cells: [[false; NCELLS as usize]; NCELLS as usize] }
    }

    fn life_random() -> Grid {
        let mut rng = rand::thread_rng();
        let mut grid = Grid::new();

        for i in 0..NCELLS {
            for j in 0..NCELLS {
                if rng.gen() {
                    grid.cells[i as usize][j as usize] = true;
                }
            }
        }
        grid
    }

    fn glider() -> Grid {
        let mut grid = Grid::new();

        grid.set(10, 11, true);
        grid.set(11, 12, true);
        grid.set(12, 10, true);
        grid.set(12, 11, true);
        grid.set(12, 12, true);
        grid
    }


    fn set(&mut self, r: usize, c: usize, value: bool) {
        self.cells[r][c] = value;
    }

    fn get(&self, r: usize, c: usize) -> bool {
        self.cells[r][c]
    }

    fn alive(&self, r: u32, c: u32) -> bool {
        let n = self.count_surrounding(r, c);

        let curr = self.cells[r as usize][c as usize];

        match (curr, n) {
            (true, 0...1) => false,
            (true, 2...3) => true,
            (true, _) => false,
            (false, 0...2) => false,
            (false, 3) => true,
            (false, _) => false,
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn count_surrounding(&self, r: u32, c: u32) -> i32 {
        let r = r as usize;
        let c = c as usize;
        let dec_r = dec(r);
        let inc_r = inc(r);
        let dec_c = dec(c);
        let inc_c = inc(c);
        let v_r = &self.cells[r];
        let v_dec_r = &self.cells[dec_r];
        let v_inc_r = &self.cells[inc_r];

        v_dec_r[c] as i32 +
        v_inc_r[c] as i32 +
        v_r[dec_c] as i32 +
        v_r[inc_c] as i32 +
        v_dec_r[dec_c] as i32 +
        v_dec_r[inc_c] as i32 +
        v_inc_r[inc_c] as i32 +
        v_inc_r[dec_c] as i32
    }
}

trait DisplayFrame {
    fn display_frame(&self, r: &mut Renderer);
}

impl DisplayFrame for Grid {
    fn display_frame(&self, r: &mut Renderer) {
        r.set_draw_color(Color::RGB(200, 200, 200));
        r.clear();

        for i in 0..NCELLS {
            for j in 0..NCELLS {
                if self.get(i as usize, j as usize) {
                    display_cell(r, i, j);
                }
            }
        }

        r.present();
    }
}

trait LifeNext {
    fn life_next(&self) -> Self;
}

impl LifeNext for Grid {
    fn life_next(&self) -> Self {
        let mut grid2 = Grid::new();

        for i in 0..NCELLS {
            for j in 0..NCELLS {
                let a = self.alive(i, j);
                grid2.set(i as usize, j as usize, a);
            }
        }

        grid2
    }
}

fn display_cell(r: &mut Renderer, col: u32, row: u32) {
    let x = CELL_WIDTH * col;
    let y = CELL_HEIGHT * row;

    let cell_color = Color::RGB(255, 0, 0);
    r.set_draw_color(cell_color);
    r.fill_rect(Rect::new(x as i32, y as i32, CELL_WIDTH, CELL_HEIGHT)).unwrap();
}

fn inc(n: usize) -> usize {
    (n + 1) % (NCELLS as usize)
}

fn dec(n: usize) -> usize {
    if n == 0 {
        (NCELLS - 1) as usize
    } else {
        (n - 1) as usize
    }
}

fn init<'a>() -> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", MAX_X + 1, MAX_Y + 1)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    (renderer, event_pump)
}

fn main() {
    let (mut renderer, mut event_pump) = init();

    let mut grid = Grid::life_random();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        grid.display_frame(&mut renderer);
        grid = grid.life_next();
        thread::sleep(time::Duration::from_millis(50));
    }
}
