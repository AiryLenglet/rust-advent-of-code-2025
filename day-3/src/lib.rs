
fn find_first_highest(str: &str) -> (u64, u64) {
    let mut highest = 0;
    let mut index = 0;
    for (i, c) in str.chars().enumerate() {
        let char = c.to_digit(10).expect("Found invalid char");
        if char > highest {
            highest = char;
            index = i;
        }
    }
    (highest as u64, index as u64)
}

const battery_to_select: usize = 12;

fn find_best_jotage(str: &str) -> u64 {
    let mut jotage = String::new();
    let mut start = 0;
    while jotage.len() < battery_to_select {
        let (highest, position) = find_first_highest(&str[start..=str.len() - battery_to_select + jotage.len()]);
        start += position as usize + 1;
        jotage.push_str(highest.to_string().as_str());
    }

    jotage.parse::<u64>().expect("Couldn't parse jotage")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{find_best_jotage, find_first_highest,};

    #[test]
    fn test_find_first_highest() {
        assert_eq!(find_first_highest("12345"), (5, 4));
        assert_eq!(find_first_highest("98765943219"), (9, 0));
    }

    #[test]
    fn test_find_best_jotage_() {
        assert_eq!(find_best_jotage("987654321111111"), 987654321111);
        assert_eq!(find_best_jotage("811111111111119"), 811111111119);
        assert_eq!(find_best_jotage("234234234234278"), 434234234278);
        assert_eq!(find_best_jotage("818181911112111"), 888911112111);
    }

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let result = input.lines()
            .map(|line| find_best_jotage(line))
            .reduce(|a, b| a + b);
        assert_eq!(result, Some(168627047606506));
    }
}
