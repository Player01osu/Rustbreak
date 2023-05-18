use std::{cell::RefCell, rc::Rc};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{Entity, BACKGROUND_COLOR, HEIGHT, WIDTH, PLAYER_WIDTH, PLAYER_HEIGHT};

pub struct Player {
    pub rect: Rect,
    vel: i32,
}

impl Entity for Player {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.translate();
        canvas.fill_rect(self.rect).unwrap();
    }

    fn kill(&mut self, _canvas: &mut Canvas<Window>) {
        todo!()
    }
}

impl Player {
    pub fn new() -> Player {
        let rect = Rect::new(
            (WIDTH as i32 / 2) - (PLAYER_WIDTH as i32 / 2),
            HEIGHT as i32 - 30,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );
        Player {
            rect,
            vel: Default::default(),
        }
    }

    pub fn translate(&mut self) {
        self.rect.x += self.vel;
    }

    pub fn set_vel(&mut self, vel: i32) {
        self.vel = vel;
    }
}
