fn split(input: &str) -> (&str, &str) {
    input.trim().split_once("\n\n").expect("Malformed input")
}

#[cfg(test)]
mod tests {
    use crate::split;

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


}
