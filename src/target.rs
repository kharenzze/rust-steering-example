use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::event::{EventHandler};
use glam::*;

pub struct Target {
  pos: Vec2,
}

impl Target {
  pub fn new(pos: Vec2) -> Self {
    Target {
      pos
    }
  }
}

impl EventHandler<ggez::GameError> for Target {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
      // Update code here...
      Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
      let mut mb = graphics::MeshBuilder::new();
      mb.circle(
          DrawMode::fill(),
          self.pos,
          20.0,
          1.0,
          Color::YELLOW
      )?;
      let mesh = mb.build(ctx)?;
      // Draw code here...
      graphics::draw(ctx, &mesh, DrawParam::default())
  }
}