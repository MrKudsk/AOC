use nom::{IResult, multi::separated_list1, sequence::{preceded, delimited}, bytes::complete::{is_a, tag}
, character::complete::{newline}, branch::alt};
use std::collections::{BTreeMap, VecDeque};
use petgraph::prelude::*;
use petgraph::{
    algo::dijkstra,
    dot::{Config, Dot},
};

#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/16.txt");

fn main() {
    println!("day 16, output 1: {}", parse1(INPUT));
    println!("day 16, output 2: {}", parse2(INPUT));
}

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    rate: u32,
    leads: Vec<&'a str>,
    open: bool,
}

fn get_leads(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag("tunnels lead to valves "), separated_list1(tag(", "), is_a("QWERTYUIOPASDFGHJKLZXCVBNM")))
    (input)
}

fn get_lead(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, lead) = preceded(tag("tunnel leads to valve "), is_a("QWERTYUIOPASDFGHJKLZXCVBNM"))(input)?;
    Ok((input, vec![lead]))
}

fn get_valve(input: &str) -> IResult<&str, Valve> {
    let (input, valve) = delimited(tag("Valve "), is_a("QWERTYUIOPASDFGHJKLZXCVBNM"), tag(" has flow rate"))(input)?;
    let (input, rate) = delimited(tag("="), nom::character::complete::u32, tag("; "))(input)?;
    let (input, leads) = alt((get_leads, get_lead)) 
        (input)?;
    //dbg!(&leads);
    let mut open = false;
    if valve == "AA" {
        open = true;
    }
    Ok((input, Valve{name: valve, rate: rate, leads: leads, open}))
}

fn get_map(input: &str) -> IResult<&str, BTreeMap<&str, Valve>> {
    let (input, list) = separated_list1(newline, get_valve)(input)?;
    Ok((input, list.into_iter()
        .map(|valve| {
            (valve.name, valve)
        })
        .collect::<BTreeMap<&str, Valve>>()))
}
/*
fn get_neighbors(key: &str, valves: &BTreeMap<&str, Valve>) -> Vec<(&str, u8)> {
    valves.get(&*key).unwrap().leads
    .iter()
    .map(|v| (v, 1))
    .collect::<Vec<(&str, u8)>>()
}
        if casched.contains_key(&(pos,timer - 1)) {
           score = *casched.get(&(pos,timer-1 )).unwrap();
           println!("casched {}", score);
        } else {
            score = valve.leads.iter()
                .map(|p| {
                    solve(p, timer -1  , opened, &valves, casched)
                })
                .max().unwrap();
            casched.insert((pos, timer - 1), score);
        }*/

fn solve<'a>(
        pos: &'a str, 
        timer: i32, 
        mut opened: &mut Vec<&'a str>, 
        valves: &BTreeMap<&str, Valve<'a>>,
        casched: &mut BTreeMap<(i32, &'a str), i32>
    ) -> i32 {
    println!("time is {} {}", timer, &pos);
    let mut score = 0;
    if casched.contains_key(&(timer, &pos)) {
        score = *casched.get(&(timer, &pos)).unwrap();
    } else {
        if timer > 1 {
            let valve = valves.get(pos).unwrap();
            //dbg!(&valve);
                score = valve.leads.iter()
                    .map(|p| {
                        println!(" solve {} {:?}", p, opened);
                        //opened.push(pos);
                        solve(p, timer -1  , &mut opened, &valves, casched)
                    })
                    .max().unwrap();

            println!("rate {} is opened {:?}", valve.rate, opened);
            if valve.rate > 0 && !opened.contains(&pos) {
                opened.push(pos);
                let mut new_opened = opened.clone();
                println!("new {:?}", new_opened);
                let flow = ((timer - 1) * valve.rate as i32) + solve(pos, timer -1, &mut new_opened, &valves, casched);
                println!("{} * {} = {}", (timer - 1), valve.rate, flow);
                if flow > score {
                    score = flow;
                } 
            }
            casched.insert((timer, &pos), score);
        }
    }
    score
}

fn dist<'a>(pos: &'a str, time: u32, valves: &BTreeMap<&str, Valve<'a>>) {
    let valve = valves.get(pos).unwrap();
    let t = valve.leads.iter()
        .map(|p| {
            let t = dist(p, time + 1, &valves);
            println!("{:?}", t);
            (*p, time + 1)
        })
        .filter(|(p, _)| valves.get(p).unwrap().rate > 0)
        .collect::<Vec<(&str, u32)>>();
    dbg!(t);
}

fn parse1(s: &str) -> usize {
    let timer = 30; //Minutes
    let (_, valves) = get_map(s).unwrap();

    let mut cashed: BTreeMap<(i32, &str), i32> = BTreeMap::new();
    let mut opened: Vec<&str> = Vec::new();
    //let t = solve(&"AA", timer, &mut opened, &valves, &mut cashed);
    //let t = dist(&"AA", 0, &valves);

    let node = valves.iter()
        .map(|(_k, v)| v.name)
        .collect::<Vec<_>>();

    dbg!(&node);

    /*
    let edges = valves.iter()
        .enumerate()
        .map(|(i, (_k, v))| {
            let neighbors = v.leads.iter()
                .map(|val| {
                    (i, node.binary_search(val).unwrap())
                })
                .collect::<Vec<_>>();
            neighbors
        })
        .flatten()
        .collect::<Vec<_>>();
    dbg!(&edges);
    
    let graph = DiGraphMap::<usize,usize>::from_edges(&edges,);
    let res = dijkstra(
        &graph,
        (node.binary_search(&"DD").unwrap()),
        Some(node.binary_search(&"BB").unwrap()),
        |_| 1,
    );
    dbg!(&res);
    */
    let edges = valves.iter()
        .map(|(k, v)| {
            let neighbors = v.leads.iter()
                .map(|val| {
                    (*k, *val)
                })
                .collect::<Vec<_>>();
            neighbors
        })
        .flatten()
        .collect::<Vec<_>>();
    dbg!(&edges);
    
    let graph = DiGraphMap::<&str,&str>::from_edges(&edges,);
    let res = dijkstra(
        &graph,
        (&"DD"),
        Some(&"BB"),
        |_| 1,
    );
    dbg!(&res);
    todo!()
}

fn parse2(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 1651);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1);
    }
}
