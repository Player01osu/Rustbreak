use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::{rect::Rect, render::Canvas};

use crate::{Entity, BALL_COLOR, BALL_RADIUS, BALL_VEL, GRID_HEIGHT, GRID_WIDTH, HEIGHT, WIDTH};

use super::bricks::{Bricks, Touch};
use super::player::Player;

#[derive(Debug)]
pub struct Ball {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub radius: i16,
    lives: u8,
}

pub trait BallInteraction {
    fn intersect(&mut self, ball: &mut Ball);
}
pub fn intersect_bricks(ball: &mut Ball, bricks: &mut Bricks) -> Touch {
    for col in bricks.bricks.iter_mut() {
        for brick in col.iter_mut() {
            let ball_right = (ball.x + BALL_RADIUS) as i32;
            let ball_left = (ball.x - BALL_RADIUS) as i32;

            let ball_top = (ball.y - BALL_RADIUS) as i32;
            let ball_bottom = (ball.y + BALL_RADIUS) as i32;

            let brick_right = brick.x() + brick.width() as i32;
            let brick_left = brick.x();

            let brick_top = brick.y();
            let brick_bottom = brick.y() + brick.height() as i32;

            let is_between = |a_a, a_b, b_a, b_b| a_a <= b_b && a_b >= b_a;

            let touch = is_between(ball_left, ball_right, brick_left, brick_right)
                && (is_between(ball_top, ball_bottom, brick_top, brick_bottom));

            if brick.alive && touch {
                brick.dead();

                if is_between(ball.x as i32, ball.x as i32, brick_left, brick_right) {
                    return Touch::Top;
                } else {
                    return Touch::Side;
                }
            }
        }
    }
    Touch::None
}

impl BallInteraction for Bricks {
    fn intersect(&mut self, ball: &mut Ball) {
        match intersect_bricks(ball, self) {
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
            ball.neg_dy();
            let mid = self.rect.x + self.rect.width() as i32 / 2;
            if ball.x as i32 > mid {
                ball.pos_dx()
            } else {
                ball.neg_dx()
            }
        }
    }
}

impl Entity for Ball {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(BALL_COLOR);
        canvas.fill_rect(Rect::new(
            (self.x - self.radius).into(),
            (self.y - self.radius).into(),
            self.radius as u32 * 2,
            self.radius as u32 * 2,
        )).unwrap();
        //canvas
        //    .filled_circle(self.x, self.y, self.radius, BALL_COLOR)
        //    .unwrap();
    }
    fn kill(&mut self, _canvas: &mut Canvas<Window>) {
        todo!()
    }
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            x: GRID_WIDTH as i16 / 2,
            y: GRID_HEIGHT as i16 - 50,
            radius: BALL_RADIUS,
            lives: 3,
            dx: BALL_VEL,
            dy: BALL_VEL,
        }
    }

    pub fn reflect_x(&mut self) {
        self.dx *= -1;
    }

    pub fn reflect_y(&mut self) {
        self.dy *= -1;
    }

    pub fn pos_dx(&mut self) {
        self.dx = self.dx.abs();
    }

    pub fn neg_dx(&mut self) {
        self.dx = -self.dx.abs();
    }

    pub fn neg_dy(&mut self) {
        self.dy = -self.dy.abs();
    }

    pub fn physics(&mut self, dt: f64) {
        if self.x < self.radius || self.x as u32 > GRID_WIDTH - self.radius as u32 {
            self.reflect_x();
        }

        if self.y < self.radius || self.y as u32 > GRID_HEIGHT - self.radius as u32 {
            self.reflect_y();
        }

        self.x += (self.dx as f64 * dt) as i16;
        self.y += (self.dy as f64 * dt) as i16;
    }
}
