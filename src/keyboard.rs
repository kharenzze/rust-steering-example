use ggez::event::KeyCode;
use glam::Vec2;
use std::convert::TryFrom;

pub enum Direction {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}

impl From<Direction> for Vec2 {
  fn from(d: Direction) -> Self {
    match d {
      Direction::Down => Vec2::new(0.0, 1.0),
      Direction::Up => Vec2::new(0.0, -1.0),
      Direction::Right => Vec2::new(1.0, 0.0),
      Direction::Left => Vec2::new(-1.0, 0.0),
    }
  }
}

impl TryFrom<KeyCode> for Direction {
  type Error = ();
  fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
    match value {
      KeyCode::W => Ok(Direction::Up),
      KeyCode::D => Ok(Direction::Right),
      KeyCode::S => Ok(Direction::Down),
      KeyCode::A => Ok(Direction::Left),
      _ => Err(()),
    }
  }
}

#[derive(Debug)]
pub struct DirPressedStatus([bool; 4]);

impl Default for DirPressedStatus {
  fn default() -> Self {
    Self([false; 4])
  }
}

pub trait DirectionKeyHandler {
  fn get_mut_dir_pressed_status(&mut self) -> &mut DirPressedStatus;
  fn on_dir_key_pressed(&mut self, k: KeyCode) {
    let st = self.get_mut_dir_pressed_status();
    if let Ok(d) = Direction::try_from(k) {
      st.0[d as usize] = true;
    }
  }
  fn on_dir_key_released(&mut self, k: KeyCode) {
    let st = self.get_mut_dir_pressed_status();
    if let Ok(d) = Direction::try_from(k) {
      st.0[d as usize] = false;
    }
  }
}
