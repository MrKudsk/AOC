
const INPUT: &'static str = include_str!("../input/7.txt");

pub fn run() {
    println!("day 7, output 1: {}", parser1(INPUT));
    println!("day 7, output 2: {}", parser2(INPUT));
}

fn get_number(s: &str) -> Vec<i32> {
    s.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

pub fn parser1(s: &str) -> i32 {
    let mut puzzle = get_number(s);

    let nth = puzzle.len() / 2;
    let (_, midian, _) = &puzzle.select_nth_unstable(nth);
    let midian: i32 = **midian;
    let cost = puzzle.iter().fold(0, |acc, pos| i32::abs(pos - midian) + acc);
    //dbg!(cost);

    cost    
}

pub fn parser2(s: &str) -> i32 {
    let puzzle = get_number(s);

    let min = puzzle.iter().min().unwrap();
    let max = puzzle.iter().max().unwrap();
    let cost = (*min..=*max)
        .into_iter()
        .map(|mid| {
            puzzle.iter()
                .map(|pos| {
                    let dist = (pos - mid).abs();
                    (1 + dist) * dist / 2
                })
                .sum()
        })
        .min()
        .unwrap();

    cost    
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14\n";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 37)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 168)
    }
}

