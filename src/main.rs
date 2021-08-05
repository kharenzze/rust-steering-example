mod bot;
mod target;

use bot::{Bot, StateUpdate};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use glam::*;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use target::Target;

const DIM_1080: Vec2 = const_vec2!([1920.0, 1080.0]);
const DIM_720: Vec2 = const_vec2!([1280.0, 720.0]);

fn get_resolution() -> Vec2 {
  if cfg!(windows) {
    DIM_720
  } else {
    DIM_1080
  }
}

fn main() {
  SimpleLogger::new()
    .with_colors(true)
    .with_level(LevelFilter::Error)
    .with_module_level("steering_behaviours", LevelFilter::Debug)
    .init()
    .unwrap();
  info!("Start!");
  let res = get_resolution();
  let window_setup = ggez::conf::WindowSetup::default().title("Steering");
  let window_mode = ggez::conf::WindowMode::default()
    .min_dimensions(res.x, res.y)
    .dimensions(res.x, res.y);
  // Make a Context.
  let (mut ctx, event_loop) = ContextBuilder::new("Steering", "Kharenzze")
    .window_setup(window_setup)
    .window_mode(window_mode)
    .build()
    .expect("aieee, could not create ggez context!");

  // Create an instance of your event handler.
  // Usually, you should provide it with the Context object to
  // use when setting your game up.
  let my_game = MainState::new(&mut ctx, res);

  // Run!
  event::run(ctx, event_loop, my_game);
}
#[derive(Debug)]
pub struct MainState {
  pub target: Target,
  pub bots: [Bot; 4],
  pub resolution: Vec2,
  pub x: usize,
}

impl MainState {
  pub fn new(_ctx: &mut Context, res: Vec2) -> MainState {
    let mut bots: [Bot; 4] = Default::default();
    let x = res.x * 0.7;
    let gap = (res.y - 200.0) / 5.0;
    for (i, b) in bots.iter_mut().enumerate() {
      b.pos.x = x;
      b.pos.y = 100.0 + gap * ((i + 1) as f32);
      b.disabled = i != 0;
    }
    MainState {
      target: Target::new(Vec2::new(500.0, 500.0)),
      bots,
      resolution: res,
      x: 1,
    }
  }
}

impl EventHandler<ggez::GameError> for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    let ref imm = *self;
    let bot_updates: Vec<StateUpdate> = imm
      .bots
      .iter()
      .map(|b| b.calculate_steering_impulse(imm))
      .collect();
    self.target.update(ctx)?;
    for (i, b) in self.bots.iter_mut().enumerate() {
      b.update(ctx, &bot_updates[i])?;
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, Color::BLACK);
    self.target.draw(ctx)?;
    for b in self.bots.iter_mut() {
      b.draw(ctx)?;
    }
    graphics::present(ctx)
  }

  fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    self.target.mouse_button_down_event(ctx, button, x, y);
  }
}
