#[allow(unused_variables)]
use std::collections::HashMap;
use itertools::Itertools;

const INPUT: &'static str = include_str!("../../input/02.txt");

fn main() {
    println!("day 02, output 1: {}", parse1(INPUT));
    println!("day 02, output 2: {}", parse2(INPUT));
}

fn find_winner(a: &str, b: &str) -> u32 {
    if a == "A" {               // Rock
        return match b {
            "X" => 3,       // Rock     - draw
            "Y" => 6,       // Paper    - win
            "Z" => 0,       // Scissors - lost
            _   => 0,
        };
    } else if a == "B" {               // Paper
        return match b {
            "X" => 0,       // Rock     - lost 
            "Y" => 3,       // Paper    - draw
            "Z" => 6,       // Scissors - win
            _   => 0,
        };
    } else if a == "C" {               // Scissors
        return match b {
            "X" => 6,       // Rock     - win
            "Y" => 0,       // Paper    - lost
            "Z" => 3,       // Scissors - draw
            _   => 0,
        };
    } else {
        return 0;
    }
}

fn parse1(s: &str) -> i32 {
    let mut shape_point = HashMap::new();
    shape_point.insert("X", 1);     // Rock
    shape_point.insert("Y", 2);     // Paper
    shape_point.insert("Z", 3);     // Scissors
    
    let points = s.lines()
        .map(|l| { 
            let (a,b) = l.split_once(" ").unwrap();
            return shape_point.get(b).unwrap() + find_winner(a, b) as i32;
        })
        .collect_vec()
        .into_iter()
        .sum();
    //dbg!(&points);
    points
}

fn parse2(s: &str) -> usize {
    let mut shape_point = HashMap::new();
    shape_point.insert("X", 1);     // Rock
    shape_point.insert("Y", 2);     // Paper
    shape_point.insert("Z", 3);     // Scissors
    
    let points = s.lines()
        .map(|l| { 
            let (a,b) = l.split_once(" ").unwrap();
            return find_winner(a, b) as i32;
        })
        .collect_vec()
        //.into_iter()
        //.sum();
    dbg!(&points);
    //points
}
    
#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 15);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 12);
    }
}   
