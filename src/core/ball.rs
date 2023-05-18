use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use crate::{Entity, BACKGROUND_COLOR, HEIGHT, WIDTH, BALL_RADIUS, BALL_VEL};

use super::bricks::{Bricks, Touch};
use super::player::Player;

#[derive(Debug)]
pub struct Ball {
    pub x: i16,
    pub y: i16,
    vel_x: i16,
    vel_y: i16,
    pub radius: i16,
    lives: u8,
}

pub trait BallInteraction {
    fn intersect(&mut self, ball: &mut Ball);
}
pub fn intersect_bricks(ball: &Ball, bricks: &mut Bricks) -> Touch {
    for col in bricks.bricks.iter_mut() {
        for brick in col.iter_mut() {
            let ball_right = (ball.x + BALL_RADIUS / 2) as i32;
            let ball_left = (ball.x - BALL_RADIUS / 2) as i32;

            let ball_top = (ball.y - BALL_RADIUS / 2) as i32;
            let ball_bottom = (ball.y + BALL_RADIUS / 2) as i32;

            let brick_right = brick.x() + brick.width() as i32;
            let brick_left = brick.x();

            let brick_top = brick.y();
            let brick_bottom = brick.y() + brick.height() as i32;

            let is_between = |a_top, a_b, b_t, b_bottom| a_top <= b_bottom && a_b >= b_t;

            let touch = is_between(ball_left, ball_right, brick_left, brick_right)
                && (is_between(ball_top as i32, ball_bottom as i32, brick_top, brick_bottom));

            if brick.alive {
                if touch {
                    brick.dead();
                    if is_between(ball.x as i32, ball.x as i32, brick_left, brick_right) {
                        return Touch::Top;
                    } else {
                        return Touch::Side;
                    }
                }
            }
        }
    }
    Touch::None
}

impl BallInteraction for Bricks {
    fn intersect(&mut self, ball: &mut Ball) {
        match intersect_bricks(&ball, self) {
            Touch::Top => {
                ball.reflect_y();
            }
            Touch::Side => {
                ball.reflect_x();
            }
            _ => (),
        }
    }
}

impl BallInteraction for Player {
    fn intersect(&mut self, ball: &mut Ball) {
        let ball_rect = Rect::new(
            ball.x as i32 - ball.radius as i32,
            ball.y as i32 - ball.radius as i32,
            ball.radius as u32 * 2,
            ball.radius as u32 * 2,
        );

        if ball_rect.has_intersection(self.rect) {
            ball.neg_vel_y();
            let mid = self.rect.x + self.rect.width() as i32 / 2;
            if ball.x as i32 > mid { ball.pos_vel_x() }
            else { ball.neg_vel_x() }
        }
    }
}

impl Entity for Ball {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas
            .filled_circle(self.x, self.y, self.radius, Color::BLACK)
            .unwrap();
    }
    fn kill(&mut self, _canvas: &mut Canvas<Window>) {
        todo!()
    }
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            x: WIDTH as i16 / 2,
            y: HEIGHT as i16 - 50,
            radius: BALL_RADIUS,
            lives: 3,
            vel_x: BALL_VEL,
            vel_y: BALL_VEL,
        }
    }

    pub fn reflect_x(&mut self) {
        self.vel_x *= -1;
    }

    pub fn reflect_y(&mut self) {
        self.vel_y *= -1;
    }

    pub fn pos_vel_x(&mut self) {
        self.vel_x = self.vel_x.abs();
    }

    pub fn neg_vel_x(&mut self) {
        self.vel_x = self.vel_x.abs() * -1;
    }

    pub fn neg_vel_y(&mut self) {
        self.vel_y = self.vel_y.abs() * -1;
    }

    pub fn physics(&mut self) {
        if self.x < 0 + self.radius || self.x as u32 > WIDTH - self.radius as u32 {
            self.vel_x *= -1;
        }

        if self.y < 0 + self.radius || self.y as u32 > HEIGHT - self.radius as u32 {
            self.vel_y *= -1;
        }

        self.x += self.vel_x;
        self.y += self.vel_y;
    }
}
