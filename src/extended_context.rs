use ggez::Context;
use ggez::graphics::window;
use glam::Vec2;

pub trait ExtendedContext {
  fn inner_size(&self) -> Vec2;
}

impl ExtendedContext for Context {
  fn inner_size(&self) -> Vec2 {
    let s = window(self).inner_size();
    Vec2::new(s.width as f32, s.height as f32)
  }
}