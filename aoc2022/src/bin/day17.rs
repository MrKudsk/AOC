use core::iter::Cycle;
use std::collections::{HashMap, VecDeque};

#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/17.txt");

const OUCH: usize = 1_000_000_000_000;

const SHAPES: [[u8; 4]; 5] = [
    [0b0000000, 0b0000000, 0b0000000, 0b0011110],
    [0b0000000, 0b0001000, 0b0011100, 0b0001000],
    [0b0000000, 0b0000100, 0b0000100, 0b0011100],
    [0b0010000, 0b0010000, 0b0010000, 0b0010000],
    [0b0000000, 0b0000000, 0b0011000, 0b0011000],
];

struct Chamber<'a> {
    rocks: VecDeque<u8>,
    jets: Vec<char>,
    jetsnum: usize,
    shapes: Cycle<std::slice::Iter<'a, [u8; 4]>>,
}

impl Chamber<'_> {
    /*    const SHAPES: [[u8; 4]; 5] = [
            [0b0000000, 0b0000000, 0b0000000, 0b0011110],
            [0b0000000, 0b0001000, 0b0011100, 0b0001000],
            [0b0000000, 0b0000100, 0b0000100, 0b0011100],
            [0b0010000, 0b0010000, 0b0010000, 0b0010000],
            [0b0000000, 0b0000000, 0b0011000, 0b0011000],
        ];
    */
    fn new(jets: Vec<char>) -> Self {
        Self {
            rocks: VecDeque::from(vec![0,0,0,0,0,0,0]),
            jets,
            jetsnum: 0,
            shapes: self::SHAPES.iter().cycle(),
        }
    }

    fn drop_one(&mut self) {
        let mut piece: [u8; 4] = *self.shapes.next().unwrap();
        let mut last = self.rocks.len() - 7;
        while self.rocks[last] != 0 {
            self.rocks.push_back(0);
            last += 1;
        }

        let mut bottom = self.rocks.len() - 4;

        loop {
            let jet = self.jets[self.jetsnum];
            self.jetsnum = (self.jetsnum + 1) % self.jets.len();

            match jet {
                '<' => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p <<= 1;
                        }
                    }
                }
                '>' => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p >>= 1;
                        }
                    }
                }
                _ => panic!("bad input"),
            }

            // drop the piece by one if it can
            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break;
            }
        }

        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            self.rocks[bottom] |= piece[p_row];
            bottom += 1;
        }
    }

    fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            if (piece[p_row] & 0x40) != 0 || (self.rocks[bottom] & (piece[p_row] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            if (piece[p_row] & 0x01) != 0 || (self.rocks[bottom] & (piece[p_row] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut p_row = 4;
        while p_row > 0 {
            p_row -= 1;
            if (self.rocks[bottom] & piece[p_row]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    // find the height by go from top until first line include a rock.
    fn height(&self) -> usize {
        let mut top = self.rocks.len();
        while top > 0 && self.rocks[top - 1] == 0 {
            top -= 1;
        }
        top
    }

    fn _print_row(row: u8) {
        let mut bit = 0b1000000;
        while bit > 0 {
            print!("{}", if (bit & row) != 0 { "#" } else { "." });
            bit >>= 1;
        }
    }

    fn _draw(&self) {
        let mut top = self.rocks.len();
        while top > 0 {
            top -= 1;
            print!("|");
            Self::_print_row(self.rocks[top]);
            println!("|");
        }
        println!("+-------+");
    }
}

fn main() {
    println!("day 17, output 1: {}", parse1(INPUT));
    println!("day 17, output 2: {}", parse2(INPUT));
}

fn parse1(s: &str) -> usize {
    let mut jets = s.chars().collect::<Vec<char>>();
    if jets[jets.len() - 1] == '\n' {
        jets.pop();
    }
    let mut chamber = Chamber::new(jets);
    for _ in 0..2022 {
    // for _ in 0..1_000_000_000_000usize {
        chamber.drop_one();
    }
    chamber.height()
}

fn parse2(s: &str) -> usize {
    let mut jets = s.chars().collect::<Vec<char>>();
    if jets[jets.len() - 1] == '\n' {
        jets.pop();
    }

    let mut height = 0;
    let mut drops = 0;
    let mut chamber = Chamber::new(jets);
    
    loop {
        for _ in 0..1_000_000_000 {
        // for _ in 0..1_000_000_000_000usize {
            chamber.drop_one();
            drops += 1;
            if drops >= OUCH {
                return height + chamber.height();
            }
        }
        let delta_height = chamber.height() - 10;
        height += delta_height;
        //println!("height {height} after {drops} drops.");
        for _ in 0..delta_height {
            chamber.rocks.pop_front();
        }
        println!("{}", drops);
        //chamber._draw();
    }
   //height
}

fn _parse2(s: &str) -> usize {
    let mut jets = s.chars().collect::<Vec<char>>();
    if jets[jets.len() - 1] == '\n' {
        jets.pop();
    }
    // state will be; current piece number, current jet index, top 4 rows of chamber
    // if we get a repeat, then we found a cycle
    //      -- delta_height: height from revious cycle to this on
    //      -- delta_drops: how many drops were needed to get to delta_height
    //      -- offset_height: how high the tower was when the cycle began for the first time
    //      -- offset_drops: how many drops it took to get to offset_height

    let mut drops = 0;
    let mut piecnum = 0;
    let mut cycle_map = HashMap::new();
    let mut chamber = Chamber::new(jets);

    // map state to (height, drops)
    cycle_map.insert((piecnum, chamber.jetsnum, 0usize), (0usize, 0usize));

    loop {
        chamber.drop_one();
        drops += 1;
        piecnum = (piecnum + 1) % SHAPES.len();

        let height = chamber.height();
        if height < 4 {
            continue;
        }

        let shap = ((chamber.rocks[height - 1] as usize) << 24)
            | ((chamber.rocks[height - 2] as usize) << 16)
            | ((chamber.rocks[height - 3] as usize) << 8)
            | (chamber.rocks[height - 4] as usize);

        if let Some(entry) = cycle_map.get(&(piecnum, chamber.jetsnum, shap)) {
            println!("piece  = {}", piecnum);
            println!("jets   = {}", chamber.jetsnum);
            println!("shap   = {}", shap);
            println!("height = {}", height);
            println!("drops  = {}", drops);

            let delta_height = height - entry.0;
            let delta_drops = drops - entry.1;
            println!("There is an increase of {delta_height} rows for every {delta_drops} drops");

            let remaining = OUCH - entry.1;
            println!("There are still {} left to go - {}", remaining, entry.1);

            let needed = remaining / delta_drops;
            let leftover = remaining % delta_drops;
            let integral_height = entry.0 + delta_height * needed;
            println!("The height will reach {integral_height}, but there are still {leftover} drops left");

            for _ in 0..leftover {
                chamber.drop_one();
            }

            let leftover_height = chamber.height() - height;
            println!("After {leftover} more drops, we added {leftover_height} rows.");

            return integral_height + leftover_height;
        } else {
            cycle_map.insert((piecnum, chamber.jetsnum, shap), (height, drops));
        }
    }
    // chamber.height()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 3068);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1514285714288);
    }
}
