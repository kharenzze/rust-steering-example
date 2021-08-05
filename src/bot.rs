use ggez::graphics::{self, Color, DrawMode, DrawParam, window};
use ggez::{Context, GameResult};
use glam::*;

use crate::MainState;

const MAX_SPEED: f32 = 10.0;
const MAX_IMPULSE: f32 = 3.0;

#[derive(Debug)]
pub enum SteeringBehaviour {
  SimpleSeek,
  SimpleFlee,
}

#[derive(Debug, Default)]
pub struct Bot {
  pub pos: Vec2,
  speed: Vec2,
  desired_speed: Vec2,
  pub disabled: bool,
}

#[derive(Debug, Default)]
pub struct StateUpdate {
  desired_speed: Vec2,
  steering_impulse: Vec2,
}

impl Bot {
  pub fn update(&mut self, _ctx: &mut Context, state_update: &StateUpdate) -> GameResult<()> {
    if self.disabled {
      return Ok(());
    }
    self.desired_speed = state_update.desired_speed;
    self.speed += state_update.steering_impulse;
    self.pos += self.speed;
    Ok(())
  }

  pub fn calculate_steering_impulse(&self, state: &MainState) -> StateUpdate {
    match state.steering_behaviour {
      SteeringBehaviour::SimpleSeek => self.calculate_simple_seek(state),
      SteeringBehaviour::SimpleFlee => self.calculate_simple_flee(state),
    }
  }

  pub fn calculate_simple_seek(&self, state: &MainState) -> StateUpdate {
    let desired_speed = (state.target.pos - self.pos).clamp_length_max(MAX_SPEED);
    let steering_impulse = (desired_speed - self.speed).clamp_length_max(MAX_IMPULSE);
    StateUpdate {
      desired_speed,
      steering_impulse,
    }
  }

  pub fn calculate_simple_flee(&self, state: &MainState) -> StateUpdate {
    let diff = self.pos - state.target.pos;
    let safe_diff = if diff.length_squared() < 0.1 {
      Vec2::new(-1.0, -1.0)
    } else {
      diff
    };
    let desired_speed = safe_diff.clamp_length(MAX_SPEED, MAX_SPEED);
    let steering_impulse = (desired_speed - self.speed).clamp_length_max(MAX_IMPULSE);
    StateUpdate {
      desired_speed,
      steering_impulse,
    }
  }

  pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    if self.disabled {
      return Ok(());
    }
    let size = window(ctx).inner_size();
    let h = size.height as f32;
    let mut mb = graphics::MeshBuilder::new();
    mb.circle(DrawMode::fill(), self.pos, h / 36.0, 1.0, Color::WHITE)?;
    mb.line(&[self.pos, self.pos + self.speed * 10.0], 2.0, Color::RED)?;
    mb.line(
      &[self.pos, self.pos + self.desired_speed * 10.0],
      2.0,
      Color::GREEN,
    )?;
    let mesh = mb.build(ctx)?;
    // Draw code here...
    graphics::draw(ctx, &mesh, DrawParam::default())
  }
}
