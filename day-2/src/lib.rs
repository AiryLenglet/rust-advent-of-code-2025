#[derive(Debug, PartialEq)]
struct IdRange {
    start: u64,
    end: u64,
}

impl Iterator for IdRange {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

impl IdRange {
    fn parse(id_range: &str) -> Self {
        let (start_str, end_str) = id_range.split_once('-')
            .expect("Malformed id {id_range}");
        let start = start_str.parse::<u64>().unwrap();
        let end = end_str.parse::<u64>().unwrap();
        Self {
            start,
            end,
        }
    }
}

fn invalid(id: u64) -> Option<u64> {
    let id_str = id.to_string();
    for chunk_size in 1..id_str.len() {
        let chunks = id_str.as_bytes()
            .chunks(chunk_size)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .expect("Unable to chunk id");

        let first = chunks[0];
        let mut all_equals = true;
        for chunk in chunks {
            if chunk != first {
                all_equals = false;
                break;
            }
        }

        if all_equals {
            return Some(id);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_parse() {
        let range = IdRange::parse("1-3");
        assert_eq!(range, IdRange{ start: 1, end: 3 });
    }

    #[test]
    fn test_iter() {
        let range = IdRange::parse("1-3");
        assert_eq!(range.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(invalid(1), None);
        assert_eq!(invalid(11), Some(11));
        assert_eq!(invalid(99), Some(99));
        assert_eq!(invalid(3446456), None);
        assert_eq!(invalid(121212), Some(121212));
    }

    #[test]
    fn test_mini_game() {
        let raw_ranges = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862"
        ];
        
        let result = raw_ranges.iter()
            .map(|str_range| IdRange::parse(str_range))
            .flat_map(|id_range| id_range)
            .map(|id| invalid(id))
            .map(|id| id.unwrap_or_else(|| 0))
            .reduce(|a, b| a + b);
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let result = input.split(',')
            .map(|str_range| IdRange::parse(str_range))
            .flat_map(|id_range| id_range)
            .map(|id| invalid(id))
            .map(|id| id.unwrap_or_else(|| 0))
            .reduce(|a, b| a + b);
        assert_eq!(result, Some(28858486244));
    }
}