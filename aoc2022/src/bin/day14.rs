use itertools::{Itertools};
#[allow(unused_variables)]
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list1},
    sequence::{separated_pair},
    *,
};

const INPUT: &'static str = include_str!("../../input/14.txt");

fn main() {
    println!("day 14, output 1: {}", parse1(INPUT));
    println!("day 14, output 2: {}", parse2(INPUT));
}

fn get_point(input: &str) -> IResult<&str, (usize,usize)> {
    let (input, (x, y)) = separated_pair(nom::character::complete::u32,
                                        tag(","),
                                        nom::character::complete::u32)
                        (input)?;
    Ok((input, (x as usize, y as usize)))
} 

fn get_rocks(input: &str) -> IResult<&str, Vec<Vec<(usize,usize)>>> {
    separated_list1(newline, 
        separated_list1(tag(" -> "), get_point)
    )(input)
}

fn print_map(draw: &Vec<Vec<char>>) {
    for (i, line) in draw.iter().enumerate() {
        println!("{:3} - {}", i, line.into_iter().join(""));
    }
    println!("");
}

fn fill_rock(draw: &mut Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) {
    if start.0 == end.0 {
        let mut a = start.1;
        let mut b = end.1;
        if start.1 > end.1 { 
            b = start.1;
            a = end.1;
        }
        for y in a..=b {
            draw[y][start.0] = '#';
        }
    } else {
        let mut a = start.0;
        let mut b = end.0;
        if start.0 > end.0 { 
            b = start.0;
            a = end.0;
        }
        for x in a..=b {
            draw[start.1][x] = '#';
        }
    }
}

fn simulate_sand(draw: &mut Vec<Vec<char>>, start: (usize,usize)) -> usize {
    let ymax = draw.len();
    let mut sand = start;
    let mut i = 0;
    dbg!(&ymax, &sand);
    while true {
        while sand.1 < ymax - 1 {
            if draw[sand.1 + 1][sand.0] == '.' {
                sand.1 += 1;
                continue;
            } else if draw[sand.1 + 1][sand.0 - 1] == '.' {
                sand.1 += 1;
                sand.0 -= 1;
                continue;
            } else if draw[sand.1 + 1][sand.0 + 1] == '.' {
                sand.1 += 1;
                sand.0 += 1;
                continue;
            } else {
                //dbg!(&sand);
                draw[sand.1][sand.0] = 'o';
                i += 1;
                break;
            }
        }
        if sand.1 == ymax - 1 || sand.1 == 0 {
            dbg!(&sand);
            break;
        }
        sand = start;
        //i += 1;
    }
    print_map(&draw);
    i
}


fn parse1(s: &str) -> usize {
    let (_, rocks) = get_rocks(s).unwrap();
    let all_rocks = rocks.clone();
    let xstart = all_rocks.iter().flatten().map(|(x,_y)| x).min().unwrap() - 1;
    let xmax = all_rocks.iter().flatten().map(|(x,_y)|x).max().unwrap() + 2;
    let ymax = all_rocks.iter().flatten().map(|(_x,y)| y).max().unwrap() + 2;
    dbg!(xstart, xmax, ymax);
    let mut draw = vec![vec!['.'; xmax-xstart]; ymax];

    rocks.iter()
        .for_each(|vec| {
            let mut points = vec.iter();
            let mut point_a = points.next().unwrap();
            for point_b in points {
                fill_rock(&mut draw, (point_a.0 - xstart, point_a.1), (point_b.0-xstart,point_b.1));
                point_a = point_b;
            }
        });
    print_map(&draw);
    simulate_sand(&mut draw, (500-xstart,0))
}

fn parse2(s: &str) -> usize {
    let (_, rocks) = get_rocks(s).unwrap();
    let all_rocks = rocks.clone();
    let ymax = all_rocks.iter().flatten().map(|(_x,y)| y).max().unwrap() + 3;
    let xstart = 500 - ymax;
    let xmax = all_rocks.iter().flatten().map(|(x,_y)|x).max().unwrap() + ymax;
    dbg!(xstart, xmax, ymax);
    let mut draw = vec![vec!['.'; xmax-xstart]; ymax];
    // bottum
    fill_rock(&mut draw, (0, ymax - 1), (xmax - xstart -1, ymax - 1));
    rocks.iter()
        .for_each(|vec| {
            let mut points = vec.iter();
            let mut point_a = points.next().unwrap();
            for point_b in points {
                fill_rock(&mut draw, (point_a.0 - xstart, point_a.1), (point_b.0-xstart,point_b.1));
                point_a = point_b;
            }
        });
    print_map(&draw);
    simulate_sand(&mut draw, (500-xstart,0)) 
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 24);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 93);
    }
}           
