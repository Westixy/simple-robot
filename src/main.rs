mod actions;
mod errors;
mod field;
mod game;
mod position;
mod robot;

pub use crate::actions::{Action, RobotRequiredAction};
pub use crate::errors::RobotError;
pub use crate::field::Field;
pub use crate::game::Game;
pub use crate::position::{Direction, Position};
pub use crate::robot::Robot;

fn main() {
  // run tests to try
  let mut game = Game::new(5, 5);
  println!("Game initialised");
  let output = game
    .execute_all(
      "PLACE 0,0,NORTH
       MOVE
       REPORT
       MOVE
       RIGHT
       MOVE
       REPORT
       RIGHT
       MOVE
       REPORT
       RIGHT
       MOVE
       REPORT",
    )
    .expect("robot not working :(");
  println!("Output:\n{}", output);
}
