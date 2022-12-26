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


fn get_dist(from: &str, to: &str, graph: &DiGraphMap<&str, &str>) -> usize {
    let res = dijkstra(
        graph,
        from,
        Some(to),
        |_| 1,
    );
    res[to]
}

fn solve(pos: &str, time: usize, 
    target_nodes: &Vec<&str>, 
    valves: &BTreeMap<&str, Valve>, 
    graph: &DiGraphMap<&str, &str>
) -> usize {
    let score = target_nodes.iter()
        .map(|target| {
            let dist = get_dist(pos, target, gr;
            let flow = valves.get(target).unwrap().rate as usize;
            (target, (time - dist - 1) * flow, dist)
        })
        .collect::<Vec<_>>();
    println!("Score {:?}", score);
    1
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

    let target_nodes = valves.iter()
        .filter(|(_k, v)| v.rate > 0)
        .map(|(_k, v)| v.name)
        .collect::<Vec<_>>();
    dbg!(&target_nodes);

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
    //dbg!(&edges);
    
    let graph = DiGraphMap::<&str,&str>::from_edges(&edges,);

    println!("DD -> BB is {}", get_dist(&"DD", &"BB", &graph));

    let res = solve(&"AA", 30, &target_nodes, &valves, &graph);
    res
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
