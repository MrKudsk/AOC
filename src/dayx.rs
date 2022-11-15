
const INPUT: &'static str = include_str!("../input/5.txt");

pub fn run() {
    println!("day 4, output 1: {}", parser1(INPUT));
    println!("day 4, output 2: {}", parser2(INPUT));
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

    const INPUT: &str = "";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 4512)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 1924)
    }
}

