use log::debug;
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};
use glam::*;

use crate::target::Target;

const MAX_SPEED: f32 = 10.0;
const MAX_IMPULSE: f32 = 3.0;

#[derive(Debug, Default)]
pub struct Bot {
  pub pos: Vec2,
  speed: Vec2,
  desired_speed: Vec2,
  pub disabled: bool,
}

impl Bot {
  pub fn update(&mut self, _ctx: &mut Context, target: &Target) -> GameResult<()> {
    if self.disabled {
      return Ok(())
    }
    self.desired_speed = (target.pos - self.pos)
      .clamp_length_max(MAX_SPEED);
    let steering_impulse = (self.desired_speed - self.speed)
      .clamp_length_max(MAX_IMPULSE);
    self.speed += steering_impulse;
    self.pos += self.speed;
    Ok(())
  }
  
  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.disabled {
      return Ok(())
    }
    let mut mb = graphics::MeshBuilder::new();
    mb.circle(DrawMode::fill(), self.pos, 30.0, 1.0, Color::WHITE)?;
    mb.line(&[self.pos, self.pos + self.speed * 10.0], 2.0, Color::RED)?;
    mb.line(&[self.pos, self.pos + self.desired_speed * 10.0], 2.0, Color::GREEN)?;
    let mesh = mb.build(ctx)?;
    // Draw code here...
    graphics::draw(ctx, &mesh, DrawParam::default())
  }
}