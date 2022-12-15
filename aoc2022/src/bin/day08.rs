#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/08.txt");

fn main() {
    println!("day 08, output 1: {}", parse1(INPUT));
    println!("day 08, output 2: {}", parse2(INPUT));
}

/*
fn is_visible(trees: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let x_max = trees.len();
    let x_mid = trees.len()/2;
    let y_max = trees[0].len();
    let y_mid = y_max/2;
    let height = trees[x][y];
    let visible = false;
    if x <= x_mid && y <= y_mid {
        for cx in (0..x) {
             
        }
    }
}
*/

fn parse1(s: &str) -> usize {
    let trees = s.lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10).unwrap()
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    //let edges = trees.len() * 2 + trees[0].len() * 2 - 4;
    //println!("{}", edges);

    let mut visible_trees: Vec<Vec<bool>> = trees.iter()
        .enumerate()
        .map(|(ix, tree_line)| {
            let tree_line_max = tree_line.len() - 1;
            tree_line.iter()
                .enumerate()
                .map(|(iy, _)| {
                    if ix == 0 || ix == trees.len() - 1 
                    || iy == 0 || iy == tree_line_max {
                        true
                    } else {
                        false
                    } 
                }).collect()
        })
        .collect();
    // Control for Ys
    for x in 0..trees.len() {
        let mut current_tree_size = 0;
        for y in 0..trees[0].len() {
            if y == 0 {
                current_tree_size = trees[x][y];
            } else if trees[x][y] > current_tree_size {
                current_tree_size = trees[x][y];
                visible_trees[x][y] = true;
            }
        }
    }
    for x in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for y in (0..trees[0].len()).rev() {
            if y == 0 {
                current_tree_size = trees[x][y];
            } else if trees[x][y] > current_tree_size {
                current_tree_size = trees[x][y];
                visible_trees[x][y] = true;
            }
        }
    }
    // Control for Xs
    for y in 0..trees[0].len() {
        let mut current_tree_size = 0;
        for x in 0..trees.len() {
            if y == 0 {
                current_tree_size = trees[x][y];
            } else if trees[x][y] > current_tree_size {
                current_tree_size = trees[x][y];
                visible_trees[x][y] = true;
            }
        }
    }
    for y in (0..trees[0].len()).rev() {
        let mut current_tree_size = 0;
        for x in (0..trees.len()).rev() {
            if y == 0 {
                current_tree_size = trees[x][y];
            } else if trees[x][y] > current_tree_size {
                current_tree_size = trees[x][y];
                visible_trees[x][y] = true;
            }
        }
    }
    //dbg!(&visible_trees);
    visible_trees.iter()
        .flatten()
        .filter(|&&v| v)
        .count()
}

fn parse2(s: &str) -> usize {
    let trees = s.lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_digit(10).unwrap()
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut high_score = 0;

    for (yi, tree_line) in trees.iter().enumerate() {
        for (xi, tree_height) in tree_line.iter().enumerate() {
            let mut score = [0,0,0,0];

            // to left
            for xp in (0..xi).rev() {
                if trees[yi][xp] < *tree_height {
                    score[0] += 1;
                } else {
                    score[0] += 1;
                    break;
                }
            }

            // to right
            for xp in (xi +1 )..tree_line.len() {
                if trees[yi][xp] < *tree_height {
                    score[1] += 1;
                } else {
                    score[1] += 1;
                    break;
                }
            }

            // to up 
            for yp in (0..yi).rev() {
                if trees[yp][xi] < *tree_height {
                    score[2] += 1;
                } else {
                    score[2] += 1;
                    break;
                }
            }
    
            // to down 
            for yp in (yi+1)..trees.len() {
                if trees[yp][xi] < *tree_height {
                    score[3] += 1;
                } else {
                    score[3] += 1;
                    break;
                }
            }
            let scenic_score: u32 = score.iter().product();

            if scenic_score > high_score {
                high_score = scenic_score;
            }
        }
    }
    
    high_score as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 21);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 8);
    }
}
