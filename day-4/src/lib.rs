fn adjacent_keys(key: String) -> Vec<String> {
    let (i, j) = key.split_once("#")
        .expect("Invalid key");
    let int_i = i.parse::<usize>().expect("Malformed index") as i64;
    let int_j = j.parse::<usize>().expect("Malformed index") as i64;
    (int_i-1..=int_i+1)
        .flat_map(|x| (int_j-1..=int_j+1)
            .map(move |y| format!("{}#{}", x, y)))
        .filter(|k| !key.eq(k))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::adjacent_keys;

    #[test]
    fn test_adjacent_keys() {
        assert_eq!(adjacent_keys(String::from("2#2")), ["1#1", "1#2", "1#3", "2#1", "2#3", "3#1", "3#2", "3#3"]);
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

        let mut map : HashMap<String, char> = HashMap::new();

        setup.lines().enumerate()
            .for_each(|(i, line)| {
                line.chars().enumerate().for_each(|(j, c)| {
                    let key = format!("{}#{}", i, j);
                    map.insert(key, c);
                })
            });

        let rolls = setup.lines().enumerate()
            .map(|(i, line)| {
                line.chars().enumerate().map(|(j, c)| {
                    if c == '@' {
                        let adjacent_roll = adjacent_keys(format!("{}#{}", i, j))
                            .iter().map(|key| map.get(key))
                            .flatten()
                            .filter(|c| **c == '@')
                            .count();
                        if adjacent_roll < 4 {
                            return 1;
                        }
                        return 0;
                    }
                    0
                }).reduce(|a, b| a + b)
            }).flatten().reduce(|a, b| a + b)
            .expect("Malformed index");

        assert_eq!(rolls, 13);
    }
}
