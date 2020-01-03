use std::ops;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use std::fmt;

fn overlap(start_a: i32, end_a: i32, start_b: i32, end_b:i32) -> bool {
    return start_a <= end_b && start_b <= end_a;
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point{
        return Point {
            x: x,
            y: y
        };
    }

    fn manhattan_distance(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        return Point::new(self.x + other.x, self.y + other.y);
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct WireSegment {
    start: Point,
    end: Point
}

impl WireSegment {
    fn new(start: Point, end: Point) -> WireSegment {
        return WireSegment {
            start: start,
            end: end
        };
    }

    fn from(start: Point, direction: Direction, distance: i32) -> WireSegment {
        let delta = match direction {
            Direction::Up => Point::new(0, distance),
            Direction::Down => Point::new(0, -distance),
            Direction::Left => Point::new(-distance, 0),
            Direction::Right => Point::new(distance, 0)
        };

        let end = start.clone() + delta;
        return WireSegment::new(start, end);
    }

    fn intersects(&self, other: &WireSegment) -> Option<Point> {
        let self_max_x = cmp::max(self.start.x, self.end.x);
        let self_min_x = cmp::min(self.start.x, self.end.x);
        let self_max_y = cmp::max(self.start.y, self.end.y);
        let self_min_y = cmp::min(self.start.y, self.end.y);

        let other_max_x = cmp::max(other.start.x, other.end.x);
        let other_min_x = cmp::min(other.start.x, other.end.x);
        let other_max_y = cmp::max(other.start.y, other.end.y);
        let other_min_y = cmp::min(other.start.y, other.end.y);

        if overlap(self_min_x, self_max_x, other_min_x, other_max_x) &&
               overlap(self_min_y, self_max_y, other_min_y, other_max_y) {
            
            if self.start == other.start || self.start == other.end {
                return Some(self.start.clone());
            } else if self.end == other.start || self.end == other.end {
                return Some(self.end.clone());
            } else if self_min_x == self_max_x {
                return Some(Point::new(self_min_x, other_min_y));
            } else {
                return Some(Point::new(other_min_x, self_min_y));
            }
        }
        else {
            return None;
        }
    }
}

impl fmt::Display for WireSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

fn parse_wire_segment(start: Point, description: &str) -> WireSegment {
    let direction_char = description.chars().nth(0).unwrap();
    let direction = match direction_char {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Unknown direction: {}", direction_char)
    };

    return WireSegment::from(start, direction, description[1..].parse().unwrap());
}

fn parse_wire(line: &String) -> Result<Vec<WireSegment>, std::io::Error> {
    return Ok(line.split(",")
               .scan(Point::new(0, 0), |state, description| {
                   let segment = parse_wire_segment(state.clone(), &description);
                   *state = segment.end.clone();
                   return Some(segment);
                })
                .collect());
}

fn parse_file(file_name: &Path) -> Result<(Vec<WireSegment>, Vec<WireSegment>), std::io::Error> {
    let file = File::open(&file_name)?;

    let lines: Vec<io::Result<String>> = io::BufReader::new(file).lines().collect();

    let wire_1: Vec<WireSegment> = match &lines[0] {
        Err(why) => panic!("{}", why),
        Ok(line) => parse_wire(&line)?
    };
    
    let wire_2: Vec<WireSegment> = match &lines[1] {
        Err(why) => panic!("{}", why),
        Ok(line) => parse_wire(&line)?
    };

    return Ok((wire_1, wire_2));
}

fn find_intersections(segment: &WireSegment, other_wire: &Vec<WireSegment>) -> Vec<Point> {
    return other_wire.iter()
                     .filter_map(|other_segment| segment.intersects(&other_segment))
                     .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let (wire_1, wire_2) = match parse_file(&input_file) {
        Err(why) => panic!("{}", why),
        Ok(value) => value
    };
    
    let mut intersections: Vec<Point> = wire_1.iter()
                              .flat_map(|wire| find_intersections(&wire, &wire_2))
                              .collect();
    intersections.sort_by_cached_key(|a| a.manhattan_distance());

    println!("Intersections:");
    for intersection in intersections {
        println!("\t({}, {}) => {}", intersection.x, intersection.y, intersection.manhattan_distance());
    }
}