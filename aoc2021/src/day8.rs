use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("../input/8.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 8, output 1: {}", parser1(INPUT));
    println!("day 8, output 2: {}", parser2(INPUT));
}

fn get_segment(s: &str) -> Vec<&str> {
    s.lines()
        .collect()
}

pub fn parser1(s: &str) -> usize {
    let puzzle = get_segment(s);
    /* for line in &puzzle {
        let res = line.split_once(" | ").unwrap();
        println!("{:?}", res);
    } */
    puzzle.iter()
        .map(|line| line.split_once(" | ").unwrap().1)
        .flat_map(|item| item.split_whitespace().collect_vec())
        .map(|s| s.len())
        .filter(|p| match p {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .collect_vec()
        .len()
}



pub fn parser2(s: &str) -> usize {
    let puzzle = get_segment(s);

    let num = puzzle.iter()
        .fold(0, |acc, line| {
            let (input, seg) = line.split_once(" | ").unwrap();
            let head: Vec<HashSet<_>> = input
                .split_whitespace()
                .map(|s| HashSet::from_iter(s.chars()))
                .collect();

            let mut map: Vec<Option<HashSet<char>>> = vec![None; 10];

            let mut remain = vec![];
            for set in head.into_iter() {
                match set.len() {
                    2 => map[1] = Some(set),
                    4 => map[4] = Some(set),
                    3 => map[7] = Some(set),
                    7 => map[8] = Some(set),
                    _ => remain.push(set),
                }
            }
            
            let head = remain;
            let mut remain = vec![];

            for set in head.into_iter() {
                if set.len() == 6 && set.is_superset(map[4].as_ref().unwrap()) {
                    map[9] = Some(set);
                } else if set.len() == 5 && set.is_superset(map[1].as_ref().unwrap()) {
                    map[3] = Some(set);
                } else {
                    remain.push(set);
                }
            }

            let head = remain;
            let mut remain = vec![];

            for set in head.into_iter() {
                if set.len() == 6 && set.is_superset(map[1].as_ref().unwrap()) {
                    map[0] = Some(set);
                } else {
                    remain.push(set);
                }
            }

            let head = remain;
            let mut remain = vec![];

            for set in head.into_iter() {
                if set.len() == 6 {
                    map[6] = Some(set);
                } else {
                    remain.push(set);
                }
            }

            let head = remain;

            for set in head.into_iter() {
                if set.is_subset(map[6].as_ref().unwrap()) {
                    map[5] = Some(set);
                } else {
                    map[2] = Some(set);
                }
            }

            let map: Vec<_> = map.into_iter().map(|item| item.unwrap()).collect();
            /*let mut output: i32 = 0;
            let mut numbers = seg.split_whitespace().collect_vec().into_iter();
            output += convert_number(&numbers.next().unwrap()) * 1000; 
            output += convert_number(&numbers.next().unwrap()) * 100; 
            output += convert_number(&numbers.next().unwrap()) * 10; 
            output += convert_number(&numbers.next().unwrap());
            */
            //dbg!(&map);
            let mut count = 0;
            for i in seg.split_whitespace() {
                let set: HashSet<char> = HashSet::from_iter(i.chars());
                let n = map.iter().position(|cmp| cmp == &set).unwrap();
                count *= 10;
                count += n;
            }
            //dbg!(input, count);
            acc + count
        });
    num
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 26)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 61229)
    }
}

