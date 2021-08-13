use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, window, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};
use glam::*;
use log::debug;

use crate::keyboard::{DirPressedStatus, DirectionKeyHandler};

#[derive(Debug, Default)]
pub struct Target {
  pub pos: Vec2,
  dir_pressed: DirPressedStatus,
}

impl Target {
  pub fn new(pos: Vec2) -> Self {
    Target {
      pos,
      dir_pressed: DirPressedStatus::default()
    }
  }
}

impl DirectionKeyHandler for Target {
  fn get_mut_dir_pressed_status(&mut self) -> &mut DirPressedStatus {
    &mut self.dir_pressed
  }
}

impl EventHandler<ggez::GameError> for Target {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    //debug!("{:?}", &self.dir_pressed);
    let dir: Vec2 = (&self.dir_pressed).into();
    self.pos += dir * 5.0;
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let size = window(ctx).inner_size();
    let h = size.height as f32;
    let mut mb = graphics::MeshBuilder::new();
    mb.circle(DrawMode::fill(), self.pos, h / 72.0, 1.0, Color::YELLOW)?;
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
