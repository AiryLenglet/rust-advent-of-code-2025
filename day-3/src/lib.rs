#[derive(Debug, PartialEq)]
struct BatteryBank {
    labels: String,
}

impl BatteryBank {
    fn parse(labels: &str) -> Self {
        Self {
            labels: labels.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BatteryBank;

    #[test]
    fn test_parse_battery_bank() {
        let bank = BatteryBank::parse("123456789");
        assert_eq!(
            bank,
            BatteryBank {
                labels: String::from("123456789")
            }
        );
    }
}
