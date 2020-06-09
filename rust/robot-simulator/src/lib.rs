// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, direction: d }
    }

    pub fn turn_right(self) -> Self {
        let direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };
        Self { direction, ..self }
    }

    pub fn turn_left(self) -> Self {
        let direction = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        };
        Self { direction, ..self }
    }

    pub fn advance(self) -> Self {
        let (x, y) = match self.direction {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };
        Self { x, y, ..self }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robo, ch| match ch {
            'R' => robo.turn_right(),
            'L' => robo.turn_left(),
            'A' => robo.advance(),
            _ => unreachable!(),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
