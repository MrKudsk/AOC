
const INPUT: &'static str = include_str!("../input/7.txt");

pub fn run() {
    println!("day 7, output 1: {}", parser1(INPUT));
    println!("day 7, output 2: {}", parser2(INPUT));
}

pub fn parser1(s: &str) -> i32 {
    dbg!(s);
    todo!();
}

pub fn parser2(s: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14\n";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 4512)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 1924)
    }
}

