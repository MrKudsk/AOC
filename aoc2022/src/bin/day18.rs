use std::collections::HashSet;

use glam::Vec3;
use nom::{bytes::complete::tag,
    multi::separated_list1,
    character::complete,
    character::complete::line_ending,
    *,
};

const INPUT: &'static str = include_str!("../../input/18.txt");

const DELTA_XYZ: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    ( 1, 0, 0),
    (0, -1, 0),
    (0,  1, 0),
    (0, 0, -1),
    (0, 0,  1),
];

fn main() {
    println!("day 18, output 1: {}", parse1(INPUT));
    println!("day 18, output 2: {}", parse2(INPUT));
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::i32)
            .map(|vec| vec),
        )(input)
}

fn parse1(s: &str) -> usize {
    let (_, puzzle) = parse(s).unwrap();

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for vec in puzzle.iter() {
        cubes.insert((vec[0], vec[1], vec[2]));
    }
    
    let mut sides = cubes.len() * 6;

    for c in &cubes {
        for d in DELTA_XYZ {
            let pos = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
            println!("{c:?} - {d:?} -> {pos:?}");
            if cubes.contains(&pos) {
                sides -= 1;
            }
        }
    }
    sides
}

fn parse2(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 64);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1);
    }
}
