struct Dial {
    current_position: u8,
}

impl Dial {
    fn new() -> Self {
        Self { current_position: 50 }
    }

    fn read_position(&self) -> u8 {
        self.current_position
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.current_position +=  match rotation {
            Rotation::Right(position) => position,
            Rotation::Left(position) => 100 - position,
        };
        self.current_position %= 100;
    }
}

enum Rotation {
    Right(u8),
    Left(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_dial_position() {
        assert_eq!(Dial::new().read_position(), 50);
    }

    #[test]
    fn test_rotate_dial() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Right(1));
        assert_eq!(dial.read_position(), 51);
    }

    #[test]
    fn test_rotate_dial_past_99() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Right(50));
        assert_eq!(dial.read_position(), 0);
    }

    #[test]
    fn test_rotate_dial_left() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Left(1));
        assert_eq!(dial.read_position(), 49);
    }

    #[test]
    fn test_rotate_dial_left_past_0() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Left(51));
        assert_eq!(dial.read_position(), 99);
    }
}
