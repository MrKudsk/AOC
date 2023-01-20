use std::collections::{HashSet, VecDeque};

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
    let (_, puzzle) = parse(s).unwrap();

    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for vec in puzzle.iter() {
        cubes.insert((vec[0], vec[1], vec[2]));
    }
    
    let mut xrange = (i32::MAX, i32::MIN);
    let mut yrange = (i32::MAX, i32::MIN);
    let mut zrange = (i32::MAX, i32::MIN);
    
    for c in &cubes {
        xrange.0 = xrange.0.min(c.0);
        xrange.1 = xrange.1.max(c.0);
        yrange.0 = yrange.0.min(c.1);
        yrange.1 = yrange.1.max(c.1);
        zrange.0 = zrange.0.min(c.2);
        zrange.1 = zrange.1.max(c.2);
    }

    xrange = (xrange.0 - 1, xrange.1 + 1);
    yrange = (yrange.0 - 1, yrange.1 + 1);
    zrange = (zrange.0 - 1, zrange.1 + 1);

    println!("{:?}", xrange);
    println!("{:?}", yrange);
    println!("{:?}", zrange);

    let mut sides = 0;
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((xrange.0, yrange.0, zrange.0));

    while let Some(pos) = to_visit.pop_front() {
        if !seen.insert(pos) {
            continue;
        }

        for d in DELTA_XYZ {
            let next_pos = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);

            if next_pos.0 < xrange.0
                || next_pos.0 > xrange.1
                || next_pos.1 < yrange.0
                || next_pos.1 > yrange.1
                || next_pos.2 < zrange.0
                || next_pos.2 > zrange.1
            {
                continue;
            } 

            if cubes.contains(&next_pos) {
                sides += 1;
            } else {
                to_visit.push_back(next_pos);
            }
        }
    }
    sides
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
        assert_eq!(parse2(INPUT), 58);
    }
}
