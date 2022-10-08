mod core;

use crate::core::{
    ball::Ball,
    bricks::{Bricks, Touch, BRICK_HEIGHT, BRICK_WIDTH},
    player::Player,
};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::pixels::Color;
use std::{cell::RefCell, rc::Rc, time::Duration};

use sdl2::render::Canvas;
use sdl2::video::Window;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const BACKGROUND_COLOR: Color = Color::RGB(0, 255, 255);

pub fn rerender(canvas: Rc<RefCell<Canvas<Window>>>) {
    let canvas = &mut *canvas.borrow_mut();
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rustbreak", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let canvas = Rc::new(RefCell::new(canvas));
    rerender(Rc::clone(&canvas));

    // Keylogger :tf:
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bricks = Bricks::new(Rc::clone(&canvas));
    bricks.draw();

    let mut ball = Ball::new(Rc::clone(&canvas));
    ball.draw();

    let mut player = Player::new(Rc::clone(&canvas));
    player.draw();

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
                    bricks.kill(4, 4);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    player.translate(-5);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    player.translate(5);
                }
                _ => {}
            }
        }

        pub fn intersect(ball: &Ball, bricks: &Bricks) -> Touch {
            for brick in bricks.brick.iter() {
                // [  ] <-
                //if ball.x < brick.x() as i16 + BRICK_WIDTH as i16
                //    && ball.y > brick.y() as i16
                //    && ball.y < (brick.y() as i16 + BRICK_HEIGHT as i16)
                //{
                //    dbg!("pee");
                //    return Touch::Side;
                //}

                //// -> [  ] ;;;;;;;;
                //if ball.x < brick.x() as i16
                //    && ball.y > brick.y() as i16
                //    && ball.y < (brick.y() as i16 + BRICK_HEIGHT as i16)
                //{
                //    return Touch::Side;
                //}

                //// [  ] ;;;;;;;;;;;
                ////   ^
                //if ball.y < brick.y() as i16 + BRICK_HEIGHT as i16
                //    && ball.x < (brick.x() as i16 + BRICK_WIDTH as i16)
                //    && ball.x > (brick.x() as i16 )
                //{
                //    return Touch::Top;
                //}

                //   ,
                // [  ]
                //if ball.y > brick.y() as i16
                //    && ball.x > brick.x() as i16
                //    && ball.x < (brick.x() as i16 + BRICK_WIDTH as i16)
                //{
                //    return Touch::Top;
                //}

                if ball.x + 20 > brick.x() as i16
                    && ball.x - 20 < brick.x() as i16 + BRICK_WIDTH as i16
                    && ball.y + 20 > brick.y() as i16
                    && ball.y - 20 < brick.y() as i16 + BRICK_HEIGHT as i16
                {
                    return Touch::Top;
                }
            }
            Touch::None
        }

        //match bricks.intersect_ball(ball.y as i32, ball.x as i32) {
        match intersect(&ball, &bricks) {
            Touch::Top => {
                ball.reflect_y();
                ball.reflect_x();
            }
            Touch::Side => {
                ball.reflect_x();
                ball.reflect_y();
            }
            _ => (),
        }
        ball.physics();

        canvas.borrow_mut().present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
