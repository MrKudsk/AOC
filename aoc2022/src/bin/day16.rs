use std::collections::HashMap;
use regex::Regex;

const INPUT: &'static str = include_str!("../../input/16.txt");

fn main() {
    //println!("day 16, output 1: {}", parse1(INPUT, 30));
    println!("day 16, output 2: {}", parse2(INPUT, 26));
}

fn solve(node: &str, 
    time: i32, 
    flow: &HashMap<&str, i32>,
    graph: &HashMap<&str, Vec<&str>>,
    visited: &mut Vec<String>,
    cashe: &mut HashMap<(String, Vec<String>, i32, bool), i32>,
    elephane: bool,
) -> i32 {
    if time <= 0 {
        if elephane {
            return solve("AA", 26, flow, graph, visited, cashe, false)
        }
        return 0;
    }
    if let Some(&ans) = cashe.get(&(node.to_string(), visited.clone(), time, elephane)) {
        return ans;
    }
    let mut best = i32::MIN;
    if !visited.contains(&node.to_string()) && *flow.get(node).unwrap() > 0 {
        for &neighbor in graph.get(&node).unwrap() {
            visited.push(node.to_string());
            let sub_result = solve(neighbor, time - 2 , flow, graph, visited, cashe, elephane);
            best = best.max(sub_result + flow.get(node).unwrap() * (time -1));
            visited.pop();
        }

    }
    for &neighbor in graph.get(node).unwrap() {
        let sub_result = solve(neighbor, time - 1, flow, graph, visited, cashe, elephane);
        best = best.max(sub_result);
    }
    cashe.insert((node.to_string(), visited.clone(), time, elephane), best);
    best
}

fn parse1(s: &str, time: i32) -> i32 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flow: HashMap<&str, i32> = HashMap::new();

    let re1 = Regex::new(r"\d+").unwrap();
    let re2 = Regex::new(r"[[:upper:]]{2}").unwrap();
    
    for line in s.lines() {
        let r = re1.find(line).unwrap()
            .as_str()
            .parse::<i32>().unwrap();
        let valves = re2.find_iter(line)
            .map(|v| v.as_str())
            .collect::<Vec<&str>>();
        graph.insert(valves[0], valves[1..].to_vec());
        flow.insert(valves[0], r);
    }

    let mut visit = vec![];
    let mut cashe = HashMap::new();

    let res = solve("AA", time, &flow, &graph, &mut visit, &mut cashe, false);
    res
}

// Not woriking not memory 
fn parse2(s: &str, time: i32) -> i32 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flow: HashMap<&str, i32> = HashMap::new();

    let re1 = Regex::new(r"\d+").unwrap();
    let re2 = Regex::new(r"[[:upper:]]{2}").unwrap();
    
    for line in s.lines() {
        let r = re1.find(line).unwrap()
            .as_str()
            .parse::<i32>().unwrap();
        let valves = re2.find_iter(line)
            .map(|v| v.as_str())
            .collect::<Vec<&str>>();
        graph.insert(valves[0], valves[1..].to_vec());
        flow.insert(valves[0], r);
    }

    let mut visit = vec![];
    let mut cashe = HashMap::new();

    let res = solve("AA", time, &flow, &graph, &mut visit, &mut cashe, true);
    res
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
        assert_eq!(parse1(INPUT, 30), 1651);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT, 26), 1707);
    }
}
