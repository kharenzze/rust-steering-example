use ggez::event::{EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};
use glam::*;

#[derive(Debug, Default)]
pub struct Bot {
  pub pos: Vec2,
}

impl Bot {
  pub fn new(pos: Vec2) -> Self {
    Bot { pos }
  }
}

impl EventHandler<ggez::GameError> for Bot {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
  
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut mb = graphics::MeshBuilder::new();
    mb.circle(DrawMode::fill(), self.pos, 30.0, 1.0, Color::WHITE)?;
    let mesh = mb.build(ctx)?;
    // Draw code here...
    graphics::draw(ctx, &mesh, DrawParam::default())
  }
}