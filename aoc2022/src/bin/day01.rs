#[allow(unused_variables)]
use itertools::Itertools;
use itertools::max;

const INPUT: &'static str = include_str!("../../input/01.txt");

fn main() {
    println!("day 01, output 1: {}", parse1(INPUT));
    println!("day 01, output 2: {}", parse2(INPUT));
}

fn parse1(s: &str) -> usize {
    let tmp: Vec<i32> = s.split("\n\n")
            .map(|elf| elf.lines()
                .map(|l| l.parse::<i32>().unwrap_or(0))
                .collect_vec()
                .iter().sum())
        .collect_vec();
    //dbg!(&tmp);
    max(tmp).unwrap() as usize
}    

fn parse2(s: &str) -> usize {
    let mut tmp: Vec<i32> = s.split("\n\n")
            .map(|elf| elf.lines()
                .map(|l| l.parse::<i32>().unwrap_or(0))
                .collect_vec()
                .iter().sum())
        .collect_vec();
    tmp.sort();
    
    tmp.iter()
        .rev()
        .take(3)
        .sum::<i32>() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 24000);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 45000);
    }
}
