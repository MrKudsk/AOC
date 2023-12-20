use crate::custom_error::AocError;

#[derive(Debug, Default)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

impl Turn {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let games: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, turns) = line.split_once(": ").unwrap();
            let turns = turns.split("; ").collect::<Vec<_>>();
            //let mut turn_result = Vec::new();
            let rest: Vec<_> = turns
                .iter()
                .map(|t| {
                    let mut turn = Turn::default();
                    let cubes = t.split(", ").collect::<Vec<_>>();
                    for c in cubes {
                        let (amount, color) = c.split_once(' ').unwrap();
                        let amount: usize = amount.parse().unwrap();
                        match &color[0..1] {
                            "r" => turn.red = amount,
                            "g" => turn.green = amount,
                            "b" => turn.blue = amount,
                            _ => panic!("bug in code"),
                        }
                    }
                    turn
                })
                .collect();
            rest
        })
        .collect();
    let mut game_total = 0;

    'nextgame: for (index, game) in games.iter().enumerate() {
        for turn in game {
            if !turn.is_valid() {
                continue 'nextgame;
            }
        }
        game_total += index + 1;
    }

    Ok(game_total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
