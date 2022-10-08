use self::{bricks::Bricks, ball::Ball, player::Player};

pub mod bricks;
pub mod ball;
pub mod player;

pub fn physics(brick: &mut Bricks, ball: &mut Ball, player: &mut Player) {
    ball.physics();
}
