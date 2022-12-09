use itertools::Itertools;

#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/09.txt");

#[derive(Debug)]
enum Cmd {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, Clone)]
enum Command {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Pos {
    x: u32,
    y: u32,
}

fn main() {
    println!("day 09, output 1: {}", parse1(INPUT));
    println!("day 09, output 2: {}", parse2(INPUT));
}

fn parse1(s: &str) -> usize {
    let cmds = s.lines()
        .map(|l| {
            let (cmd, moves) = l.split_once(" ").unwrap();
            let moves = moves.parse::<u32>().unwrap();
            match cmd {
                "U" => Cmd::Up(moves),
                "D" => Cmd::Down(moves),
                "L" => Cmd::Left(moves),
                "R" => Cmd::Right(moves),
                &_ => panic!("Mistake in input"),
            }
        })
        .collect::<Vec<Cmd>>();
    println!("Number of commands {}", cmds.len());
    println!("Last comd {:?}", cmds.last());
    let mut x_max:i32 = 0;
    let mut y_max:i32 = 0;
    let mut x_min:i32 = 0;
    let mut y_min:i32 = 0;
    let mut x: i32 = 1;
    let mut y: i32 = 1;
    for cmd in &cmds {
        match cmd {
            Cmd::Up(m) => y += *m as i32, 
            Cmd::Down(m) => y -= *m as i32,
            Cmd::Left(m) => x -= *m as i32,
            Cmd::Right(m) => x += *m as i32,
        }
        if x_max < x {
            x_max = x;
        }
        if x_min > x {
            x_min = x;
        }
        if y_max < y {
            y_max = y;
        }
        if y_min > y {
            y_min = y;
        }
    }
    println!("Min {x_min} {y_min}");
    println!("Max {x_max} {y_max}");
    let mut board = vec![vec!['.'; (x_max - x_min) as usize]; (y_max - y_min) as usize];
    println!("Board is {} * {}", board.len(), board[0].len());

    let mut h = Pos{x: (x_min.abs()+1) as u32 , y: (y_max - 1) as u32};
    let mut t = h.clone();
    dbg!(&h);
    board[t.y as usize][t.x as usize] = '#';
    for cmd in &cmds {
        match cmd {
            Cmd::Up(m) => {
                let y_range = (t.y - 1)..=(t.y + 1);
                for _y in 0..*m {
                    h.y -= 1;
                    //let y_range = (t.y - 1)..=(t.y + 1);
                    if !y_range.contains(&h.y) {
                        t.y -= 1;
                        t.x = h.x;
                    }
                    board[t.y as usize][t.x as usize] = '#';
                }
            }, 
            Cmd::Down(m) => {
                //dbg!(&t);
                let y_range = (t.y)..=(t.y + 1);
                for _y in 0..*m {
                    h.y += 1;
                    //let y_range = (t.y - 1)..=(t.y + 1);
                    if !y_range.contains(&h.y) {
                        t.y += 1;
                        t.x = h.x;
                    }
                    board[t.y as usize][t.x as usize] = '#';
                }
            },
            Cmd::Left(m) => {
                let x_range = (t.x - 1)..=(t.x + 1);
                for _x in 0..*m {
                    let old_h = h.clone();
                    h.x -= 1;
                    //let x_range = (t.x - 1)..=(t.x + 1);
                    if !x_range.contains(&h.x) {
                        t = old_h;
                    }
                    board[t.y as usize][t.x as usize] = '#';
                }
            },
            Cmd::Right(m) => {
                let x_range = (t.x - 1)..=(t.x + 1);
                for _x in 0..*m {                    
                    h.x += 1;
                    //let x_range = (t.x - 1)..=(t.x + 1);
                    if !x_range.contains(&h.x) {
                        t.x += 1;
                        t.y = h.y;
                    }
                    board[t.y as usize][t.x as usize] = '#';
                }
            },
        }
        //print_board(&board);
    }
    
    //print_board(&board);

    board.iter()
        .flatten()
        .filter(|c| *c==&'#')
        .count()
}

fn print_board(board: &Vec<Vec<char>>) {
    for lt in board {
        let line: String = lt.iter().cloned().collect();
        println!("{line}");
    }
    println!("");
} 

fn parse2(s: &str) -> usize {
    let cmds = s.lines()
        .map(|l| {
            let (cmd, moves) = l.split_once(" ").unwrap();
            let moves = moves.parse::<u32>().unwrap();
            match cmd {
                "U" => Cmd::Up(moves),
                "D" => Cmd::Down(moves),
                "L" => Cmd::Left(moves),
                "R" => Cmd::Right(moves),
                &_ => panic!("Mistake in input"),
            }
        })
        .collect::<Vec<Cmd>>();
    println!("Number of commands {}", cmds.len());
    println!("Last comd {:?}", cmds.last());
    let mut x_max:i32 = 0;
    let mut y_max:i32 = 0;
    let mut x_min:i32 = 0;
    let mut y_min:i32 = 0;
    let mut x: i32 = 1;
    let mut y: i32 = 1;
    for cmd in &cmds {
        match cmd {
            Cmd::Up(m) => y += *m as i32, 
            Cmd::Down(m) => y -= *m as i32,
            Cmd::Left(m) => x -= *m as i32,
            Cmd::Right(m) => x += *m as i32,
        }
        if x_max < x {
            x_max = x;
        }
        if x_min > x {
            x_min = x;
        }
        if y_max < y {
            y_max = y;
        }
        if y_min > y {
            y_min = y;
        }
    }
    println!("Min {x_min} {y_min}");
    println!("Max {x_max} {y_max}");
    let mut board = vec![vec!['.'; (x_max - x_min) as usize]; (1 + y_max - y_min) as usize];
    println!("Board is {} * {}", board.len(), board[0].len());

    let moves = cmds.iter()
        .map(|cmd| {
            match cmd {
                Cmd::Up(m) => vec![Command::Up; *m as usize], 
                Cmd::Down(m) => vec![Command::Down; *m as usize],
                Cmd::Left(m) => vec![Command::Left; *m as usize],
                Cmd::Right(m) => vec![Command::Right; *m as usize],
            }
        })
        .flatten()
        .collect::<Vec<Command>>();
    //dbg!(&moves);

    let h = Pos{x: (x_min.abs()+1) as u32 , y: (y_max - 1) as u32};
    let mut rops = [(h.x as i32,h.y as i32);10];
    //let last_rop = (rops.len() - 1) as usize;

    for cmd in &moves {
        match cmd {
            Command::Up => {
                rops[0].1 -= 1;
            }, 
            Command::Down => {
                rops[0].1 += 1;
            },
            Command::Left => {
                rops[0].0 -= 1;
            },
            Command::Right => {
                rops[0].0 += 1;
            },
        }
        //board[rops[0].1 as usize][rops[0].0 as usize] = 'H'; 

        for i in 1..rops.len() {
            let x_range = (rops[i-1].0 - 1)..=(rops[i-1].0 + 1);
            let y_range = (rops[i-1].1 - 1)..=(rops[i-1].1 + 1);

            let tail_is_scope = x_range
                .cartesian_product(y_range)
                .any(|tuple| tuple == (rops[i].0, rops[i].1));
            //dbg!(&tail_is_scope);

            if !tail_is_scope {
                if rops[i-1].0 == rops[i].0 {
                    if rops[i-1].1 > rops[i].1 {
                        rops[i].1 += 1;
                    } else {
                        rops[i].1 -= 1;
                    }
                } else if rops[i-1].1 == rops[i].1 {
                    if rops[i-1].0 > rops[i].0 {
                        rops[i].0 += 1;
                    } else {
                        rops[i].0 -= 1;
                    }
                } else if rops[i-1].1 < rops[i].1 && rops[i-1].0 > rops[i].0 {
                    rops[i].0 += 1;
                    rops[i].1 -= 1;
                } else if rops[i-1].1 > rops[i].1 && rops[i-1].0 > rops[i].0 {
                    rops[i].0 += 1;
                    rops[i].1 += 1;
                } else if rops[i-1].1 > rops[i].1 && rops[i-1].0 < rops[i].0 {
                    rops[i].0 -= 1;
                    rops[i].1 += 1;
                } else if rops[i-1].1 < rops[i].1 && rops[i-1].0 < rops[i].0 {
                    rops[i].0 -= 1;
                    rops[i].1 -= 1;
                } 
            }
            //board[rops[i].1 as usize][rops[i].0 as usize] = format!("{i}").chars().nth(0).unwrap(); 
            //println!("{} rop({},{})", i, rops[i].0, rops[i].1);
        }
        board[rops[9].1 as usize][rops[9].0 as usize] = '#';
        //print_board(&board);
    }
    //print_board(&board);

    board.iter()
        .flatten()
        .filter(|c| *c==&'#')
        .count() 
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 13);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT2), 36);
    }
}
