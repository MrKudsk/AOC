#[warn(unused_mut)]

use std::{collections::HashSet, str::Lines};
use itertools::Itertools;

const INPUT: &'static str = include_str!("../input/4.txt");

#[derive(Debug, Clone)]
struct Board {
    /// List of sets af winning comibination
    sets: Vec<HashSet<i32>>,
}

impl Board {
    fn new(lines: &mut Lines) -> Option<Board> {
        let mut sets: Vec<HashSet<i32>> = Vec::new();

        let empty = lines.next()?;
        if empty != "" {
            panic!("Dude, you have messed up somethings");
        }

        let rows: Vec<Vec<i32>> = lines
            .take(5)
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect_vec();

        // Column-wise
        for col in 0..5 {
            let mut set = HashSet::new();
            for row in 0..5 {
                set.insert(rows[row][col]);
            }

            sets.push(set);
        }

        // Row-wise
        for row in rows {
            sets.push(HashSet::from_iter(row));
        }

        Some(Board {sets})
    }

    fn turn(&mut self, m: i32) -> bool {
        let mut complete = false;
        for set in self.sets.iter_mut() {
            if set.remove(&m) {
                complete |= set.is_empty();
            }
        }

        complete
    }

    fn remaining_sum(&self) -> i32 {
        HashSet::<&i32>::from_iter(self.sets.iter().flatten())
            .into_iter()
            .sum()

    }
}

pub fn run() {
    println!("-----------------------------------");
    println!("day 4, output 1: {}", parser1(INPUT));
    println!("day 4, output 2: {}", parser2(INPUT));
}

pub fn parser1(s: &str) -> i32 {
    let mut res = 0; 
    let mut lines = s.lines();
    let moves: Vec<i32> = lines.next().unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    while let Some(board) = Board::new(&mut lines) {
        boards.push(board)
    }

    {
        let mut boards = boards.clone();
        'moves: for m in &moves {
            for board in boards.iter_mut() {
                if board.turn(*m) {
                    res = board.remaining_sum();
                    //println!("{} * {}", m, res);
                    res = res * m;
                    break 'moves;
                }
            }
        }
    }
    
    //dbg!(boards);

    res
}

pub fn parser2(s: &str) -> i32 {
    let mut last_res = 0; 
    let mut lines = s.lines();
    let moves: Vec<i32> = lines.next().unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    while let Some(board) = Board::new(&mut lines) {
        boards.push(board)
    }
    
    for m in &moves {
        // the idx's of boards which win from last moves
        let mut idxs_to_remove = Vec::new();
        for (idx, board) in boards.iter_mut().enumerate() {
            if board.turn(*m) {
                last_res = m * board.remaining_sum();
                idxs_to_remove.push(idx);
            }
        }

        // Remove board that have won
        for idx in idxs_to_remove.iter().rev() {
            boards.remove(*idx);
        }
    }
    last_res
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 4512)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 1924)
    }
}
