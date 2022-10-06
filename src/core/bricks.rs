use std::cell::RefCell;
use std::rc::Rc;

use sdl2::video::Window;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

pub const BRICK_WIDTH: u32 = 100;
pub const BRICK_HEIGHT: u32 = 40;

pub struct Brick {
    rect: Rect,
    row: i32,
    col: i32,
    pub alive: bool,
}

pub struct Bricks {
    brick: Vec<Brick>,
    canvas: Rc<RefCell<Canvas<Window>>>,
}

pub const MAX_ROW: i32 = 5;
pub const MAX_COL: i32 = 6;

impl Bricks {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Bricks {
        let mut brick: Vec<Brick> = Vec::new();

        let mut row = 0;
        for i in 0..(MAX_ROW * MAX_ROW) {
            if i % MAX_ROW == 0 {
                row += 1;
            }
            brick.push(Brick::new(row, i % MAX_ROW));
        }

        Bricks { brick, canvas }
    }

    pub fn draw(&mut self) {
        for brick in self.brick.iter_mut() {
            brick.draw(&mut self.canvas.borrow_mut());
        }
    }

    pub fn kill(&mut self, row: i32, col: i32) {
        self.brick.iter().nth((row * col) as usize).unwrap();
    }
}

impl Brick {
    pub fn new(row: i32, col: i32) -> Brick {
        let rect = Rect::new((row * 109) + 10, (col * 49) + 10, BRICK_WIDTH, BRICK_HEIGHT);

        Brick {
            rect,
            row,
            col,
            alive: true,
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        if self.alive {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(self.rect).unwrap();
        } else {
            canvas.set_draw_color(Color::RGB(0, 255, 255));
            canvas.fill_rect(self.rect).unwrap();
        }
    }
}
