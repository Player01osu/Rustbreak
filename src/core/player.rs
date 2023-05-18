use std::{cell::RefCell, rc::Rc};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{Entity, BACKGROUND_COLOR, HEIGHT, WIDTH};

pub struct Player {
    pub rect: Rect,
    vel: i32,
}

const PLAYER_WIDTH: u32 = 90;
const PLAYER_HEIGHT: u32 = 10;

impl Entity for Player {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.translate(self.vel);
        canvas.draw_rect(self.rect).unwrap();
    }

    fn kill(&mut self, canvas: &mut Canvas<Window>) {
        todo!()
    }
}

impl Player {
    pub fn new() -> Player {
        let rect = Rect::new(
            (WIDTH as i32 / 2) - (PLAYER_WIDTH as i32 / 2),
            HEIGHT as i32 - 15,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );
        Player {
            rect,
            vel: Default::default(),
        }
    }

    pub fn translate(&mut self, vel: i32) {
        self.rect.x += vel;
    }

    pub fn set_vel(&mut self, vel: i32) {
        self.vel = vel;
    }
}
