#[allow(unused_variables)]
//use std::collections::HashMap;
use itertools::Itertools;

const INPUT: &'static str = include_str!("../../input/03.txt");

fn main() {
    println!("day 03, output 1: {}", parse1(INPUT));
    println!("day 03, output 2: {}", parse2(INPUT));
}

fn find_point(s: char) -> u32 {
    //let mut point = s.parse::<char>().unwrap() as u32;
    let mut point = s as u32;
    //println!("{:?}", point);
    if point >= 97 {
        point -= 96;
    } else {
        point -= 64 - 26;
    }
    //println!("{:?}", point);
    point
}

fn isCharIn(c: char, s: &str) -> bool {
    match s.find(c) {
        Some(_) => true,
        None => false,
    }
}

fn parse1(s: &str) -> usize {
    let point: u32 = s.lines().map(|l| {
        let splitpoint:usize = l.len() / 2;
        let (a, b) = l.split_at(splitpoint);
        //println!("{a} - {b}");
        let item = a.chars().filter(|ii| isCharIn(*ii, b)) 
            .collect_vec();
        find_point(item[0])
    })
    //.collect_vec()
    .sum();
    dbg!(&point);
    point as usize
}

fn parse2(_s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_point () {
        assert_eq!(find_point('a'), 1);
        assert_eq!(find_point('A'), 27);
    }

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 157);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1);
    }
}
