use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::event::{self, EventHandler, MouseButton};
use simple_logger::SimpleLogger;
use log::{LevelFilter, info};
use glam::*;

mod target;
mod bot;

use target::Target;
use bot::Bot;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

fn main() {
  SimpleLogger::new()
    .with_colors(true)
    .with_level(LevelFilter::Error)
    .with_module_level("steering_behaviours", LevelFilter::Debug)
    .init()
    .unwrap();
  info!("Start!");
  let window_setup = ggez::conf::WindowSetup::default()
  .title("Steering");
  let window_mode = ggez::conf::WindowMode::default()
  .min_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
  .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);
  // Make a Context.
  let (mut ctx, event_loop) = ContextBuilder::new("Steering", "Kharenzze")
  .window_setup(window_setup)
  .window_mode(window_mode)
  .build()
  .expect("aieee, could not create ggez context!");
  
  // Create an instance of your event handler.
  // Usually, you should provide it with the Context object to
  // use when setting your game up.
  let my_game = MainState::new(&mut ctx);
  
  // Run!
  event::run(ctx, event_loop, my_game);
}

struct MainState {
  target: Target,
  bots: [Bot; 4],
}

impl MainState {
  pub fn new(_ctx: &mut Context) -> MainState {
    let mut bots: [Bot; 4] = Default::default();
    let x = WINDOW_WIDTH * 0.7;
    let gap = (WINDOW_HEIGHT - 200.0) / 5.0;
    for (i, b) in bots.iter_mut().enumerate() {
      b.pos.x = x;
      b.pos.y = 100.0 + gap * ((i + 1) as f32);
    }
    MainState {
      target: Target::new(Vec2::new(500.0, 500.0)),
      bots
    }
  }
}

impl EventHandler<ggez::GameError> for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Update code here...
    self.target.update(ctx)?;
    for b in self.bots.iter_mut() {
      b.update(ctx)?;
    }
    Ok(())
  }
  
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, Color::BLACK);
    let mut mb = graphics::MeshBuilder::new();
    mb.circle(
      DrawMode::fill(),
      Vec2::new(600.0, 380.0),
      40.0,
      1.0,
      Color::RED
    )?;
    let mesh = mb.build(ctx)?;
    // Draw code here...
    graphics::draw(ctx, &mesh, DrawParam::default())?;
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
