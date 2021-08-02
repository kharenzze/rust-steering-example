use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::event::{self, EventHandler};
use glam::*;

mod target;

use target::Target;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

fn main() {
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
}

impl MainState {
  pub fn new(_ctx: &mut Context) -> MainState {
    // Load/create resources such as images here.
    MainState {
      target: Target::new(Vec2::new(500.0, 500.0))
    }
  }
}

impl EventHandler<ggez::GameError> for MainState {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Update code here...
    self.target.update(ctx)?;
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
    graphics::present(ctx)
  }
}
