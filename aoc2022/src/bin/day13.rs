#[allow(unused_variables)]
use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};
use std::cmp::Ordering::{self, *};

const INPUT: &'static str = include_str!("../../input/13.txt");

#[derive(Debug, Eq)]
enum Packet {
    List(Vec<Packet>),
    Num(u32),
}

#[derive(Debug, PartialEq)]
struct Pair {
    l: Packet,
    r: Packet,
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Num(l0), Self::Num(r0)) => l0 == r0,
            (Self::List(l0), Self::Num(r0)) => {
                l0 == &vec![Packet::Num(*r0)] 
            },
            (Self::Num(l0), Self::List(r0)) => {
                &vec![Packet::Num(*l0)] == r0
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Num(b)) => {
                a.cmp(&vec![Packet::Num(*b)])
            },
            (Packet::Num(a), Packet::List(b)) => {
                vec![Packet::Num(*a)].cmp(&b)
            },
            (Packet::Num(a), Packet::Num(b)) => {
                a.cmp(b)
            }
            
        }
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt(
        (delimited(tag("["), separated_list0(tag(","), packet), tag("]"),)
            .map(|vec| Packet::List(vec))
        , 
        nom::character::complete::u32
            .map(|num| Packet::Num(num))
        ),
    )(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet,)
            .map(|(p1, p2)| Pair{l: p1, r: p2,}),
    )(input)
}

fn main() {
    println!("day 13, output 1: {}", parse1(INPUT));
    println!("day 13, output 2: {}", parse2(INPUT));
}

fn parse1(s: &str) -> usize {
    let (_, list) = pairs(&s).unwrap();
    //dbg!(&list);
    let res = list.iter()
        .enumerate()
        .filter_map(|(i, Pair{l, r})| {
            let res = l.cmp(r);
            //dbg!(&res);
            match res {
                Greater => None,
                Less => Some(i + 1),
                Equal => panic!("Help somethings worrong equal??"),
            }
        })
        .sum::<usize>();
    res
}

fn parse2(s: &str) -> usize {
    let (_, plist) = pairs(&s).unwrap();
    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]); 
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]); 
    let mut packets: Vec<&Packet> = plist.iter()
        .flat_map(|Pair{l, r}| [l, r])
        .chain([&packet_6, &packet_2])
        .collect();
    packets.sort();
    //dbg!(&packets);
    let index_2 = packets.iter()
        .enumerate()
        .find(|(i, pack)| pack == &&&packet_2)
        .unwrap();
    
    let index_6 = packets.iter()
        .enumerate()
        .find(|(i, pack)| pack == &&&packet_6)
        .unwrap();
    
    //dbg!(index_2, index_6);
    (index_2.0 + 1) * (index_6.0 + 1)
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
        assert_eq!(parse2(INPUT), 140);
    }
}
