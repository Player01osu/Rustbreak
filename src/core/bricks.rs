use sdl2::video::Window;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use crate::{
    Entity, BACKGROUND_COLOR, GRID_RATIO_D, GRID_RATIO_N, HEIGHT, MAX_COL, MAX_ROW, WIDTH, GRID_HEIGHT, GRID_WIDTH, BALL_RADIUS,
};

// Both row and col are zero indexed
#[derive(Debug)]
pub struct Brick {
    pub rect: Rect,
    pub row: usize,
    pub alive: bool,
}

pub struct Bricks {
    pub bricks: Vec<Vec<Brick>>,
}

impl Entity for Brick {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        if self.alive {
            let color = self.row as u8 / 2;
            canvas.set_draw_color(Color::RGB(
                color.wrapping_add(90).wrapping_mul(40),
                color.wrapping_mul(20).wrapping_sub(12),
                color.wrapping_add(230).wrapping_mul(60),
            ));
            canvas.fill_rect(self.rect).unwrap();
        } else {
            canvas.set_draw_color(BACKGROUND_COLOR);
            canvas.fill_rect(self.rect).unwrap();
        }
    }

    fn kill(&mut self, _canvas: &mut Canvas<Window>) {
        todo!()
    }
}

impl Entity for Bricks {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for col in self.bricks.iter_mut() {
            for brick in col {
                brick.draw(canvas);
            }
        }
    }

    fn kill(&mut self, _canvas: &mut Canvas<Window>) {
        todo!()
    }
}

pub enum Touch {
    Top,
    Side,
    None,
}

impl Bricks {
    pub fn new() -> Bricks {
        let mut bricks: Vec<Vec<Brick>> = Vec::with_capacity(MAX_ROW);

        for i in 0..MAX_ROW {
            let mut col = Vec::with_capacity(MAX_ROW);
            for j in 0..MAX_COL {
                col.push(Brick::new(i, j));
            }
            bricks.push(col);
        }

        Bricks { bricks }
    }
}

impl Brick {
    pub fn new(row: usize, col: usize) -> Brick {
        let number_of_rows = MAX_ROW;
        let number_of_cols = MAX_COL;
        let inner_pad = 5;
        let outer_pad = BALL_RADIUS as i32 * 3 / 4;
        let grid_width = GRID_WIDTH;
        let grid_height = GRID_HEIGHT;

        let grid_ratio = |n| (n * GRID_RATIO_N) / GRID_RATIO_D;

        let grid_height = grid_ratio(grid_height);
        let grid_width = grid_width;

        let brick_width = (grid_width
            - (outer_pad as u32 * 2)
            - (inner_pad as u32 * (number_of_cols as u32 - 1)))
            / number_of_cols as u32;

        let brick_height = (grid_height
            - (outer_pad as u32 * 2)
            - (inner_pad as u32 * (number_of_rows as u32 - 1)))
            / number_of_rows as u32;

        let brick_x = (outer_pad + (col as i32 * inner_pad)) + (col as i32 * brick_width as i32);
        let brick_y = (outer_pad + (row as i32 * inner_pad)) + (row as i32 * brick_height as i32);

        let rect = Rect::new(brick_x, brick_y, brick_width, brick_height);

        Brick {
            rect,
            row,
            alive: true,
        }
    }

    pub fn x(&self) -> i32 {
        self.rect.x
    }

    pub fn y(&self) -> i32 {
        self.rect.y
    }

    pub fn width(&self) -> u32 {
        self.rect.width()
    }

    pub fn height(&self) -> u32 {
        self.rect.height()
    }

    pub fn dead(&mut self) {
        self.alive = false;
    }
}
