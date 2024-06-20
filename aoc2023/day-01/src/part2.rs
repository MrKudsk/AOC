use crate::custom_error::AocError;
use aho_corasick::AhoCorasick;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let nums = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let ac = AhoCorasick::new(nums).unwrap();

    let output = input
        .lines()
        .map(|line| {
            // println!("{line}");
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();

            let first = matches.first().unwrap().pattern().as_i32() / 2 + 1;
            let last = matches.last().unwrap().pattern().as_i32() / 2 + 1;
            // println!("{} - {}", first.to_string(), last.to_string());

            first * 10 + last
        })
        .sum::<i32>();

    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
