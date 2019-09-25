use super::MoveEvent;

#[derive(Debug)]
pub enum Direction {
    Forward,  //z--
    Backward, //z++
    Left,     //x--
    Right,    //x++
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Forward
    }
}

impl Direction {
    pub fn apply_move_event(&mut self, move_event: MoveEvent) {
        use Direction::*;
        use MoveEvent::*;

        *self = match &self {
            Forward => match move_event {
                TurnLeft => Left,
                TurnRight => Right,
            },
            Backward => match move_event {
                TurnLeft => Right,
                TurnRight => Left,
            },
            Left => match move_event {
                TurnLeft => Backward,
                TurnRight => Forward,
            },
            Right => match move_event {
                TurnLeft => Forward,
                TurnRight => Backward,
            },
        };
    }
}
