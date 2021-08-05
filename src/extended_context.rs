use ggez::Context;
use ggez::graphics::{window, Rect};
use glam::Vec2;

pub trait ExtendedContext {
  fn inner_size(&self) -> Vec2;
  fn view_rect(&self) -> Rect;
}

impl ExtendedContext for Context {
  fn inner_size(&self) -> Vec2 {
    let s = window(self).inner_size();
    Vec2::new(s.width as f32, s.height as f32)
  }

  fn view_rect(&self) -> Rect {
    let s = self.inner_size();
    Rect::new(0.0, 0.0, s.x, s.y)
  }
}