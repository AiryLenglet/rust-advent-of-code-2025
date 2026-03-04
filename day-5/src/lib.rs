use std::ops::RangeInclusive;

fn split(input: &str) -> (&str, &str) {
    input.trim().split_once("\n\n").expect("Malformed input")
}

fn parse_ranges(ranges: &str) -> impl Iterator<Item=RangeInclusive<u64>> {
    ranges.trim().lines()
        .map(|line| line.trim().split_once("-").expect("Malformed range"))
        .map(|(x, y)| (x.parse::<u64>().expect("Unable to parse start"), y.parse::<u64>().expect("Unable to parse end")))
        .map(|(start, end)| start..=end)
}

fn parse_ids(ids: &str) -> impl Iterator<Item=u64> {
    ids.trim().lines()
        .map(|line| line.trim().parse::<u64>().expect("Malformed id"))
}

trait A {
    fn contains(&mut self, id: u64) -> bool;
}
impl<I> A for I
where
    I: Iterator<Item = RangeInclusive<u64>>,
{
    fn contains(&mut self, id: u64) -> bool {
        // `any` consumes the iterator lazily and stops at the first `true`.
        self.any(|range| range.contains(&id))
    }
}

fn merge(left: &RangeInclusive<u64>, right: &RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
    if left.contains(&right.start()) {
        return Some(*left.start()..=*right.end());
    } else if right.contains(&left.start()) {
        return Some(*right.start()..=*left.end())
    }
    None
}

#[cfg(test)]
mod tests {
    use std::{cmp, fs};
    use std::ops::RangeInclusive;
    use crate::{split, parse_ranges, parse_ids, A};
    use super::merge;

    const MINI_GAME_INPUT: &str = r#"
    3-5
10-14
16-20
12-18

1
5
8
11
17
32
    "#;

    #[test]
    fn test_split() {
        assert_eq!(
            split(MINI_GAME_INPUT),
            (
                r#"
3-5
10-14
16-20
12-18
                "#
                .trim(),
                r#"
1
5
8
11
17
32
                "#
                .trim()
            )
        );
    }

    #[test]
    fn test_parse_ranges() {
        assert_eq!(parse_ranges("1-3").collect::<Vec<_>>(), [1..=3]);
        assert_eq!(parse_ranges(r#"
        1-2
        4-5
        "#).collect::<Vec<_>>(), [1..=2, 4..=5]);
    }

    #[test]
    fn test_parse_ids() {
        assert_eq!(parse_ids(r#"
        1
        2
        3"#).collect::<Vec<_>>(), [1,2,3]);
    }

    #[test]
    fn test_mini_game() {
        let (ranges_input, ids_input) = split(MINI_GAME_INPUT);
        let fresh_ids = parse_ids(ids_input)
            .filter(|&id| parse_ranges(ranges_input).contains(id))
            .count();
        assert_eq!(fresh_ids, 735);
    }

    #[test]
    fn test_solution() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let (ranges_input, ids_input) = split(input.as_str());
        let fresh_ids = parse_ids(ids_input)
            .filter(|&id| parse_ranges(ranges_input).contains(id))
            .count();
        assert_eq!(fresh_ids, 3);
    }

    #[test]
    fn test_merge() {
        assert_eq!(merge(&(1..=2), &(3..=4)), None);
        assert_eq!(merge(&(1..=2), &(2..=4)), Some(1..=4));
        assert_eq!(merge(&(5..=7), &(4..=6)), Some(4..=7));
    }

    #[test]
    fn test_solution_part_2() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let (ranges_input, _) = split(input.as_str());

        let f = |distinct_ranges, range| {
            let mut result = Vec::new();
            let mut merged = false;
            for r in distinct_ranges {
                if let Some(new_range) = merge(&r, &range) {
                    result.push(new_range);
                    merged = true;
                } else {
                    result.push(r);
                }
            }
            if !merged {
                result.push(range);
            }
            result
        };
        let mut folded_ranges: Vec<_> = parse_ranges(ranges_input).collect();

        loop {
            let before_fold_count = folded_ranges.len();
            folded_ranges = folded_ranges.into_iter().fold(Vec::new(), f);
            let after_fold_count = folded_ranges.len();
            if before_fold_count == after_fold_count {
                break;
            }
        }

        let distinct_ids = folded_ranges
            .into_iter()
            .map(|range| range.count())
            .reduce(|a, b| a + b)
            .expect("Expected multiple ranges");
        assert_eq!(distinct_ids, 14);
    }
    
    #[test]
    fn test_solution_part_2_() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let (ranges_input, _) = split(input.as_str());

        let mut ranges : Vec<_> = parse_ranges(ranges_input).collect();
        ranges.sort_unstable_by(|left, right| left.start().cmp(&right.start()).then_with(|| left.end().cmp(&right.end())));

        let mut merged : Vec<RangeInclusive<u64>> = Vec::new();
        for range in ranges {
            if let Some(last_range) = merged.last_mut() {
                if range.start() <= last_range.end() {
                    *last_range = *last_range.start()..=cmp::max(*range.end(), *last_range.end());
                    continue;
                }
            }
            merged.push(range);
        }
        let distinct_ids = merged
            .into_iter()
            .map(|range| range.count() as u64)
            .sum::<u64>();
        assert_eq!(distinct_ids, 344306344403172);
    }

}
