use itertools::Itertools;

const INPUT: &'static str = include_str!("../input/12.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 12, output 1: {}", parser1(INPUT));
    println!("day 12, output 2: {}", parser2(INPUT));
}

fn read(s: &str) -> Vec<Vec<String>> {
    s.lines()
        .map(|l| l.split_once("-").unwrap().collect_vec())
        .collect_vec()
}

pub fn parser1(s: &str) -> i32 {
    let paths = read(s);
    dbg!(paths);
    todo!();
}

pub fn parser2(s: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    const SMALL: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";    

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 226)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 1924)
    }
}

