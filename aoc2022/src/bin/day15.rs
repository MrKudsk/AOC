use std::collections::{BTreeMap};
use itertools::{min, max, Itertools};
use nom::{
    //branch::alt,
    sequence::preceded,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list1},
    sequence::{separated_pair},
    *,
};


const INPUT: &'static str = include_str!("../../input/15.txt");

fn main() {
    println!("day 15, output 1: {}", parse1(INPUT, 2_000_000));
    println!("day 15, output 2: {}", parse2(INPUT, 4_000_000));
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
struct Sensor {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
}

fn get_point(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), nom::character::complete::i32), 
        tag(", "),
        preceded(tag("y="), nom::character::complete::i32),
        )(input)
}

fn get_list(input: &str) -> IResult<&str, BTreeMap<Sensor, Beacon>> {
    let (input, list) = separated_list1(
        newline, 
        separated_pair(
            preceded(tag("Sensor at "), get_point.map(|(x, y)| Sensor {x, y})),
            tag(": "),
            preceded(tag("closest beacon is at "), get_point.map(|(x, y)| Beacon {x,y}))
        )
        )(input)?;
    Ok((input, list.into_iter().collect::<BTreeMap<Sensor, Beacon>>()))
}

fn parse1(s: &str, y: i32) -> usize {
    let (_, sensors) = get_list(s).unwrap();
    //dbg!(&sensors, y);
    //
    // Calulate the max distance from sensor to it's beacon
    let distancen = sensors.iter()
        .map(|(sensor, beacon)| {
            ( sensor, (beacon.x - sensor.x).abs() + (beacon.y - sensor.y).abs())
        })
        .collect::<BTreeMap<&Sensor, i32>>();
    //dbg!(&distancen);
    //
    // First find the sensor in range of the line(y), then finde the max distance on the line
    // finde the range of it sensor on the line, and then count them without the Beacon on the
    // line.

    let result = distancen
        .iter()
        .filter(|(sensor, distancen)| {
            let sensor_y_range = (sensor.y - **distancen)..(sensor.y + **distancen);
            sensor_y_range.contains(&y)
        })
        .flat_map(|(sensor, sensor_distance)| {
            let distance_to_line = (sensor.y - y).abs();
            let distance_on_line = sensor_distance - distance_to_line;
            (sensor.x - distance_on_line)..=(sensor.x + distance_on_line)
        })
        .unique()
        .filter(|x| {
            !sensors.values().contains(&Beacon{ x: *x, y: y,})
        })
        .count();
    //dbg!(&result);
    result
}

fn get_x_range(distance: &BTreeMap<&Sensor, i32>, y: i32, limit: i32) -> Vec<i32> {
    distance
        .iter()
        .filter(|(sensor, distancen)| {
            let sensor_y_range = (sensor.y - **distancen)..(sensor.y + **distancen);
            sensor_y_range.contains(&y)
        })
        .flat_map(|(sensor, sensor_distance)| {
            let distance_to_line = (sensor.y - y).abs();
            let distance_on_line = sensor_distance - distance_to_line;
            ((sensor.x - distance_on_line).max(0))..=((sensor.x + distance_on_line).min(limit))
        })
        .unique()
        .collect::<Vec<i32>>()
}

fn parse2(s: &str, limit: i32) -> i32 {
    let (_, sensors) = get_list(s).unwrap();
    //dbg!(&sensors, y);
    //
    // Calulate the max distance from sensor to it's beacon
    let distance = sensors.iter()
        .map(|(sensor, beacon)| {
            ( sensor, (beacon.x - sensor.x).abs() + (beacon.y - sensor.y).abs())
        })
        .collect::<BTreeMap<&Sensor, i32>>();

    let min_limt = distance.iter()
        .map(|(sensor, distance)| {
            sensor.y - distance
        })
        .min().unwrap().max(0);
    println!("Min limt is {}", &min_limt);

    println!("Calulated Sensor {}", distance.len());
    //let mut map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    
    let mut result = (0,0);
    for y in min_limt..=limit {
        let vec = get_x_range(&distance, y, limit);
        if y % 100_000 == 0 {
            println!("-> {y}");
        }
        if (max(&vec).unwrap() - min(&vec).unwrap()) != (vec.len() as i32 - 1) {
            let mut x = *min(&vec).unwrap();
            while x < *max(&vec).unwrap() {
                if !vec.contains(&x) {
                    break;
                }
                x += 1;
            }
            println!("X = {x}");
            result = (x, y);
            break;
        }
        //map.insert(y, vec);
    }

    //println!("Limit {}{}", pos.0, pos.1);
    //let result = (1,1);
    /*map
        .iter()
        .map(|(y, vec)| {
            if (max(vec).unwrap() - min(vec).unwrap()) != (vec.len() as i32 - 1) {
                let mut x = *min(vec).unwrap();
                while x < *max(vec).unwrap() {
                    if !vec.contains(&x) {
                        break;
                    }
                    x += 1;
                }
                Some((x,y))
            } else {
                None
            }
        })
        .filter(|val| val.is_some())
        .collect::<Vec<_>>()[0].unwrap();
    */
    dbg!(&result);
    (result.0 * 4_000_000) + result.1
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT, 10), 26);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT, 20), 56000011);
    }
}
