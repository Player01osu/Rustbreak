use self::{ball::Ball, bricks::Bricks, player::Player};

pub mod ball;
pub mod bricks;
pub mod player;

pub fn physics(brick: &mut Bricks, ball: &mut Ball, player: &mut Player) {
    ball.physics();
}
