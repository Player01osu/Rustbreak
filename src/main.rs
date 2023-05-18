mod core;

use self::core::ball::BallInteraction;
use crate::core::{ball::Ball, bricks::Bricks, player::Player};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, rect::Rect};
use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const BACKGROUND_COLOR: Color = Color::RGB(0, 255, 255);

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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rustbreak", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bricks = Bricks::new();
    let mut ball = Ball::new();
    let mut player = Player::new();

    'running: loop {
        redraw_bg(&mut canvas);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    player.set_vel(-5);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    player.set_vel(5);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::L),
                    ..
                } => {
                    player.set_vel(0);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::H),
                    ..
                } => {
                    player.set_vel(0);
                }
                _ => {}
            }
        }
        bricks.intersect(&mut ball);
        player.intersect(&mut ball);

        ball.physics();

        bricks.draw(&mut canvas);
        ball.draw(&mut canvas);
        player.draw(&mut canvas);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
