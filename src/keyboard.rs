use ggez::event::KeyCode;
use glam::Vec2;
use std::convert::TryFrom;

#[repr(usize)]
pub enum Direction {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}

impl DirPressedStatus {
  fn iter_direction(&self) -> impl Iterator<Item = (Direction, bool)> {
    let value = self.0;
    (0..4_usize).into_iter().map(move |i| {
      let d: Direction = unsafe {::std::mem::transmute(i)};
      let active = (value >> i) | 1;
      let active = active == 1;
      (d, active)
    })
  }
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

#[derive(Debug, Default)]
pub struct DirPressedStatus(usize);

impl From<&DirPressedStatus> for Vec2 {
  fn from(dir_status: &DirPressedStatus) -> Self {
    let mut v = Vec2::ZERO;
    for (dir, active) in dir_status.iter_direction() {
      if active {
        let vec: Vec2 = dir.into();
        v += vec;
      }
    }
    v
  }
}

pub trait DirectionKeyHandler {
  fn get_mut_dir_pressed_status(&mut self) -> &mut DirPressedStatus;
  fn on_dir_key_pressed(&mut self, k: KeyCode) {
    let st = self.get_mut_dir_pressed_status();
    if let Ok(d) = Direction::try_from(k) {
      let d = d as usize;
      let mask: usize = 1 << d;
      st.0 = st.0 | mask;
    }
  }
  fn on_dir_key_released(&mut self, k: KeyCode) {
    let st = self.get_mut_dir_pressed_status();
    if let Ok(d) = Direction::try_from(k) {
      let d = d as usize;
      let mask: usize = !(1 << d);
      st.0 = st.0 & mask;
    }
  }
}
