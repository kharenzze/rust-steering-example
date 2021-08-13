use ggez::graphics::{
  window, Color, DrawMode, DrawParam, MeshBuilder, PxScale, Rect, Text, TextFragment,
};
use ggez::{graphics, timer, Context, GameResult};
use glam::*;
use std::time::Duration;

#[derive(Debug)]
pub struct Notification {
  text: String,
  display_time: Duration,
  display_interval: Duration,
}

impl Default for Notification {
  fn default() -> Self {
    Self {
      text: "Some text".to_string(),
      display_time: Duration::from_secs(1_000_000),
      display_interval: Duration::from_secs(3),
    }
  }
}

impl Notification {
  fn should_display(&self, ctx: &Context) -> bool {
    let time = timer::time_since_start(ctx);
    time < (self.display_time + self.display_interval)
  }

  #[inline]
  fn bg_color() -> Color {
    Color::from_rgba_u32(0x44444480)
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let size = window(ctx).inner_size();
    let h = size.height as f32;
    let rect = Rect::new(0.0, 0.0, h / 2.0, h / 2.0);
    let mut mb = MeshBuilder::new();
    mb.rectangle(DrawMode::fill(), rect, Self::bg_color())?;
    let mesh = mb.build(ctx)?;
    graphics::draw(ctx, &mesh, DrawParam::default())?;

    let text = Text::new(TextFragment {
      text: self.text.clone(),
      color: Some(Color::WHITE),
      font: Some(graphics::Font::default()),
      scale: Some(PxScale::from(30.0)),
    });
    graphics::queue_text(ctx, &text, Vec2::new(50.0, 50.0), None);
    graphics::draw_queued_text(
      ctx,
      DrawParam::default(),
      None,
      graphics::FilterMode::Linear,
    )
  }
}
