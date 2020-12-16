pub enum Instruction {
    NORTH(usize),
    SOUTH(usize),
    EAST(usize),
    WEST(usize),
    LEFT(usize),
    RIGHT(usize),
    FORWARD(usize),
}

/**
 * 0 degrees is north, 90 degrees east
 */
pub struct Ship {
    degrees: usize,
    x: isize,
    y: isize,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            degrees: 90,
            x: 0,
            y: 0,
        }
    }

    pub fn get_manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[test]
fn navigate_should_work_for_example1() {
    let instructions = vec![
        Instruction::FORWARD(10),
        Instruction::NORTH(3),
        Instruction::FORWARD(7),
        Instruction::RIGHT(90),
        Instruction::FORWARD(11),
    ];
    let mut ship = Ship::new();
    ship.navigate(&instructions);
    assert_eq!(ship.get_manhattan_distance(), 25);
}

impl Ship {
    pub fn navigate(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions {
            match instruction {
                Instruction::NORTH(n) => self.y += *n as isize,
                Instruction::SOUTH(n) => self.y -= *n as isize,
                Instruction::EAST(n) => self.x += *n as isize,
                Instruction::WEST(n) => self.x -= *n as isize,
                Instruction::LEFT(n) => {}
                Instruction::RIGHT(n) => {}
                Instruction::FORWARD(n) => {}
            }
        }
    }
}
