#[warn(unused_must_use)]

use itertools::Itertools;

const INPUT: &'static str = include_str!("../input/9.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 9, output 1: {}", parser1(INPUT));
    println!("day 9, output 2: {}", parser2(INPUT));
}

/*
fn printtable(p: &Vec<Vec<u32>>) {
    for (row, line) in p.iter().enumerate() {
        let mut string = String::new();
        for (col, val) in line.iter().enumerate() {
            string.push_str(&val.to_string());
            string.push_str(", ");
        } 
        println!("{}", string);
    }
}
*/

fn adjacent_locations(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut locations = Vec::new();
    // left
    if let Some(row) = row.checked_sub(1) {
        locations.push((row, col));
    } 
    // up
    if let Some(col) = col.checked_sub(1) {
        locations.push((row, col));
    }
    // right
    if let Some(row) = row.checked_add(1) {
        if row < rows { // < because 0..len-1
            locations.push((row, col));
        }
    }
    // down
    if let Some(col) = col.checked_add(1) {
        if col < cols {
            locations.push((row, col));
        }
    }
    locations
}

pub fn parser1(s: &str) -> usize {
    let puzzle = s.lines()
                    .map(|line| 
                        line.chars().into_iter()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect_vec())
                    .collect_vec();
    
    // finde the border of the matrix
    let rows = puzzle.len();
    let cols = puzzle[0].len();
    
    let mut lowest = Vec::new();
    for (row, line) in puzzle.iter().enumerate() {
        line.iter().enumerate().for_each(|(col, val)| {
            if adjacent_locations(row, col, rows, cols)
                .into_iter()
                .all(|(x,y)| puzzle[x][y] > *val) {
                lowest.push((row, col)) 
            }
        })
    }

    lowest.iter()
        .fold(0, |acc, (x,y)| acc + puzzle[*x][*y] as usize + 1)
}

pub fn parser2(s: &str) -> i32 {
    let puzzle = s.lines()
                    .map(|line| 
                        line.chars().into_iter()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect_vec())
                    .collect_vec();
    
    // finde the border of the matrix
    let rows = puzzle.len();
    let cols = puzzle[0].len();
    
    let mut lowest = Vec::new();
    for (row, line) in puzzle.iter().enumerate() {
        line.iter().enumerate().for_each(|(col, val)| {
            if adjacent_locations(row, col, rows, cols)
                .into_iter()
                .all(|(x,y)| puzzle[x][y] > *val) {
                lowest.push((row, col)) 
            }
        })
    }

    lowest.iter()
        .fold(0, |acc, (x,y)| acc + puzzle[*x][*y] as usize + 1);
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 15)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 1134)
    }
}

