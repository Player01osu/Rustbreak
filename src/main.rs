#![allow(dead_code)]
mod core;

use self::core::ball::BallInteraction;
use crate::core::{ball::Ball, bricks::Bricks, player::Player};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, rect::Rect};
use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;
pub const FPS: u32 = 30;

pub const GRID_WIDTH: u32 = WIDTH;
pub const GRID_HEIGHT: u32 = HEIGHT * 3 / 4;

pub const MAX_ROW: usize = 8;
pub const MAX_COL: usize = 14;
pub const GRID_RATIO_N: u32 = 2;
pub const GRID_RATIO_D: u32 = 7;

pub const BALL_RADIUS: i16 = GRID_WIDTH as i16/ 54;
pub const BALL_VEL: i16 = 230;
pub const BALL_COLOR: Color = Color::RGB(200, 60, 60);

pub const PLAYER_WIDTH: u32 = GRID_WIDTH / 8;
pub const PLAYER_HEIGHT: u32 = GRID_HEIGHT / 40;
pub const PLAYER_VEL: i32 = 300;
pub const PLAYER_COLOR: Color = Color::RGB(80, 240, 220);

pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub const LEFT_KEY: Keycode = Keycode::H;
pub const RIGHT_KEY: Keycode = Keycode::L;

pub fn redraw_bg(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();
}

pub trait Entity {
    fn draw(&mut self, canvas: &mut Canvas<Window>);
    fn kill(&mut self, canvas: &mut Canvas<Window>);
}

pub fn intersect_player(ball: &Ball, player: &Player) -> bool {
    Rect::new(
        (ball.x - ball.radius).into(),
        (ball.y - ball.radius).into(),
        ball.radius as u32 * 2,
        ball.radius as u32 * 2,
    )
    .has_intersection(player.rect)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Input {
    Left,
    Right,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("TEST", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bricks = Bricks::new();
    let mut ball = Ball::new();
    let mut player = Player::new();

    let timer = sdl_context.timer().unwrap();
    let mut dt;
    let mut counter = timer.performance_counter();
    let mut previous_counter = counter;

    let mut inputs = [false, false];

    'running: loop {
        dt = (counter - previous_counter) as f64 / timer.performance_frequency() as f64;
        previous_counter = counter;
        counter = timer.performance_counter();

        redraw_bg(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                        match keycode {
                            LEFT_KEY => {
                                inputs[Input::Left as usize] = true;
                                inputs[Input::Right as usize] = false;
                            }
                            RIGHT_KEY => {
                                inputs[Input::Right as usize] = true;
                                inputs[Input::Left as usize] = false;
                            }
                            _ => (),
                        }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                        match keycode {
                            LEFT_KEY => inputs[Input::Left as usize] = false,
                            RIGHT_KEY => inputs[Input::Right as usize] = false,
                            _ => (),
                        }
                }
                _ => {}
            }
        }

        if inputs[Input::Left as usize] {
            player.vel = -(PLAYER_VEL as f64 * dt) as i32;
        } else if inputs[Input::Right as usize] {
            player.vel = (PLAYER_VEL as f64 * dt) as i32;
        } else {
            player.vel = 0;
        }

        bricks.intersect(&mut ball);
        player.intersect(&mut ball);

        ball.physics(dt);

        bricks.draw(&mut canvas);
        ball.draw(&mut canvas);
        player.draw(&mut canvas);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
