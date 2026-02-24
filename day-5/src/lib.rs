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

#[cfg(test)]
mod tests {
    use crate::{split, parse_ranges, parse_ids};

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
}
