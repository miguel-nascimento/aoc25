#![feature(iter_map_windows)]
const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

fn part1(input: &str) -> u32 {
    let mut total_joltage = 0;
    for line in input.lines() {
        let mut digit1 = 0;
        let mut digit2 = 0;
        let joltage = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map_windows(|[first, second]| {
                if *first > digit1 {
                    digit1 = *first;
                    digit2 = *second;
                } else if *second > digit2 {
                    digit2 = *second;
                }
                (digit1 * 10) + digit2
            })
            .max()
            .unwrap_or_default();
        total_joltage += joltage;
    }

    total_joltage
}

#[test]
fn example_pass() {
    assert_eq!(part1(EXAMPLE), 357);
}

fn main() {
    let input = include_str!("../../input/03.txt");
    println!("{}", part1(input));
}
