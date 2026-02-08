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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_dial_position() {
        assert_eq!(Dial::new().read_position(), 50);
    }
}
