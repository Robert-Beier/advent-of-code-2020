pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    fn next(&mut self) -> &Self {
        self = &mut match self {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        };
        self
    }
}

pub struct Modification {
    rotation: Rotation,
    flip: bool,
}

impl Modification {
    fn next(&mut self) -> Option<&Self> {
        // if self.flip {
        //     self.rotation = Rotation::
        // }
        None
    }
}
