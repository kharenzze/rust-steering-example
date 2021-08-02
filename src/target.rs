use log::debug;
use ggez::event::{MouseButton, EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};
use glam::*;

#[derive(Debug, Default)]
pub struct Target {
  pos: Vec2,
}

impl Target {
  pub fn new(pos: Vec2) -> Self {
    Target { pos }
  }
}

impl EventHandler<ggez::GameError> for Target {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }
  
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let mut mb = graphics::MeshBuilder::new();
    mb.circle(DrawMode::fill(), self.pos, 20.0, 1.0, Color::YELLOW)?;
    let mesh = mb.build(ctx)?;
    // Draw code here...
    graphics::draw(ctx, &mesh, DrawParam::default())
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    if button == MouseButton::Left {
      self.pos = Vec2::new(x, y)
    }
    debug!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
  }
}
