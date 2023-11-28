#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/19.txt");

fn main() {
    println!("day 19, output 1: {}", parse1(INPUT));
    println!("day 19, output 2: {}", parse2(INPUT));
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

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
  Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 33);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 24);
    }
}
