#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/13.txt");

fn main() {
    println!("day 13, output 1: {}", parse1(INPUT));
    println!("day 13, output 2: {}", parse2(INPUT));
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

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 13);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1);
    }
}
