use ggez::Context;
use ggez::graphics::{window, Rect};
use glam::Vec2;
use rand::random;
use std::f32::consts::PI;

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

pub trait RandNonZero {
  fn rand_unitary() -> Self;
}

impl RandNonZero for Vec2 {
  fn rand_unitary() -> Self {
    let angle:f32 = random::<f32>() * 2.0 * PI;
    Vec2::new(angle.cos(), angle.sin())
  }
}