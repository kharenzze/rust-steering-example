use std::time::Duration;
use ggez::{Context, timer};

#[derive(Debug)]
pub struct Notification {
  text: String,
  display_time: Duration,
  display_interval: Duration,
}

impl Default for Notification {
  fn default() -> Self {
    Self {
      text: "".to_string(),
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
}