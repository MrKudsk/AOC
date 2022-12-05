#[allow(unused_variables)]
use itertools::{min, max};

const INPUT: &'static str = include_str!("../../input/04.txt");

fn main() {
    println!("day 04, output 1: {}", parse1(INPUT));
    println!("day 04, output 2: {}", parse2(INPUT));
}

fn make_vec(con: &str) -> Vec<u32> {
    let (con_x, con_y) = con.split_once("-")
        .unwrap();
    let con_x: u32 = con_x.clone().parse::<u32>().unwrap();
    let con_y: u32 = con_y.clone().parse::<u32>().unwrap();
    //dbg!(con_x, con_y);

    (con_x..=con_y)
        .into_iter()
        .map(|num| {
            //dbg!(num);
            num
        })
        .collect::<Vec<u32>>()
}

fn make_string(con: &str) -> String {
    let (con_x, con_y) = con.split_once("-")
        .unwrap();
    let con_x: char = con_x.clone().chars().nth(0).unwrap();
    let con_y: char = con_y.clone().chars().nth(0).unwrap();
    //dbg!(con_x, con_y);

    (con_x..=con_y)
        .into_iter()
        .map(|num| {
            //dbg!(num);
            num
        })
        .collect::<String>()
}

fn parse1(s: &str) -> usize {
    let res = s.lines()
        .map(|l| {
            let (con_a, con_b) = l.split_once(",").unwrap();
            let a: Vec<u32> = make_vec(con_a);
            let b: Vec<u32> = make_vec(con_b);
            //dbg!(min(&a).unwrap(), max(&a).unwrap());
//            if b.contains(a.0) || a.contains(b.0) {
            let a_min = min(&a).unwrap();
            let a_max = max(&a).unwrap();
            let b_min = min(&b).unwrap();
            let b_max = max(&b).unwrap();
            if a_min <= b_min && a_max >= b_max {
                return 1;
            } else if b_min <= a_min && b_max >= a_max {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<u32>();
    dbg!(res);
    res as usize
}

fn parse2(_s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
20-26,4-8";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 2);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1);
    }
}
