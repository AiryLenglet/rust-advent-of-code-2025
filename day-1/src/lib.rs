use crate::Rotation::{Left, Right};

struct Dial {
    current_position: i16,
    pointed_at_0: u16,
}

impl Dial {
    fn new() -> Self {
        Self { current_position: 50, pointed_at_0: 0 }
    }

    fn read_position(&self) -> u16 {
        self.current_position as u16
    }

    fn rotate(&mut self, rotation: Rotation) {
        const DIAL_POSITION_COUNT: i16 = 100;
        let ini = self.current_position;
        self.current_position +=  match rotation {
            Rotation::Right(position) => position as i16,
            Rotation::Left(position) => -1 * (position as i16),
        };
        let passed_0 = if self.current_position <= 0 {
            ((-1 * self.current_position) / DIAL_POSITION_COUNT) + if ini == 0 {0 } else {1 }
        } else {
            self.current_position / DIAL_POSITION_COUNT
        };
        self.pointed_at_0 += passed_0 as u16;
        if self.current_position < 0  {
            self.current_position = DIAL_POSITION_COUNT + self.current_position % DIAL_POSITION_COUNT;
        }
        self.current_position %= DIAL_POSITION_COUNT;
    }

    fn time_pointed_at_0(&self) -> u16 {
        self.pointed_at_0
    }
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Right(u16),
    Left(u16),
}

impl Rotation {
    fn parse(raw_rotation: &str) -> Self {
        let (direction, position_str) = raw_rotation.trim().split_at(1);
        let position = position_str.parse().unwrap();
        match direction {
            "L" => Left(position),
            "R" => Right(position),
            _ => panic!("Unknown rotation: {}", raw_rotation),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
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
    fn test_rotate_dial_left_to_0() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Left(150));
        assert_eq!(dial.read_position(), 0);
    }

    #[test]
    fn test_rotate_dial_left_past_0() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Left(51));
        assert_eq!(dial.read_position(), 99);
    }

    #[test]
    fn test_rotation_enum_parsing() {
        assert_eq!(Rotation::parse("L32"), Rotation::Left(32));
    }

    #[test]
    fn test_rotate_dial_multiple_times() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Right(300));
        assert_eq!(dial.read_position(), 50);
    }

    #[test]
    fn test_rotate_dial_left_multiple_times() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Left(300));
        assert_eq!(dial.read_position(), 50);
    }

    #[test]
    fn test_mini_game() {
        let mut dial = Dial::new();
        assert_eq!(dial.time_pointed_at_0(), 0);

        dial.rotate(Rotation::parse("L68"));
        assert_eq!(dial.read_position(), 82);
        assert_eq!(dial.time_pointed_at_0(), 1);

        dial.rotate(Rotation::parse("L30"));
        assert_eq!(dial.read_position(), 52);

        dial.rotate(Rotation::parse("R48"));
        assert_eq!(dial.read_position(), 0);
        assert_eq!(dial.time_pointed_at_0(), 2);

        dial.rotate(Rotation::parse("L5"));
        assert_eq!(dial.read_position(), 95);
        assert_eq!(dial.time_pointed_at_0(), 2);

        dial.rotate(Rotation::parse("R60"));
        assert_eq!(dial.read_position(), 55);
        assert_eq!(dial.time_pointed_at_0(), 3);

        dial.rotate(Rotation::parse("L55"));
        assert_eq!(dial.read_position(), 0);
        assert_eq!(dial.time_pointed_at_0(), 4);

        dial.rotate(Rotation::parse("L1"));
        assert_eq!(dial.read_position(), 99);

        dial.rotate(Rotation::parse("L99"));
        assert_eq!(dial.read_position(), 0);
        assert_eq!(dial.time_pointed_at_0(), 5);

        dial.rotate(Rotation::parse("R14"));
        assert_eq!(dial.read_position(), 14);

        dial.rotate(Rotation::parse("L82"));
        assert_eq!(dial.read_position(), 32);
        assert_eq!(dial.time_pointed_at_0(), 6);
    }

    #[test]
    fn test_time_pointed_at_0() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Right(50));
        assert_eq!(dial.time_pointed_at_0(), 1);
    }

    #[test]
    fn test_time_pointed_at_0_when_left_rotation() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Left(50));
        assert_eq!(dial.time_pointed_at_0(), 1);
    }

    #[test]
    fn test_time_pointed_at_0_when_multiple_rotation() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Right(183));
        assert_eq!(dial.time_pointed_at_0(), 2);
    }

    #[test]
    fn test_time_pointed_at_0_when_multiple_left_rotation() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Left(183));
        assert_eq!(dial.time_pointed_at_0(), 2);
    }

    #[test]
    fn test_time_pointed_at_0_when_multiple_multiple_left_right_rotation() {
        let mut dial = Dial::new();
        dial.rotate(Rotation::Right(1));
        assert_eq!(dial.time_pointed_at_0(), 0);
        dial.rotate(Rotation::Left(1));
        assert_eq!(dial.time_pointed_at_0(), 0);
    }

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let mut dial = Dial::new();
        let mut zero_count = 0;
        input.lines()
            .map(|line| Rotation::parse(line))
            .for_each(|rotation| {
                dial.rotate(rotation);
                if dial.read_position() == 0 {
                    zero_count += 1;
                }
            });
        println!("pass word is {}", zero_count);
        assert_eq!(zero_count, 1021);
        let zero_passed = dial.time_pointed_at_0();
        println!("0x434C49434B password is {}", zero_passed);
        assert_eq!(zero_passed, 5933);
    }
}
