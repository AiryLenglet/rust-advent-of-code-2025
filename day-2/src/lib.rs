#[derive(Debug, PartialEq)]
struct IdRange {
    start: u32,
    end: u32,
}

impl IdRange {
    fn parse(id_range: &str) -> Self {
        let (start_str, end_str) = id_range.split_once('-')
            .expect("Malformed id {id_range}");
        let start = start_str.parse::<u32>().unwrap();
        let end = end_str.parse::<u32>().unwrap();
        Self {
            start,
            end,
        }
    }

    fn iter(self) -> impl Iterator<Item = u32> {
        self.start..=self.end
    }
}

fn invalid(id: u32) -> Option<u32> {
    let id_str = id.to_string();
    if id_str.len() % 2 == 1 {
        return None;
    }
    let mid = id_str.len() / 2;
    let (upper, lower) = id_str.split_at(mid);
    if upper == lower {
        return Some(id)
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let range = IdRange::parse("1-3");
        assert_eq!(range, IdRange{ start: 1, end: 3 });
    }

    #[test]
    fn test_iter() {
        let range = IdRange::parse("1-3");
        let iter = range.iter();
        assert_eq!(iter.collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(invalid(1), None);
        assert_eq!(invalid(11), Some(11));
        assert_eq!(invalid(99), Some(99));
        assert_eq!(invalid(3446456), None);
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
            .flat_map(|id_range| id_range.iter())
            .map(|id| invalid(id))
            .map(|id| id.unwrap_or_else(|| 0))
            .reduce(|a, b| a + b);
        assert_eq!(result, Some(1227775554));
    }
}