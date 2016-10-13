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

fn life_random(ncells: u32) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();

    let mut v: Vec<Vec<bool>> = Vec::new();

    for i in 0..ncells {
        v.push(Vec::new());
        for _ in 0..ncells {
            v[i as usize].push(rng.gen());
        }
    }

    v
}

fn glider(ncells: u32) -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<bool>> = Vec::with_capacity(NCELLS as usize);

    for i in 0..ncells {
        v.push(Vec::with_capacity(NCELLS as usize));
        for _ in 0..ncells {
            v[i as usize].push(false);
        }
    }

    v[10][11] = true;
    v[11][12] = true;
    v[12][10] = true;
    v[12][11] = true;
    v[12][12] = true;

    v
}

fn display_cell(r: &mut Renderer, col: u32, row: u32) {
    let x = CELL_WIDTH * col;
    let y = CELL_HEIGHT * row;

    let cell_color = Color::RGB(255, 0, 0);
    r.set_draw_color(cell_color);
    r.fill_rect(Rect::new(x as i32, y as i32, CELL_WIDTH, CELL_HEIGHT)).unwrap();
}

fn display_frame(r: &mut Renderer, v: &Vec<Vec<bool>>) {
    r.set_draw_color(Color::RGB(200, 200, 200));
    r.clear();

    for i in 0..NCELLS {
        for j in 0..NCELLS {
            if v[i as usize][j as usize] {
                display_cell(r, i, j);
            }
        }
    }

    r.present();
}

fn alive(r: u32, c: u32, v: &Vec<Vec<bool>>) -> bool {
    let n = count_surrounding(r, c, v);

    let curr = v[r as usize][c as usize];

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
fn count_surrounding(r: u32, c: u32, v: &Vec<Vec<bool>>) -> i32 {
    let r = r as usize;
    let c = c as usize;
    let dec_r = dec(r);
    let inc_r = inc(r);
    let dec_c = dec(c);
    let inc_c = inc(c);
    let v_r = &v[r];
    let v_dec_r = &v[dec_r];
    let v_inc_r = &v[inc_r];

    v_dec_r[c] as i32 +
    v_inc_r[c] as i32 +
    v_r[dec_c] as i32 +
    v_r[inc_c] as i32 +
    v_dec_r[dec_c] as i32 +
    v_dec_r[inc_c] as i32 +
    v_inc_r[inc_c] as i32 +
    v_inc_r[dec_c] as i32
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

fn life_next(v: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut v2: Vec<Vec<bool>> = Vec::with_capacity(NCELLS as usize);

    for i in 0..NCELLS {
        v2.push(Vec::with_capacity(NCELLS as usize));
        for j in 0..NCELLS {
            let a = alive(i, j, &v);
            v2[i as usize].push(a);
        }
    }

    v2
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

    let mut v = life_random(NCELLS);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        display_frame(&mut renderer, &v);
        v = life_next(v);
        thread::sleep(time::Duration::from_millis(50));
    }
}
