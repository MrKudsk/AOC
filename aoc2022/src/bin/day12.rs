use petgraph::prelude::*;
use petgraph::{algo::dijkstra};

use itertools::{Itertools, min};

#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/12.txt");

fn main() {
    println!("day 12, output 1: {}", parse1(INPUT));
    println!("day 12, output 2: {}", parse2(INPUT));
}

fn find_pos(maze: &Vec<Vec<char>>, ch: char) -> (i32, i32) {
    let list_of_pos = maze.iter()
        .map(|row| row.iter()
            .position(|&cell| cell==ch))
        .collect::<Vec<Option<usize>>>();
    let pos = list_of_pos.iter()
        .enumerate()
        .filter(|(_, &l)| l >= Some(0))
        .map(|(id, num)| (id, num.unwrap()))
        .collect::<Vec<(usize,usize)>>()[0];
    (pos.1 as i32, pos.0 as i32)
/*
    maze.iter()
    .enumerate()
    .flat_map(|(i, v)| v.iter().enumerate().zip(std::iter::repeat(i)))
    .find_map(|((x, &c), y)| {
            if c == ch {
                Some((x as i32, y as i32))
            } else {
                None
            }
    }).unwrap()*/
}

/*
#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    position: Pos,
    visited: bool,
    globalgoal: f32,
    localgoal: f32,
    neighbours: Option<Vec<Pos>>,
    parent: Option<&'a Node<'a>>,
}

fn distance(a: &Pos, b: &Pos) -> f32 {
    let dx = a.x as f32 - b.x as f32;
    let dy = a.y as f32 - b.y as f32;
    dbg!(&dx,&dy);
    ((dx * dx) + (dy * dy)).sqrt()
}

fn solve_astar(maze: &Vec<Vec<char>>, start: Pos, end: Pos) -> usize {
    let ymax = maze.len();
    let xmax = maze[0].len();
    let mut nodes: HashMap<(usize,usize), Node> = HashMap::new();
    for y in 0..ymax {
        for x in 0..xmax {
            let mut height = maze[y][x] as u8;
            if height < 97 {
                if height == 'S' as u8 {
                    height = 'a' as u8;
                } else {
                    height = 'z' as u8;
                }
            }
            // find neighbours
            let mut neighbours: Vec<Pos> = Vec::new();
            // look up
            if y > 0 {
                let elevation:i8 = maze[y-1][x] as i8 - height as i8;
                if elevation >= -1 && elevation <= 1 {
                    neighbours.push(Pos{x: x, y: y-1});
                }
            }
            // look down
            if y < ymax-1 {
                let elevation:i8 = maze[y+1][x] as i8 - height as i8;
                if elevation >= -1 && elevation <= 1 {
                    neighbours.push(Pos{x: x, y: y+1});
                }
            }
            // look left
            if x > 0 {
                let elevation: i8 = maze[y][x-1] as i8 - height as i8;
                if elevation >= -1 && elevation <= 1 {
                    neighbours.push(Pos{x: x-1, y: y});
                }
            }
            // look right
            if x < xmax-1  {
                let elevation: i8 = maze[y][x+1] as i8 - height as i8;
                if elevation >= -1 && elevation <= 1 {
                    neighbours.push(Pos{x: x+1, y: y});
                }
            }
            let node = Node{
                position: Pos{x,y},
                visited: false,
                globalgoal: 9_999_999_999.0,
                localgoal: 9_999_999_999.0,
                neighbours: Some(neighbours),
                parent: None,
            };
            nodes.insert((x,y), node);
        }
    }
    let mut curretNode = nodes.get_mut(&(start.x, start.y)).unwrap();
    curretNode.localgoal = 0.0;
    curretNode.globalgoal = distance(&curretNode.position, &end);

    let mut not_test_nodes = vec![curretNode];

    while !not_test_nodes.is_empty() {
        not_test_nodes.sort_by_key(|a| a.globalgoal as i32);
        while !not_test_nodes.is_empty() && not_test_nodes[0].visited {
            let _ = not_test_nodes.remove(0);
        }

        if not_test_nodes.is_empty() {
            break;
        }

        let pos = not_test_nodes[0].position;
        curretNode = nodes.get_mut(&(pos.x, pos.y)).unwrap();
        curretNode.visited = true;

        for pos in curretNode.neighbours.unwrap() {
            let mut node = nodes.get_mut(&(pos.x, pos.y)).unwrap();
            if node.visited {
                continue;
            }
            let possiblylowergoal = curretNode.localgoal + distance(&curretNode.position, &node.position);
            if possiblylowergoal < node.localgoal {
                node.parent = Some(&curretNode);
                node.localgoal = possiblylowergoal;
                node.globalgoal = node.localgoal + distance(&node.position, &end);
            }
        }
    }
    dbg!(nodes.get(&(0,0)));
    1
}
*/
fn parse1(s: &str) -> usize {
    let mut maze = s.lines()
        .map(|l| {
            l.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    //dbg!(&maze);
    let start_pos = find_pos(&maze, 'S');
    let end_pos = find_pos(&maze, 'E');
    //dbg!(&start_pos, &end_pos);
    //let res = solve_astar(&maze, start_pos, end_pos);
    //set S and E to is height
    maze[start_pos.1 as usize][start_pos.0 as usize] = 'a'; // Start position
    maze[end_pos.1   as usize][end_pos.0   as usize] = 'z'; // Start position
    //dbg!(&maze[end_pos.1 as usize].iter().join(""));

    let edges = (0i32..(maze.len() as i32)) 
        .cartesian_product(0i32..(maze[0].len() as i32))
        .flat_map(|(y, x)| {
            let neighbors = vec![
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
            ];
            let current_node = (x, y);
            neighbors.iter()
                .filter_map(|cell| {
                    maze.get(cell.1 as usize)
                        .and_then(|vec| {
                            vec.get(cell.0 as usize)
                        })
                        .and_then(|existing_cell| {
                            // now we have found all existing neighbors
                            //
                            // test for height
                            let current_node_height = maze[y as usize][x as usize];

                            if //current_node_height as u8 == *existing_cell as u8 || 
                                current_node_height as u8 + 1 >= *existing_cell as u8 { 
                                Some((
                                    (
                                        current_node.0,
                                        current_node.1,
                                        current_node_height,
                                    ),
                                    (
                                        cell.0,
                                        cell.1,
                                        *existing_cell,
                                    )
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32,i32, char),(i32,i32, char))>>();
    //dbg!(&edges);

    let graph = DiGraphMap::<(i32, i32, char),()>::from_edges(&edges,);
    let res = dijkstra(
        &graph,
        (start_pos.0, start_pos.1, 'a'),
        Some((end_pos.0, end_pos.1, 'z')),
        |_| 1,
    );
    //dbg!(&res);
    res[&(end_pos.0, end_pos.1, 'z')]
}

fn parse2(s: &str) -> usize {
    let mut maze = s.lines()
        .map(|l| {
            l.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    //dbg!(&maze);
    let start_pos = find_pos(&maze, 'S');
    let end_pos = find_pos(&maze, 'E');
    //dbg!(&start_pos, &end_pos);
    //let res = solve_astar(&maze, start_pos, end_pos);
    //set S and E to is height
    maze[start_pos.1 as usize][start_pos.0 as usize] = 'a'; // Start position
    maze[end_pos.1   as usize][end_pos.0   as usize] = 'z'; // Start position
    //dbg!(&maze[end_pos.1 as usize].iter().join(""));

    let edges = (0i32..(maze.len() as i32)) 
        .cartesian_product(0i32..(maze[0].len() as i32))
        .flat_map(|(y, x)| {
            let neighbors = vec![
                (x - 1, y),
                (x + 1, y),
                (x, y - 1),
                (x, y + 1),
            ];
            let current_node = (x, y);
            neighbors.iter()
                .filter_map(|cell| {
                    maze.get(cell.1 as usize)
                        .and_then(|vec| {
                            vec.get(cell.0 as usize)
                        })
                        .and_then(|existing_cell| {
                            // now we have found all existing neighbors
                            //
                            // test for height
                            let current_node_height = maze[y as usize][x as usize];

                            if //current_node_height as u8 == *existing_cell as u8 || 
                                current_node_height as u8 + 1 >= *existing_cell as u8 { 
                                Some((
                                    (
                                        current_node.0,
                                        current_node.1,
                                        current_node_height,
                                    ),
                                    (
                                        cell.0,
                                        cell.1,
                                        *existing_cell,
                                    )
                                ))
                            } else {
                                None
                            }
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<((i32,i32, char),(i32,i32, char))>>();
    //dbg!(&edges);

    let graph = DiGraphMap::<(i32, i32, char),()>::from_edges(&edges,);
    let posiblystart = maze.iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter().enumerate().zip(std::iter::repeat(i))
        })
        .filter_map(|((x, &c), y)| {
            if c == 'a' {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let result = posiblystart.iter()
        .map(|(x, y)| {
           // println!("{x}, {y}");
           let res = dijkstra(
                &graph,
                (*x, *y, 'a'),
                Some((end_pos.0, end_pos.1, 'z')),
                |_| 1,
            );
            //dbg!(res.get(&(end_pos.0, end_pos.1, 'z')));
            if res.get(&(end_pos.0, end_pos.1, 'z')) != None {
               res[&(end_pos.0, end_pos.1, 'z')] 
            } else {
                10_000_000
            }
        })
        .collect::<Vec<i32>>();

    dbg!(min(&result));
    min(result).unwrap() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 31);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 29);
    }
}
