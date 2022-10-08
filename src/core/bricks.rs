use std::cell::RefCell;
use std::rc::Rc;

use sdl2::video::Window;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use crate::{BACKGROUND_COLOR, WIDTH};

pub const BRICK_WIDTH: u32 = 100;
pub const BRICK_HEIGHT: u32 = 40;

pub const MAX_COL: i32 = 6;
pub const MAX_ROW: i32 = 5;

// Both row and col are zero indexed
#[derive(Debug)]
pub struct Brick {
    pub rect: Rect,
    row: i32,
    col: i32,
    pub alive: bool,
}

pub struct Bricks {
    pub brick: Vec<Brick>,
    canvas: Rc<RefCell<Canvas<Window>>>,
}

pub enum Touch {
    Top,
    Side,
    None,
}

impl Bricks {
    pub fn new(canvas: Rc<RefCell<Canvas<Window>>>) -> Bricks {
        let mut brick: Vec<Brick> = Vec::new();

        let mut row = 0;
        for i in 0..(MAX_COL * MAX_ROW) {
            if i % MAX_COL == 0 {
                row += 1;
            }
            brick.push(Brick::new(i % MAX_COL, row));
        }

        Bricks { brick, canvas }
    }

    pub fn draw(&mut self) {
        for brick in self.brick.iter_mut() {
            brick.draw(&mut self.canvas.borrow_mut());
        }
    }

    pub fn intersect_ball(&mut self, y: i32, x: i32) -> Touch {
        for brick in self.brick.iter_mut() {
            match brick.intersect_ball(20, y, x) {
                Touch::Top => {
                    return Touch::Top
                }
                Touch::Side => {
                    return Touch::Side
                }
                _ => ()
            }
        }
        Touch::None
    }

    pub fn kill(&mut self, col: i32, row: i32) {
        let brick = &mut self.brick[(col + (row * MAX_COL)) as usize];
        brick.alive = false;
        self.draw();
    }
}

impl Brick {
    pub fn new(col: i32, row: i32) -> Brick {
        const LEFT_OFFSET: i32 = 80;
        const TOP_OFFSET: i32 = 20;

        let rect = Rect::new(
            (col * (BRICK_WIDTH as i32 + 9)) + LEFT_OFFSET,
            (row * (BRICK_HEIGHT as i32 + 9)) + TOP_OFFSET,
            BRICK_WIDTH,
            BRICK_HEIGHT,
        );

        Brick {
            rect,
            col,
            row: row - 1,
            alive: true,
        }
    }

    pub fn x(&self) -> i32 {
        self.rect.x
    }

    pub fn y(&self) -> i32 {
        self.rect.y
    }

    pub fn intersect_ball(&mut self, radius: i32, y: i32, x: i32) -> Touch {
        //if self.rect.x() <  radius || (self.rect.x() - BRICK_WIDTH as i32) > radius {
        //    return true
        //}
        if y - radius < self.rect.y() + BRICK_HEIGHT as i32 {
            return Touch::Top
        }

        //if x - radius < self.rect.x() + BRICK_WIDTH as i32 {
        //    return Touch::Side
        //}

        Touch::None
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        if self.alive {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.fill_rect(self.rect).unwrap();
        } else {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.fill_rect(self.rect).unwrap();
        }
    }
}
