#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/05.txt");

fn main() {
    println!("day 05, output 1: {}", parse1(INPUT));
    println!("day 05, output 2: {}", parse2(INPUT));
}

fn parse1(s: &str) -> String {
    todo!()
}

fn parse2(_s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), "CMZ");
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1);
    }
}
