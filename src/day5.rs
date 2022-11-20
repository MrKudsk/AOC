use anyhow::Result;
use std::{str::FromStr, collections::HashMap, ops::RangeInclusive};


const INPUT: &'static str = include_str!("../input/5.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 5, output 1: {}", parser1(INPUT));
    println!("day 5, output 2: {}", parser2(INPUT));
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(u32, u32);

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(",").unwrap();

        Ok(Position(x.parse()?, y.parse()?))
    }
}

#[derive(Debug)]
pub struct Line {
    start: Position,
    end: Position,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_once(" -> ").unwrap();

        Ok(Line {
            start: left.parse()?,
            end: right.parse()?,
        })
    }
}

fn step(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start < end {
        Box::new(RangeInclusive::new(start, end))
    } else {
        Box::new(RangeInclusive::new(end, start).rev())
    }
}

fn calulate_pos(start: &Position, end: &Position, diagonal: bool) -> Vec<Position> {
    if start.0 != end.0 && start.1 != end.1 {
        if !diagonal {
            Vec::new()
        } else {
            step(start.0, end.0)
                .zip(step(start.1, end.1))
                .map(|(x,y)| Position(x,y))
                .collect()
        }
    } else {
        //println!("{:?}-{:?}", start, end);
        step(start.0, end.0)
            .flat_map(|x| step(start.1, end.1).map(move |y| Position(x,y)))
            .collect()
    }
}

fn get_counts(lines: &Vec<Line>, diagonal: bool) -> u32 {
    let mut counts: HashMap<Position, u32> = HashMap::new();

    lines.iter().for_each(|item|{
        for pos in calulate_pos(&item.start, &item.end, diagonal) {
            //println!("{},{}", pos.0, pos.1);
            counts.insert(pos, counts.get(&pos).unwrap_or(&0) + 1);
        } 
    });

/*    for (pos, item) in &counts {
        if *item > 1 {
            println!("{},{} count {}", pos.0, pos.1, item);
        }
    }
*/
    counts
        .iter()
        .fold(0, |acc, (_, &count)| acc + if count > 1 { 1 } else { 0 })
} 

pub fn parser1(s: &str) -> u32 {
    let lines: Vec<Line> = s.lines().map(|l| l.parse().unwrap()).collect();

    //println!(">{}<", get_counts(&lines));
    get_counts(&lines, false)
}

pub fn parser2(s: &str) -> u32 {
    let lines: Vec<Line> = s.lines().map(|l| l.parse().unwrap()).collect();

    get_counts(&lines, true)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 5)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 12)
    }
}

