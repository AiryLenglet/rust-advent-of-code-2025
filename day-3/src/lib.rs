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

    fn find_best_jotage(&self) -> u32 {
        let labels = self.labels.chars()
            .map(|c| c.to_digit(10).ok_or("non‑digit char found {c}"))
            .collect::<Result<Vec<u32>,_>>()
            .expect("");
        let mut first_label: u32 = 0;
        let mut first_label_position = 0;
        for (i, &label) in labels[0..labels.len()-1].iter().enumerate() {
            if label >= first_label {
                first_label = label;
                first_label_position = i;
            }
        }

        let mut second_label: u32 = 0;
        for (i, &label) in labels[first_label_position + 1..labels.len()].iter().enumerate() {
            if label >= second_label {
                second_label = label;
            }
        }

        10 * first_label + second_label
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

    #[test]
    fn test_find_best_jotage() {
        assert_eq!(BatteryBank::parse("987654321111111").find_best_jotage(), 98);
        assert_eq!(BatteryBank::parse("811111111111119").find_best_jotage(), 89);
        assert_eq!(BatteryBank::parse("234234234234278").find_best_jotage(), 78);
        assert_eq!(BatteryBank::parse("818181911112111").find_best_jotage(), 92);
    }
}
