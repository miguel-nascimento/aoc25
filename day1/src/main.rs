use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_left = &s[0..1] == "L";
        if is_left {
            Ok(Dir::Left)
        } else {
            Ok(Dir::Right)
        }
    }
}

fn part1() -> i32 {
    let input = File::open("./input/01.txt").unwrap();
    let reader = BufReader::new(input);

    let mut dial = 50;
    let mut password = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let dir = Dir::from_str(&line).unwrap();
        let val: i32 = line[1..].parse().unwrap();
        match dir {
            Dir::Left => dial = (dial - val).rem_euclid(100),
            Dir::Right => dial = (dial + val).rem_euclid(100),
        };
        if dial == 0 {
            password += 1
        }
    }
    password
}

fn part2() -> i32 {
    let input = File::open("./input/01.txt").unwrap();
    let reader = BufReader::new(input);

    let mut dial = 50;
    let mut password = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let val: i32 = line[1..].parse().unwrap();
        let dir = Dir::from_str(&line).unwrap();
        let zeros = match dir {
            Dir::Left => ((100 as i32 - dial).rem_euclid(100) + val) / 100,
            Dir::Right => (dial + val) / 100,
        };
        password += zeros;
        match dir {
            Dir::Left => dial = (dial - val).rem_euclid(100),
            Dir::Right => dial = (dial + val).rem_euclid(100),
        };
    }
    password
}

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
