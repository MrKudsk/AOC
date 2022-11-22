
const INPUT: &'static str = include_str!("../input/11.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("day 11, output 1: {}", parser1(INPUT));
    println!("day 11, output 2: {}", parser2(INPUT));
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
        assert_eq!(parser1(INPUT), 1656)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 1924)
    }
}

