#[allow(unused_variables)]

const INPUT: &'static str = include_str!("../../input/06.txt");

fn main() {
    println!("day 06, output 1: {}", parse1(INPUT));
    println!("day 06, output 2: {}", parse2(INPUT));
}

fn is_start_packet_ok(s: &str) -> bool {
    let mut res = true;
    for (i,c) in s.chars().enumerate() {
        for (jj, cc) in s.chars().enumerate() {
            println!("{},{} => {},{}   {}",i,c,jj,cc,res);
            if c == cc && i != jj {
                res = false;
            }
        }        
    } 
    res
}


fn parse1(s: &str) -> usize {
    let mut start_of_packet = String::new();
    let mut buffer_iter = s.chars();
    (1..=3).for_each(|_| {
        start_of_packet.push(buffer_iter.next().unwrap());
    });
    let mut idx = 4;
    for (i,c) in buffer_iter.enumerate() {
        start_of_packet.push(c);
        println!("{} {} - {}",i ,c, start_of_packet);
        if is_start_packet_ok(&start_of_packet) {
            idx += i;
            break;
        } else {
            start_of_packet.remove(0);
        }
        //println!("{} {} - {}",i ,c, start_of_packet);
    }
    idx as usize
}

fn parse2(_s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 7);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1);
    }
}
