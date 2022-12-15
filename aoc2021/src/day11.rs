use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("../input/11.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 11, output 1: {}", parser1(INPUT));
    println!("day 11, output 2: {}", parser2(INPUT));
}

fn read_matrix(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect_vec()
        )
        .collect_vec()
}

fn step(state: &mut Vec<Vec<u32>>) -> usize {
    let mut flashed: HashSet<(i32,i32)> = HashSet::new();

    let rows = state.len();
    let cols = state[0].len();

    for x in 0..rows {
        for y in 0..cols {
            let val = state[x][y];

            if val >= 9 {
                flashed.insert((x as i32, y as i32));
                //println!("step {},{} -> {}", x,y, flashed.len());
            }

            state[x][y] += 1; 
        }
    }
    //dbg!(&flashed);
    for (x, y) in flashed.clone().iter() {
        //println!("-----------------------------------");
        //println!("flash {},{} -> {}", x,y, flashed.len());
        //println!("-----------------------------------");
        flash(state, &mut flashed, *x as i32, *y as i32);
    }

    for (x ,y) in flashed.iter() {
        state[*x as usize][*y as usize] = 0;
    }

    //printmatrix(&state);
    //println!("----------");
    flashed.len()
}


fn flash(state: &mut Vec<Vec<u32>>, flashed: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
    let rows = state.len() as i32;
    let cols = state[0].len() as i32;
    let mut newflashed = HashSet::new();

    for xs in [x - 1, x, x + 1] {
        for ys in [y - 1, y, y + 1] {
            //println!("({},{}) {},{}", x, y, xs, ys);
            if xs == x && ys == y {
                continue;  // Because it is the flashing item
            }

            if xs >= rows || xs < 0 || ys >= cols || ys < 0 {
                continue; // Because we are outside the matrix
            } 
            if flashed.contains(&(xs, ys)) {
                continue; // Only flash one time
            }
            //println!("> ({},{}) {},{} -> {}", x, y, xs, ys, state[xs as usize][ys as usize]);
            
            state[xs as usize][ys as usize] += 1;
            
            //println!("> ({},{}) {},{} -> {}", x, y, xs, ys, state[xs as usize][ys as usize]);
            if state[xs as usize][ys as usize] > 9 {
                flashed.insert((xs, ys));
                newflashed.insert((xs, ys));
                //println!("flash {},{} -> {}", x,y, flashed.len());
                //flash(state, flashed, x, y);
            }

        }
    }

    for (x, y) in newflashed.clone().iter() {
        //println!("-----------------------------------");
        //println!("flash {},{} -> {}", x,y, flashed.len());
        //println!("-----------------------------------");
        flash(state,flashed, *x, *y);
    }
}

/*fn printmatrix(state: &Vec<Vec<u32>>) {
    let rows = state.len();
    let cols = state[0].len();

    for x in 0..rows {
        let mut string = String::new();
        for y in 0..cols {
            string.push_str(&state[x][y].to_string());
        }
        println!("{}", string);
    }
}*/

pub fn parser1(s: &str) -> usize {
    let mut state = read_matrix(s);
    (0..100).fold(0, |acc, _| acc + step(&mut state))
}

pub fn parser2(s: &str) -> usize {
    let mut state = read_matrix(s);
    let rows = state.len();
    let cols = state[0].len();
    let all = rows * cols;
    dbg!(all);
    (0..1000).find(|_| all == step(&mut state)).unwrap() + 1
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT2: &str = "11111
19991
19191
19991
11111";

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn first() {
        let mut res = read_matrix(INPUT2);
        println!("{}", step(&mut res));
        //printmatrix(&res);
        println!("{}", step(&mut res));
        //printmatrix(&res);
        
        assert_eq!(res.len(), 5);
        //todo!();
        assert_eq!(parser1(INPUT), 1656)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 195)
    }
}

