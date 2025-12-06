use std::fs::{self};

fn part1(input: &str) -> u64 {
    let mut invalid_ids: Vec<u64> = Vec::new();
    let ranges: Vec<&str> = input.split(",").map(|x| x.trim()).collect();
    for range in ranges {
        let splits: Vec<&str> = range.split('-').collect();
        let first_id: u64 = splits[0].parse().expect("a int");
        let last_id: u64 = splits[1].parse().expect("second int");
        for id in first_id..=last_id {
            if check_invalid_id_part1(id) {
                invalid_ids.push(id)
            }
        }
    }
    invalid_ids.iter().sum()
}

fn part2(input: &str) -> u64 {
    let mut invalid_ids: Vec<u64> = Vec::new();
    let ranges: Vec<&str> = input.split(",").map(|x| x.trim()).collect();
    for range in ranges {
        let splits: Vec<&str> = range.split('-').collect();
        let first_id: u64 = splits[0].parse().expect("a int");
        let last_id: u64 = splits[1].parse().expect("second int");
        for id in first_id..=last_id {
            if check_invalid_id_part2(id) {
                invalid_ids.push(id)
            }
        }
    }
    invalid_ids.iter().sum()
}

fn main() {
    let input = fs::read_to_string("./input/02.txt").unwrap();
    println!("part1:{}", part1(&input));
    println!("part2:{}", part2(&input));
}

// You can find the invalid IDs by looking for any ID
// which is made only of some sequence of digits repeated twice.
// So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
fn check_invalid_id_part1(id: u64) -> bool {
    let str = id.to_string();
    let n = str.len();
    if n == 0 || n % 2 != 0 {
        return false;
    }
    let (first, second) = str.split_at(n / 2);
    first == second
}

// Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice.
// So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times),
// and 1111111 (1 seven times) are all invalid IDs.
fn check_invalid_id_part2(id: u64) -> bool {
    let str = id.to_string();
    let n = str.len();
    if n < 2 {
        return false;
    }
    let half = n / 2;
    for p in 1..=half {
        // If it not divisible, the pattern can't occur - we should skip this one
        if n % p != 0 {
            continue;
        }
        let pattern = &str[..p];
        if pattern.repeat(n / p) == str {
            return true;
        }
    }
    false
}

#[test]
fn example_pass() {
    const EXAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
    assert_eq!(part1(EXAMPLE), 1227775554);
    assert_eq!(part2(EXAMPLE), 4174379265);
}
