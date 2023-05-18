use self::{ball::Ball, bricks::Bricks, player::Player};

pub mod ball;
pub mod bricks;
pub mod player;

pub fn physics(_brick: &mut Bricks, ball: &mut Ball, _player: &mut Player) {
    ball.physics();
}
