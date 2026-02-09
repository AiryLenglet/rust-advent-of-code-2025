#[derive(Debug, PartialEq)]
struct IdRange {
    start: u16,
    end: u16,
}

impl IdRange {
    fn parse(id_range: &str) -> Self {
        let (start_str, end_str) = id_range.split_once('-')
            .expect("Malformed id {id_range}");
        let start = start_str.parse::<u16>().unwrap();
        let end = end_str.parse::<u16>().unwrap();
        Self {
            start,
            end,
        }
    }

    fn iter(&self) -> impl Iterator<Item = u16> {
        (self.start..=self.end).into_iter()
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
}