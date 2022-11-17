use itertools::Itertools;

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

fn convert_number(s: &str) -> i32 {
    let mut first = match s.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        8 => 7,
        _ => 0,
    };
    /*if s.len() == 6 {
        first = match s.con
    }
    if first == 0 {
        first = match s {
            "fdgacbe"   => 8,
            "cefdb"     => 3,
            "cefbgd"    => 9,
            "fcgedb"    => 9,
            _ => {
                let ch = s.clone().chars().sorted();    
                dbg!(ch.);
                0},
        
        };
    }*/
    first
}
pub fn parser2(s: &str) -> i32 {
    let puzzle = get_segment(s);

    let num = puzzle.iter()
        .fold(0, |acc, line| {
            let (input, seg) = line.split_once(" | ").unwrap();
            let mut output: i32 = 0;
            let mut numbers = seg.split_whitespace().collect_vec().into_iter();
            output += convert_number(&numbers.next().unwrap()) * 1000; 
            output += convert_number(&numbers.next().unwrap()) * 100; 
            output += convert_number(&numbers.next().unwrap()) * 10; 
            output += convert_number(&numbers.next().unwrap()); 
            dbg!(input, output);
            acc + output
        });
    dbg!(num);
    todo!();
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

