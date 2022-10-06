mod core;

use crate::core::bricks::{Brick, Bricks};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use std::{cell::RefCell, rc::Rc, time::Duration};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rustbreak", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut canvas = Rc::new(RefCell::new(canvas));

    {
        let canvas = &mut *canvas.borrow_mut();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
    }

    // Keylogger :tf:
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bricks = Bricks::new(Rc::clone(&canvas));
    bricks.draw();

    canvas.borrow_mut().present();
    'running: loop {
        for event in event_pump.poll_iter() {
            // Q or Esc to quit
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    bricks.kill(1, 1);
                }
                _ => {}
            }
        }

        //TextureCreator::create_texturej
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
