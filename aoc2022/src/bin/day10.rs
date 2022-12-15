use std::collections::BTreeMap;

use itertools::Itertools;
use nom::{IResult, 
    multi::separated_list1, 
    branch::alt,
    bytes::complete::{tag, is_a},
    character::complete::newline
};

#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/10.txt");

#[derive(Debug)]
enum Cmd {
    Noop,
    Addx(i32),
}

impl Cmd {
    fn cycles(&self) -> u32 {
        match self {
            Cmd::Noop => 1,
            Cmd::Addx(_) => 2,
        }
    }
}

fn main() {
    println!("day 10, output 1: {}", parse1(INPUT));
    println!("day 10, output 2: {}", parse2(INPUT));
}

fn noop(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Cmd::Noop))
}

fn addx(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("addx ")(input)?;
    let (input, xs) = is_a("-0123456789")(input)?;
    Ok((input, Cmd::Addx(xs.parse::<i32>().unwrap())))
}

/*
fn get_x_at(cmds: &Vec<Cmd>, cycle: u32) -> i32 {
    let mut x = 1;
    let mut cmds_iter = cmds.iter();
    let mut next_cmd = 0;
    for i in 0..cycle {
        if next_cmd == 0 {
            match cmds_iter.next().unwrap() {
                Cmd::Noop => next_cmd = 0,
                Cmd::Addx(no) => next_cmd = *no,
            }
        } else {
            x += next_cmd;
            next_cmd = 0;
        }
        if i > 210 {
            println!("{} - {} -> next {}", i,  x, next_cmd);
        }
    }
    println!("{} cycle x {} sum {}", cycle, x, x * cycle as i32);
    //dbg!(cmds_iter.next());
    x * cycle as i32
}
*/

fn parse1(s: &str) -> i32 {
    let (_ , cmds) = separated_list1(newline, alt((noop, addx)))(s).unwrap();
    //dbg!(&cmds);
    let notable_cycles = vec![20,60,100,140,180,220];
    let mut signal: BTreeMap<u32, i32> = BTreeMap::new();

    let mut x: i32 = 1;
    let mut cycles: u32 = 0;

    for cmd in cmds.iter() {
        if notable_cycles.contains(&(cycles + 1)) {
            signal.insert(cycles + 1 , (cycles as i32 + 1 ) * x);
        }
        if notable_cycles.contains(&(cycles + 2)) {
            signal.insert(cycles + 2, (cycles as i32 + 2 ) * x);
        }

        cycles += cmd.cycles();
        match cmd {
            Cmd::Noop => {},
            Cmd::Addx(num) => {x += num;},
        }
    }

    dbg!(&signal);
    let res = signal.iter()
        .map(|(_, value)| value)
        .sum();


/*    let res = cycles.iter()
        .map(|cycle| {
            get_x_at(&cmds, *cycle)
        })
        .sum();
*/
    res
}

fn parse2(s: &str) -> String {
    let (_ , cmds) = separated_list1(newline, alt((noop, addx)))(s).unwrap();
    let mut x: i32 = 1;
    let mut cycles: u32 = 0;
    let mut crt_display: String = "".to_string();

    for cmd in cmds.iter() {
        for cycle_add in 0..cmd.cycles() {
            let pid = (cycles as i32 + cycle_add as i32) % 40;

            if ((x-1)..=(x+1)).contains(&pid) {
                crt_display.push_str("#");
            } else {
                crt_display.push_str(".");
            }
        }

        cycles += cmd.cycles();
        match cmd {
            Cmd::Noop => {},
            Cmd::Addx(num) => {x += num;},
        };
/*        let res = crt_display.chars()
            .chunks(40)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .join("\n");

        println!("{res}"); */
    }
    
    let res = crt_display.chars()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n");

    println!("{res}");
    "#".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 13140);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }
}
