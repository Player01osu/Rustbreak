use std::cell::RefCell;
use std::rc::Rc;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::Window;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use crate::{HEIGHT, WIDTH, BACKGROUND_COLOR};

pub struct Ball {
    canvas: Rc<RefCell<Canvas<Window>>>,
    pub x: i16,
    pub y: i16,
    vel_x: i16,
    vel_y: i16,
    pub radius: i16,
    lives: u8,
}

impl Ball {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Ball {
        Ball {
            canvas,
            x: WIDTH as i16 / 2,
            y: HEIGHT as i16 - 50,
            radius: 20,
            lives: 3,
            vel_x: 8,
            vel_y: 8,
        }
    }

    pub fn draw(&self) {
        self.canvas
            .borrow_mut()
            .filled_circle(self.x, self.y, self.radius, Color::RGBA(0, 0, 0, 255))
            .unwrap();
    }

    pub fn blank(&self) {
        self.canvas
            .borrow_mut()
            .filled_circle(self.x, self.y, self.radius, BACKGROUND_COLOR)
            .unwrap();
    }

    pub fn reflect_x(&mut self) {
        self.vel_x *= -1;
    }

    pub fn reflect_y(&mut self) {
        self.vel_y *= -1;
    }

    pub fn physics(&mut self) {
        if self.x < 0 + self.radius || self.x as u32 > WIDTH - self.radius as u32{
            self.vel_x *= -1;
        }

        if self.y < 0 + self.radius || self.y as u32 > HEIGHT - self.radius as u32{
            self.vel_y *= -1;
        }

        self.blank();
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.draw();
    }

    pub fn intersect_side(&self, left: i16, right: i16, top: i16, bottom: i16) -> bool {
        if self.x < right || self.x > left || self.y < top || self.y > bottom {
            return true
        }
        false
    }

    pub fn intersect_top(&self, top: i16, bottom: i16) -> bool {
        if self.y < top || self.y > bottom {
            return true
        }
        false
    }
}
