use crate::{Direction, Position, RobotError};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
  /// Put robot on the field
  PLACE(Position, Direction),
  /// Show current status of robot
  REPORT,
  NeedRobot(RobotRequiredAction),
}

impl FromStr for Action {
  type Err = RobotError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let val = s.trim();
    let place_regex = Regex::new(r"PLACE (\d+),(\d+),(NORTH|WEST|SOUTH|EAST)").unwrap();
    match val {
      "MOVE" => Ok(Action::NeedRobot(RobotRequiredAction::MOVE)),
      "LEFT" => Ok(Action::NeedRobot(RobotRequiredAction::LEFT)),
      "RIGHT" => Ok(Action::NeedRobot(RobotRequiredAction::RIGHT)),
      "REPORT" => Ok(Action::REPORT),
      _ => {
        if let Some(cap) = place_regex.captures(val) {
          Ok(Action::PLACE(
            Position {
              x: cap[1].parse()?,
              y: cap[2].parse()?,
            },
            cap[3].parse::<Direction>()?,
          ))
        } else {
          Err(RobotError::ParseActionError {
            action: val.to_string(),
          })
        }
      }
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RobotRequiredAction {
  /// Move robot one tile on the current direction
  MOVE,
  /// rotate left
  LEFT,
  /// rotate right
  RIGHT,
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn parse_place() {
    let action: Action = "PLACE 0,0,NORTH".parse().unwrap();
    assert_eq!(
      action,
      Action::PLACE(Position { x: 0, y: 0 }, Direction::NORTH)
    );
  }

  #[test]
  fn parse_place_west() {
    let action: Action = "PLACE 2,13,WEST".parse().unwrap();
    assert_eq!(
      action,
      Action::PLACE(Position { x: 2, y: 13 }, Direction::WEST)
    );
  }

  #[test]
  fn parse_move() {
    let action: Action = "MOVE".parse().unwrap();
    assert_eq!(action, Action::NeedRobot(RobotRequiredAction::MOVE))
  }

  #[test]
  fn parse_left() {
    let action: Action = "LEFT ".parse().unwrap();
    assert_eq!(action, Action::NeedRobot(RobotRequiredAction::LEFT))
  }

  #[test]
  fn parse_right() {
    let action: Action = " RIGHT".parse().unwrap();
    assert_eq!(action, Action::NeedRobot(RobotRequiredAction::RIGHT))
  }

  #[test]
  fn parse_report() {
    let action: Action = "REPORT".parse().unwrap();
    assert_eq!(action, Action::REPORT)
  }

  #[test]
  #[should_panic]
  fn parse_should_fail() {
    "PLACE 1.1,NORTH".parse::<Action>().unwrap();
  }

  #[test]
  #[should_panic]
  fn parse_should_fail_2() {
    "PLACE NORTH ".parse::<Action>().unwrap();
  }
}