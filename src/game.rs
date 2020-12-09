use crate::{Action, Field, Robot, RobotError, RobotRequiredAction};

/// Entrypoint for game
pub struct Game {
  field: Field,
  robot: Option<Robot>,
}

impl Game {
  /// Execute an action for the robot
  fn execute(&mut self, action: Action) -> Result<String, RobotError> {
    use Action::*;
    let mut res = String::new();
    match action {
      PLACE(pos, direction) => {
        if self.field.position_in_field(&pos) {
          self.robot = Some(Robot {
            position: pos,
            direction,
          })
        } else {
          return Err(RobotError::RobotOutOfField);
        }
      }
      NeedRobot(action) => {
        if let Some(robot) = &mut self.robot {
          use RobotRequiredAction::*;
          match action {
            MOVE => robot.move_it(&self.field)?,
            LEFT => robot.direction = robot.direction.get_left(),
            RIGHT => robot.direction = robot.direction.get_right(),
            REPORT => res = format!("{}\n", robot.report()),
          }
        } else {
          return Err(RobotError::RobotNotInField);
        }
      }
    }
    Ok(res)
  }

  /// Parse and then execute action from a string
  pub fn execute_all(&mut self, lines: &str) -> Result<String, RobotError> {
    let mut res = String::new();
    for line in lines.split("\n") {
      let action = line.parse::<Action>()?;
      match self.execute(action) {
        Ok(action_result) => res = format!("{}{}", res, action_result),
        Err(RobotError::RobotNotInField) => (),
        Err(RobotError::RobotOutOfField) => (),
        Err(error) => return Err(error),
      }
    }
    Ok(res)
  }

  pub fn new(width: isize, height: isize) -> Self {
    Self {
      field: Field { width, height },
      robot: None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::position::{Direction, Position};

  fn new_game() -> Game {
    Game::new(5, 5)
  }

  #[test]
  fn place_works() {
    let mut game = new_game();
    let pos = Position { x: 0, y: 0 };
    let dir = Direction::NORTH;
    game.execute(Action::PLACE(pos, dir)).unwrap();
    let robot = game.robot.unwrap();
    assert_eq!(robot.direction, Direction::NORTH);
    assert_eq!(robot.position, Position { x: 0, y: 0 });
  }
  #[test]
  fn place_works_parse() {
    let mut game = new_game();
    game.execute("PLACE 0,0,NORTH".parse().unwrap()).unwrap();
  }

  #[test]
  #[should_panic]
  fn place_works_should_fail() {
    let mut game = new_game();
    game.execute("PLACE 0,10,NORTH".parse().unwrap()).unwrap();
  }

  #[test]
  fn example_a() {
    let mut game = new_game();
    let res = game
      .execute_all(
        "PLACE 0,0,NORTH
         MOVE
         REPORT",
      )
      .unwrap();
    assert_eq!(res, "0,1,NORTH\n");
  }

  #[test]
  fn example_b() {
    let mut game = new_game();
    let res = game
      .execute_all(
        "PLACE 0,0,NORTH
         LEFT
         REPORT",
      )
      .unwrap();
    assert_eq!(res, "0,0,WEST\n");
  }

  #[test]
  fn example_c() {
    let mut game = new_game();
    let res = game
      .execute_all(
        "PLACE 1,2,EAST
         MOVE
         MOVE
         LEFT
         MOVE
         REPORT",
      )
      .unwrap();
    assert_eq!(res, "3,3,NORTH\n");
  }

  #[test]
  fn actions_before_place() {
    let mut game = new_game();
    let res = game
      .execute_all(
        "MOVE
         LEFT
         REPORT
         RIGHT
         PLACE 1,2,EAST
         MOVE
         MOVE
         LEFT
         MOVE
         REPORT",
      )
      .unwrap();
    assert_eq!(res, "3,3,NORTH\n");
  }

  #[test]
  fn double_place() {
    let mut game = new_game();
    let res = game
      .execute_all(
        "PLACE 1,2,EAST
         MOVE
         PLACE 3,3,NORTH
         REPORT",
      )
      .unwrap();
    assert_eq!(res, "3,3,NORTH\n");
  }

  #[test]
  fn should_go_out_of_field_and_ignore_actions() {
    let mut game = new_game();
    let res = game
      .execute_all(
        "PLACE 1,2,EAST
         MOVE
         PLACE 6,2,EAST
         MOVE
         MOVE
         MOVE
         MOVE
         MOVE
         REPORT",
      )
      .unwrap();
    assert_eq!(res, "4,2,EAST\n");
  }
}
