use custom_error::custom_error;

custom_error! { pub RobotError
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