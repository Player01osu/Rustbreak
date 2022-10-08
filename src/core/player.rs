use std::{cell::RefCell, rc::Rc};

use sdl2::{rect::Rect, render::Canvas, video::Window, pixels::Color};

use crate::{HEIGHT, WIDTH, BACKGROUND_COLOR};

pub struct Player {
    canvas: Rc<RefCell<Canvas<Window>>>,
    rect: Rect,
}

const PLAYER_WIDTH: u32 = 40;
const PLAYER_HEIGHT: u32 = 10;

impl Player {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Player {
        let rect = Rect::new(
            (WIDTH as i32 / 2) - (PLAYER_WIDTH as i32 / 2),
            HEIGHT as i32 - 15,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );
        Player {
            canvas,
            rect,
        }
    }

    pub fn translate(&mut self, vel: i32) {
        let mut canvas = self.canvas.borrow_mut();
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.fill_rect(self.rect).unwrap();
        drop(canvas);

        self.rect.x += vel;

        self.draw();
    }

    pub fn draw(&self) {
        let mut canvas = self.canvas.borrow_mut();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(self.rect).unwrap();
    }
}
