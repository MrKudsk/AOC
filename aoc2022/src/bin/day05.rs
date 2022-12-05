//use std::{collections::HashMap};

use std::collections::VecDeque;

#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/05.txt");

fn main() {
    println!("day 05, output 1: {}", parse1(INPUT));
    println!("day 05, output 2: {}", parse2(INPUT));
}

/*
fn test() {
    for (i, p) in pos.iter().enumerate() {
        //dbg!(i, p);
        let stack_line = puzzel_iter.clone()
            //.lines()
            .filter(|l| l.chars().nth(*p).unwrap() != ' ')
            .map(|l| l.chars().nth(*p).unwrap())
            .collect::<Vec<char>>();
        stack.insert(i, stack_line);
    };
}
*/ 

#[derive(Debug, Clone)]
struct Move {
    num: u8,
    from: u8,
    to: u8,
}

fn parse1(s: &str) -> String {
    let (puzzel, moves_p) = s.split_once("\n\n").unwrap();
    //dbg!(puzzel);
    //let mut stack = HashMap::new();
    let mut puzzel_iter = puzzel.lines()
        .rev();
    let pos_line = puzzel_iter.next().unwrap();
    let pos = pos_line.chars()
        .filter(|c| c != &' ')
        .map(|c| {
            pos_line.find(c).unwrap()
        })
        .collect::<Vec<usize>>();
    let mut stack = pos.iter()
        .map(|p| {
            puzzel_iter.clone()
                .filter(|l| l.chars().nth(*p).unwrap() != ' ')
                .map(|l| l.chars().nth(*p).unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let moves = moves_p.lines()
        .map(|l|{
            let tmp = l.split(" ")
            .collect::<Vec<&str>>();
            //dbg!(&tmp);
            let num = tmp[1].parse::<u8>().unwrap();
            let from = tmp[3].parse::<u8>().unwrap() - 1;
            let to = tmp[5].parse::<u8>().unwrap() - 1;
            //dbg!(num);
            Move{num, from, to}
        })
        .collect::<Vec<Move>>();
    //dbg!(&moves);

    for m in moves {
        (1..=m.num).for_each(|_| {
            let tmp = &stack[m.from as usize].pop().unwrap();
            stack[m.to as usize].push(*tmp);
        })
    }
    //dbg!(&stack);
    stack.iter()
        .map(|v| {
            v.last().unwrap()
        })
        .collect::<String>()
}

fn parse2(s: &str) -> String {
    let (puzzel, moves_p) = s.split_once("\n\n").unwrap();
    //dbg!(puzzel);
    //let mut stack = HashMap::new();
    let mut puzzel_iter = puzzel.lines()
        .rev();
    let pos_line = puzzel_iter.next().unwrap();
    let pos = pos_line.chars()
        .filter(|c| c != &' ')
        .map(|c| {
            pos_line.find(c).unwrap()
        })
        .collect::<Vec<usize>>();
    let mut stack = pos.iter()
        .map(|p| {
            puzzel_iter.clone()
                .filter(|l| l.chars().nth(*p).unwrap() != ' ')
                .map(|l| l.chars().nth(*p).unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let moves = moves_p.lines()
        .map(|l|{
            let tmp = l.split(" ")
            .collect::<Vec<&str>>();
            //dbg!(&tmp);
            let num = tmp[1].parse::<u8>().unwrap();
            let from = tmp[3].parse::<u8>().unwrap() - 1;
            let to = tmp[5].parse::<u8>().unwrap() - 1;
            //dbg!(num);
            Move{num, from, to}
        })
        .collect::<Vec<Move>>();
    //dbg!(&moves);

    for m in moves {
        let mut tmp: Vec<char> = Vec::new();
        (1..=m.num).for_each(|_| {
            let t = stack[m.from as usize].pop().unwrap();
            tmp.push(t);
        });
        //dbg!(&tmp);
        for t in tmp.into_iter().rev() { 
            //dbg!(t);
            stack[m.to as usize].push(t);
        }
    }
    //dbg!(&stack);
    stack.iter()
        .map(|v| {
            v.last().unwrap()
        })
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), "CMZ");
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), "MCD");
    }
}
