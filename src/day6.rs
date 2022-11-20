use std::collections::VecDeque;

const INPUT: &'static str = include_str!("../input/6.txt");

pub fn run() {
    println!("-----------------------------------");
    println!("There total of fish after 80 days");
    println!("day 6, output 1: {}", parser1(INPUT));
    println!("day 6, output 2: {}", parser2(INPUT));
}

fn counts_fish(fish: &Vec<usize>, days: u32) -> u64 {
    // We count the fish with the same age into a Vec
    let mut counts = VecDeque::from(vec![0;9]);

    // Add init fish age into
    fish.iter().for_each(|f| counts[*f] += 1);

    (0..days).for_each(|_| {
        let babies = counts.pop_front().unwrap();
        counts[6] += babies;
        counts.push_back(babies);
    });

    counts.iter().sum()
}

pub fn parser1(s: &str) -> u64 {
    let fish: Vec<usize> = s.strip_suffix("\n").unwrap().split(',').map(|i| i.parse().unwrap()).collect();
    counts_fish(&fish, 80)
}

pub fn parser2(s: &str) -> u64 {
    let fish: Vec<usize> = s.strip_suffix("\n").unwrap().split(',').map(|i| i.parse().unwrap()).collect();
    counts_fish(&fish, 256)
}
/*
#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn first() {
        assert_eq!(parser1(INPUT), 5934)
    }

    #[test]
    fn second() {
        assert_eq!(parser2(INPUT), 26984457539)
    }
}
*/
