use crate::{Direction, Field, Position, RobotError};

pub struct Robot {
  pub position: Position,
  pub direction: Direction,
}

impl Robot {
  /// Caclulate the next position for the robot
  pub fn next_position(&self) -> Position {
    use Direction::*;
    match &self.direction {
      NORTH => Position {
        x: self.position.x,
        y: self.position.y + 1,
      },
      WEST => Position {
        x: self.position.x - 1,
        y: self.position.y,
      },
      EAST => Position {
        x: self.position.x + 1,
        y: self.position.y,
      },
      SOUTH => Position {
        x: self.position.x,
        y: self.position.y - 1,
      },
    }
  }
  /// Move robot, fail if goes out of field
  pub fn move_it(&mut self, field: &Field) -> Result<(), RobotError> {
    let pos = self.next_position();
    if !field.position_in_field(&pos) {
      return Err(RobotError::RobotOutOfField);
    }
    self.position = pos;
    Ok(())
  }
  /// get report for the current status of robot
  pub fn report(&self) -> String {
    format!("{},{},{}", self.position.x, self.position.y, self.direction)
  }
}
