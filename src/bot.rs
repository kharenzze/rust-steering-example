use crate::extensions::{ExtendedContext, RandNonZero};
use crate::MainState;
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{timer, Context, GameResult};
use glam::*;
use std::time::Duration;

const MAX_SPEED: f32 = 10.0;
const MAX_IMPULSE: f32 = 3.0;
const RADIO_RATIO: f32 = 1.0 / 36.0;
const FRICTION: f32 = 0.10;

#[derive(Debug, Clone, Copy)]
pub struct WanderProps {
  radius: f32,
  center: f32,
  interval: Duration,
}

impl Default for WanderProps {
  fn default() -> Self {
    Self {
      radius: 2.0,
      center: 3.0,
      interval: Duration::from_millis(100),
    }
  }
}

#[derive(Debug)]
pub enum SteeringBehaviour {
  SimpleSeek,
  SimpleFlee,
  SeekAndArrive(f32),
  Flee(f32),
  Wander(WanderProps),
  SeekSquad(f32, f32)
}

impl std::fmt::Display for SteeringBehaviour {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let t = match *self {
      SteeringBehaviour::SeekAndArrive(_) => "Seek and Arrive",
      SteeringBehaviour::SimpleFlee => "Simple Flee",
      SteeringBehaviour::SimpleSeek => "Simple Seek",
      SteeringBehaviour::Flee(_) => "Flee",
      SteeringBehaviour::Wander(_) => "Wander",
      SteeringBehaviour::SeekSquad(_, _) => "Seek Squad",
    };
    write!(f, "{}", t)
  }
}

#[derive(Debug, Default)]
pub struct Bot {
  pub pos: Vec2,
  speed: Vec2,
  desired_speed: Vec2,
  last_wander: Duration,
  pub disabled: bool,
  pub id: usize,
}

#[derive(Debug, Default)]
pub struct StateUpdate {
  desired_speed: Vec2,
  steering_impulse: Vec2,
  last_wander: Option<Duration>,
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
    if let Some(t) = state_update.last_wander {
      self.last_wander = t;
    }
    self.desired_speed = state_update.desired_speed;
    self.speed += state_update.steering_impulse;
    self.speed = self.speed.clamp_length_max(MAX_SPEED) * (1.0 - FRICTION);
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
    match state.steering_behaviour_target {
      SteeringBehaviour::SimpleSeek => self.calculate_seek_and_arrive(state, 0.0),
      SteeringBehaviour::SimpleFlee => self.calculate_flee(state, 10000.0),
      SteeringBehaviour::SeekAndArrive(radius) => self.calculate_seek_and_arrive(state, radius),
      SteeringBehaviour::Wander(wander_props) => self.calculate_wander(ctx, wander_props),
      SteeringBehaviour::Flee(rad) => self.calculate_flee(state, rad),
      SteeringBehaviour::SeekSquad(target_rad, flee_rad) => self.calculate_flee(state, target_rad),
    }
  }

  fn calculate_wander(&self, ctx: &Context, wp: WanderProps) -> StateUpdate {
    let desired_speed = Vec2::ZERO; //it's random
    let time = timer::time_since_start(ctx);
    let skip = time - self.last_wander < wp.interval;
    if skip {
      return StateUpdate {
        desired_speed,
        steering_impulse: Vec2::ZERO,
        last_wander: None,
      };
    }
    let speed = self.speed.try_normalize().unwrap_or(Vec2::rand_unitary());
    let speed = speed * wp.center + Vec2::rand_unitary() * wp.radius;
    let steering_impulse = (speed - self.speed).clamp_length_max(MAX_IMPULSE);
    StateUpdate {
      desired_speed,
      steering_impulse,
      last_wander: Some(time),
    }
  }

  fn calculate_seek_and_arrive(&self, state: &MainState, radius: f32) -> StateUpdate {
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
      last_wander: None,
    }
  }

  fn calculate_flee(&self, state: &MainState, radius: f32) -> StateUpdate {
    let diff = self.pos - state.target.pos;
    if diff.length_squared() > (radius * radius) {
      return StateUpdate {
        desired_speed: Vec2::ZERO,
        steering_impulse: Vec2::ZERO,
        last_wander: None,
      };
    }
    let safe_diff = if diff.length_squared() < 0.1 {
      Vec2::rand_unitary()
    } else {
      diff
    };
    let desired_speed = safe_diff.clamp_length(MAX_SPEED, MAX_SPEED);
    let steering_impulse = (desired_speed - self.speed).clamp_length_max(MAX_IMPULSE);
    StateUpdate {
      desired_speed,
      steering_impulse,
      last_wander: None,
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
