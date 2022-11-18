
const INPUT: &'static str = include_str!("../input/9.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 9, output 1: {}", parser1(INPUT));
    println!("day 9, output 2: {}", parser2(INPUT));
}

pub fn parser1(s: &str) -> i32 {
    todo!();
}

pub fn parser2(s: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 15)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 1924)
    }
}

