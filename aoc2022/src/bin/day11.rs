use nom::{sequence::{delimited, preceded}, character::{complete::multispace1,}};
#[allow(unused_variables)]

use std::collections::VecDeque;

use nom::{
    bytes::complete::tag,
    branch::alt,
    multi::separated_list1,
    *,
};

const INPUT: &'static str = include_str!("../../input/11.txt");

#[derive(Debug)]
struct Monkey {
    inspected: u32,
    items: VecDeque<u64>,
    opration: Operation,
    test_divisible: u32,
    if_true: u32,
    if_false: u32,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operation {
    Mul((Value,Value)),
    Add((Value,Value)),
}

fn value_parse(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64.map(|num| Value::Num(num)),
    ))(input)
}

fn operation_parse(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value1) = value_parse(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, value2) = value_parse(input)?;
    let res = match operator {
        "*" => Operation::Mul((value1,value2)),
        "+" => Operation::Add((value1,value2)),
        _ => panic!("Unknow operator!!"),
    };
    Ok((input, res))
}

fn monkey_parse(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), nom::character::complete::u32, tag(":"))(input)?;
    //let (input, _) = newline(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(tag("Starting items: "),
                        separated_list1(tag(", "), nom::character::complete::u64))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation_parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = preceded(tag("Test: divisible by "), nom::character::complete::u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, if_true) = preceded(tag("If true: throw to monkey "), nom::character::complete::u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, if_false) = preceded(tag("If false: throw to monkey "), nom::character::complete::u32)(input)?;

    Ok((
        input,
        Monkey {
            inspected: 0,
            items: VecDeque::from(items),
            opration: op,
            test_divisible: test,
            if_true: if_true,
            if_false: if_false,
        }
    ))
}

impl Value {
    fn value(self, item: u64) -> u64 {
        match &self {
            Value::Old => item,
            Value::Num(num) => *num,
        }
    } 
}

impl Monkey {
    fn inspect(&mut self, worry: bool, devisible_product: u64) -> u64 {
        self.inspected += 1;
        let item = self.items.pop_front().unwrap();
        let mut worry_level = match &self.opration {
            Operation::Mul((v1, v2)) => {
                //dbg!( v1.value(item), v2.value(item) );
                (v1.value(item) * v2.value(item)) % devisible_product
            },
            Operation::Add((v1, v2)) => {
                (v1.value(item) + v2.value(item)) % devisible_product
            }
        };
        if worry {
            worry_level / 3
        } else {
            worry_level
        }
    }

    fn test(&self, item: u64) -> u32 {
        if item % self.test_divisible as u64 == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

fn get_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, vecs) = separated_list1(tag("\n\n"),monkey_parse)(input)?;
    Ok((input, vecs))
}

fn main() {
    println!("day 11, output 1: {}", parse1(INPUT));
    println!("day 11, output 2: {}", parse2(INPUT));
}

fn parse1(s: &str) -> usize {
    let (_, mut monkeys) = get_monkeys(s).unwrap();
    //dbg!(&monkeys);
    let devisible_product = monkeys.iter()
        .map(|monkey| monkey.test_divisible as u64)
        .product::<u64>();

    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            for _item_idx in 0..monkeys[monkey_idx].items.len() {
                let monkey = monkeys.get_mut(monkey_idx).unwrap();
                let item = monkey.inspect(true, devisible_product);
                //println!("-> {item}");
                let send_to = monkey.test(item);
                monkeys.get_mut(send_to as usize).unwrap()
                    .items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspected);

    let res = monkeys.iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspected)
        .product::<u32>();
    res as usize
}

fn parse2(s: &str) -> usize {
    let (_, mut monkeys) = get_monkeys(s).unwrap();
    //dbg!(&monkeys);
    let devisible_product = monkeys.iter()
        .map(|monkey| monkey.test_divisible as u64)
        .product::<u64>();

    for _round in 0..10_000 {
        for monkey_idx in 0..monkeys.len() {
            for _item_idx in 0..monkeys[monkey_idx].items.len() {
                let monkey = monkeys.get_mut(monkey_idx).unwrap();
                let item = monkey.inspect(false, devisible_product);
                //println!("-> {item}");
                let send_to = monkey.test(item);
                monkeys.get_mut(send_to as usize).unwrap()
                    .items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.inspected);

    let res = monkeys.iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.inspected as u64)
        .product::<u64>();
    res as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 10605);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 2713310158);
    }
}
