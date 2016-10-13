extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;

fn init<'a>() -> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", 400, 400)
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

    let mut x = 0;
    let y = 20;
    let white = Color::RGB(255, 255, 255);
    let red = Color::RGB(255, 0, 0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        renderer.set_draw_color(white);
        renderer.clear();
        renderer.set_draw_color(red);
        renderer.fill_rect(Rect::new(x, y, 10, 10)).unwrap();
        renderer.present();
        x = (x + 5) % 400;
        thread::sleep(time::Duration::from_millis(50));
    }
}
