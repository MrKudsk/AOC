use std::collections::HashMap;
use itertools::Itertools;

const INPUT: &'static str = include_str!("../input/10.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 10, output 1: {}", parser1(INPUT));
    println!("day 10, output 2: {}", parser2(INPUT));
}

pub fn parser1(s: &str) -> usize{
    let opens = vec!['(', '[', '{', '<'];
    let mut closes = HashMap::new();
    closes.insert(')', '(');
    closes.insert(']', '[');
    closes.insert('}', '{');
    closes.insert('>', '<');

    let puzzle = s.lines()
        .fold(0, |acc, line| {
            let mut stack = Vec::new();

            for c in line.chars() {
                if opens.contains(&c) {
                    stack.push(c);
                } else {
                    if stack.last().unwrap() == closes.get(&c).unwrap() {
                        stack.pop();
                    } else {
                        if c == ')' { return acc + 3; }
                        else if c == ']' { return acc + 57; }
                        else if c == '}' { return acc + 1197; }
                        else if c == '>' { return acc + 25137; }
                        else { panic!("Aaaaahhhh!");}
                    }
                }
            }
            acc
        });

    puzzle
}

pub fn parser2(s: &str) -> usize {
    let opens = vec!['(', '[', '{', '<'];
    let mut closes = HashMap::new();
    closes.insert(')', '(');
    closes.insert(']', '[');
    closes.insert('}', '{');
    closes.insert('>', '<');

    let puzzle = s.lines()
        .filter_map(|line| {
            let mut stack = Vec::new();

            for c in line.chars() {
                if opens.contains(&c) {
                    stack.push(c);
                } else {
                    if stack.last().unwrap() == closes.get(&c).unwrap() {
                        stack.pop();
                    } else {
                        return None;
                    }
                }
            }
            let points = Some(stack.iter()
                .rev()
                .fold(0, |pt, &c| {
                    if c == '(' { return (pt * 5) + 1}
                    else if c == '[' { return (pt * 5) + 2; }
                    else if c == '{' { return (pt * 5) + 3; }
                    else if c == '<' { return (pt * 5) + 4; }
                    else { panic!("Aaaaahhhh!");}
                }));
            //dbg!(line, points);
            points
        })
        .sorted()
        .collect_vec();

    puzzle[puzzle.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 26397)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 288957)
    }
}

