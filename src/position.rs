use crate::RobotError;
use std::str::FromStr;

#[derive(Debug, Eq)]
pub struct Position {
  pub x: isize,
  pub y: isize,
}

impl PartialEq for Position {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == self.y
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
  NORTH,
  WEST,
  EAST,
  SOUTH,
}
impl std::fmt::Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use Direction::*;
    write!(
      f,
      "{}",
      match self {
        NORTH => "NORTH",
        WEST => "WEST",
        EAST => "EAST",
        SOUTH => "SOUTH",
      }
    )
  }
}

impl Direction {
  /// get direction at -90 deg
  pub fn get_left(&self) -> Direction {
    use Direction::*;
    match self {
      NORTH => WEST,
      WEST => SOUTH,
      SOUTH => EAST,
      EAST => NORTH,
    }
  }

  /// get direction at +90 deg
  pub fn get_right(&self) -> Direction {
    use Direction::*;
    match self {
      NORTH => EAST,
      WEST => NORTH,
      SOUTH => WEST,
      EAST => SOUTH,
    }
  }
}

impl FromStr for Direction {
  type Err = RobotError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let val = s.trim();
    match val {
      "NORTH" => Ok(Direction::NORTH),
      "WEST" => Ok(Direction::WEST),
      "EAST" => Ok(Direction::EAST),
      "SOUTH" => Ok(Direction::SOUTH),
      _ => Err(RobotError::ParseDirectionError {
        direction: val.to_string(),
      }),
    }
  }
}
