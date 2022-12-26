#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/17.txt");

fn main() {
    println!("day 17, output 1: {}", parse1(INPUT));
    println!("day 17, output 2: {}", parse2(INPUT));
}

fn parse1(s: &str) -> usize {
    todo!()
}

fn parse2(s: &str) -> usize {
    todo!()
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
        assert_eq!(parse2(INPUT), 1);
    }
}
