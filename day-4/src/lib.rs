use std::collections::HashMap;
use std::fmt::Display;

fn adjacent_keys(key: &String) -> Vec<String> {
    let (i, j) = key.split_once(KEY_SEPARATOR)
        .expect("Invalid key");
    let int_i = i.parse::<usize>().expect("Malformed index") as i64;
    let int_j = j.parse::<usize>().expect("Malformed index") as i64;
    (int_i-1..=int_i+1)
        .flat_map(|x| (int_j-1..=int_j+1)
            .map(move |y| compute_key(x, y)))
        .filter(|k| !key.eq(k))
        .collect()
}

const ROLL_THRESHOLD: usize = 4;
const KEY_SEPARATOR: &str = "#";

fn compute_key<T: Display, U: Display>(x: T, y: U) -> String {
    format!("{}{}{}", x, KEY_SEPARATOR, y)
}

fn find_movable_rolls(setup: &str) -> u64 {
    let mut map : HashMap<String, char> = HashMap::new();

    setup.lines().enumerate()
        .for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                let key = compute_key(i, j);
                map.insert(key, c);
            })
        });

    let mut movable_rolls = 0;

    loop {
        let rolls = setup.lines().enumerate()
            .map(|(i, line)| {
                line.chars().enumerate().map(|(j, c)| {
                    if c == '@' {
                        let cell_key = compute_key(i, j);
                        let adjacent_roll = adjacent_keys(&cell_key)
                            .iter().map(|key| map.get(key))
                            .flatten()
                            .filter(|c| **c == '@')
                            .count();
                        if adjacent_roll < ROLL_THRESHOLD {
                            map.remove(&cell_key);
                            return 1;
                        }
                        return 0;
                    }
                    0
                }).reduce(|a, b| a + b)
            }).flatten().reduce(|a, b| a + b)
            .expect("Malformed index");
        if rolls == movable_rolls {
            break;
        }
        movable_rolls = rolls;
    }
    movable_rolls
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{adjacent_keys, find_movable_rolls};

    #[test]
    fn test_adjacent_keys() {
        assert_eq!(adjacent_keys(&String::from("2#2")), ["1#1", "1#2", "1#3", "2#1", "2#3", "3#1", "3#2", "3#3"]);
    }

    #[test]
    fn test_accessible_rolls() {
        let setup = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(find_movable_rolls(setup), 43);
    }

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let result = find_movable_rolls(&input);
        assert_eq!(8451, result);
    }
}
