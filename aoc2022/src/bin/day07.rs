#[allow(unused_variables)]
use std::collections::BTreeMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, is_a},
    multi::separated_list1,
    character::complete::{alpha1, newline},
    sequence::separated_pair,
    *,
};
use itertools::min;

const INPUT: &'static str = include_str!("../../input/07.txt");

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File {size:u32, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size,name)) = separated_pair(
        nom::character::complete::u32, 
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;
    Ok((input, Files::File { size, name}))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(
        newline,
        alt((file, directory)),
    )(input)?;
    //dbg!(&files);
    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/"     => Operation::Cd(Cd::Root),
        ".."    => Operation::Cd(Cd::Up),
        name    => Operation::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((ls,cd)))(input)?;
    //println!("{:?}",cmds);
    Ok((input, cmds))
}

fn main() {
    println!("day 07, output 1: {}", parse1(INPUT));
    println!("day 07, output 2: {}", parse2(INPUT));
}

fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>), 
    command: &'a Operation) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        Operation::Cd(Cd::Root) => { context.push(""); }
        Operation::Cd(Cd::Up) => { context.pop(); }
        Operation::Cd(Cd::Down(name)) => { context.push(name); }
        Operation::Ls(files) => {
            let sum = files.iter()
                .filter_map(|file| {
                    if let Files::File { size, .. } = file {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum::<u32>();
            for i in 0..context.len() {
                sizes.entry(context[0..=i].to_vec())
                    .and_modify(|v| *v += sum)
                    .or_insert(sum);
            }
        }
        
    };
    (context, sizes)
}

fn parse1(s: &str) -> usize {
    let cmds = commands(s).unwrap().1;
    
    let (_, sizes) = cmds.iter().fold(
        (vec![], BTreeMap::new()), calculate_sizes,
    );
   // dbg!(&sizes);

    let res = sizes.iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u32>();
    res as usize
}

fn parse2(s: &str) -> usize {
    let cmds = commands(s).unwrap().1;
    
    let (_, sizes) = cmds.iter().fold(
        (vec![], BTreeMap::new()), calculate_sizes,
    );
    //dbg!(&sizes);

    let total_space = 70_000_000;
    let need_space = 30_000_000;

    let used_space = sizes.get(&vec![""]).unwrap();
    println!("Used {}", used_space);

    let free_space = total_space - used_space;
    println!("Free {}", free_space);
    let need_to_free_space: u32 = need_space - free_space;
    println!("Need {:?}", need_to_free_space);

    let mut valid_dirs = sizes.iter()
        .filter(|(_, &size)| size > need_to_free_space)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    //valid_dirs.sort();
    *min(valid_dirs).unwrap() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 95437);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 24933642);
    }
}
