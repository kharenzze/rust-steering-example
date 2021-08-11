use crate::extended_context::ExtendedContext;
use crate::MainState;
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};
use glam::*;

const MAX_SPEED: f32 = 10.0;
const MAX_IMPULSE: f32 = 3.0;
const RADIO_RATIO: f32 = 1.0 / 36.0;

#[derive(Debug)]
pub enum SteeringBehaviour {
  SimpleSeek,
  SimpleFlee,
  SeekAndArrive(f32),
  Wander(f32, f32),
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
  #[inline]
  pub fn get_radius(&self, ctx: &mut Context) -> f32 {
    ctx.inner_size().y * RADIO_RATIO
  }

  pub fn update(&mut self, ctx: &mut Context, state_update: &StateUpdate) -> GameResult<()> {
    if self.disabled {
      return Ok(());
    }
    self.desired_speed = state_update.desired_speed;
    self.speed += state_update.steering_impulse;
    self.speed = self.speed.clamp_length_max(MAX_SPEED);
    self.pos += self.speed;
    self.enforce_pos_inside_view(ctx);
    Ok(())
  }

  fn enforce_pos_inside_view(&mut self, ctx: &Context) {
    let rect = ctx.view_rect();
    if !rect.contains(self.pos) {
      self.pos = Vec2::new(
        self.pos.x.clamp(rect.x, rect.w),
        self.pos.y.clamp(rect.y, rect.h),
      )
    }
  }

  pub fn calculate_steering_impulse(&self, state: &MainState, ctx: &Context) -> StateUpdate {
    match state.steering_behaviour {
      SteeringBehaviour::SimpleSeek => self.calculate_seek_and_arrive(state, 0.0),
      SteeringBehaviour::SimpleFlee => self.calculate_simple_flee(state),
      SteeringBehaviour::SeekAndArrive(radius) => self.calculate_seek_and_arrive(state, radius),
      SteeringBehaviour::Wander(dist, rad) => self.calculate_wander(state, dist, rad),
    }
  }

  pub fn calculate_wander(&self, state: &MainState, dist: f32, radius: f32) -> StateUpdate {
    let distance_vector = state.target.pos - self.pos;
    let mut desired_speed = distance_vector.clamp_length_max(MAX_SPEED);
    let steering_impulse = (desired_speed - self.speed).clamp_length_max(MAX_IMPULSE);
    StateUpdate {
      desired_speed,
      steering_impulse,
    }
  }

  pub fn calculate_seek_and_arrive(&self, state: &MainState, radius: f32) -> StateUpdate {
    let distance_vector = state.target.pos - self.pos;
    let mut desired_speed = distance_vector.clamp_length_max(MAX_SPEED);
    let distance = distance_vector.length();
    if distance < radius {
      desired_speed = desired_speed * (distance / radius);
    }
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
    let mut mb = graphics::MeshBuilder::new();
    mb.circle(
      DrawMode::fill(),
      self.pos,
      self.get_radius(ctx),
      1.0,
      Color::WHITE,
    )?;
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
