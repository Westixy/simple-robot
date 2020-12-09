use crate::Position;

pub struct Field {
  pub width: isize,
  pub height: isize,
}

impl Field {
  /// to know if the given position is in field
  pub fn position_in_field(&self, pos: &Position) -> bool {
    pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
  }
}
