use custom_error::custom_error;
use regex::Regex;
use std::str::FromStr;


fn main() {
    // run tests to try
}

custom_error! { RobotError
    RobotNotInField = "Robot is not placed on field",
    RobotOutOfField = "Robot will fall out of the field",
    ParseActionError{action:String} = "Unable to parse action: {action}",
    ParseActionIntError{error: std::num::ParseIntError} = "Unable to parse action because of int: {error}",
    ParseDirectionError{direction: String} = "Unable to parse direction: {direction}",
}
impl std::convert::From<std::num::ParseIntError> for RobotError {
    fn from(error: std::num::ParseIntError) -> Self {
        RobotError::ParseActionIntError { error }
    }
}

/// Entrypoint for game
struct Game {
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
                    }
                } else {
                    return Err(RobotError::RobotNotInField);
                }
            }
            REPORT => {
                if let Some(robot) = &self.robot {
                    res = format!("{}\n", robot.report())
                } else {
                    res = format!("Robot not in field\n");
                }
            }
        }
        Ok(res)
    }

    /// Parse and then execute action from a string
    fn execute_all(&mut self, lines: &str) -> Result<String, RobotError> {
        let mut res = String::new();
        for line in lines.split("\n") {
            let action = line.parse::<Action>()?;
            res = format!("{}{}", res, self.execute(action)?);
        }
        Ok(res)
    }
}

#[derive(Debug, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    NORTH,
    WEST,
    EAST,
    SOUTH,
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::NORTH => "NORTH",
                Direction::WEST => "WEST",
                Direction::EAST => "EAST",
                Direction::SOUTH => "SOUTH",
            }
        )
    }
}

impl Direction {
    /// get direction at -90 deg
    fn get_left(&self) -> Direction {
        use Direction::*;
        match self {
            NORTH => WEST,
            WEST => SOUTH,
            SOUTH => EAST,
            EAST => NORTH,
        }
    }

    /// get direction at +90 deg
    fn get_right(&self) -> Direction {
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

struct Robot {
    position: Position,
    direction: Direction,
}

impl Robot {
    /// Caclulate the next position for the robot
    fn next_position(&self) -> Position {
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
    fn move_it(&mut self, field: &Field) -> Result<(), RobotError> {
        let pos = self.next_position();
        if !field.position_in_field(&pos) {
            return Err(RobotError::RobotOutOfField);
        }
        self.position = pos;
        Ok(())
    }
    /// get report for the current status of robot
    fn report(&self) -> String {
        format!("{},{},{}", self.position.x, self.position.y, self.direction)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
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
enum RobotRequiredAction {
    /// Move robot one tile on the current direction
    MOVE,
    /// rotate left
    LEFT,
    /// rotate right
    RIGHT,
}

struct Field {
    width: isize,
    height: isize,
}

impl Field {
    /// to know if the given position is in field
    fn position_in_field(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn new_game() -> Game {
        Game {
            field: Field {
                width: 5,
                height: 5,
            },
            robot: None,
        }
    }

    #[test]
    fn parse_place() {
        let action = "PLACE 0,0,NORTH".parse::<Action>().unwrap();
        assert_eq!(
            action,
            Action::PLACE(Position { x: 0, y: 0 }, Direction::NORTH)
        );
    }
    #[test]
    fn parse_move() {
        "MOVE".parse::<Action>().unwrap();
    }
    #[test]
    #[should_panic]
    fn parse_should_fail() {
        "PLACE 1.1,NORTH".parse::<Action>().unwrap();
    }

    #[test]
    fn place_works() {
        let mut game = new_game();
        let pos = Position { x: 0, y: 0 };
        let dir = Direction::NORTH;
        game.execute(Action::PLACE(pos, dir)).expect("");
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
        let res = game.execute_all("PLACE 0,0,NORTH
            MOVE
            REPORT").unwrap();
        assert_eq!(res, "0,1,NORTH\n");
    }

    #[test]
    fn example_b() {
        let mut game = new_game();
        let res = game.execute_all("PLACE 0,0,NORTH
            LEFT
            REPORT").unwrap();
        assert_eq!(res, "0,0,WEST\n");
    }

    #[test]
    fn example_c() {
        let mut game = new_game();
        let res = game.execute_all("PLACE 1,2,EAST
            MOVE
            MOVE
            LEFT
            MOVE
            REPORT").unwrap();
        assert_eq!(res, "3,3,NORTH\n");
    }

    #[test]
    #[should_panic]
    fn should_go_out_of_field() {
        let mut game = new_game();
        game.execute_all("PLACE 1,2,EAST
            MOVE
            MOVE
            MOVE
            MOVE
            MOVE
            LEFT
            MOVE
            REPORT").unwrap();
    }

    #[test]
    #[should_panic]
    fn should_fail_move_before_place() {
        let mut game = new_game();
        game.execute_all("MOVE
            PLACE 1,2,EAST
            REPORT").unwrap();
    }
}
