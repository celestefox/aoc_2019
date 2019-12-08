use aoc_2019::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Represents a segment of a wire, with direction and length.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
enum WireSegment {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

/// Simple error type to represent the case of no valid direction when parsing
/// a segment.
#[derive(Debug)]
struct NoDirection {}

impl fmt::Display for NoDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No direction in segment")
    }
}

impl Error for NoDirection {}

/// Parses a single segment from a string like "U12" into WireSegment::Up(12)
fn parse_segment(segment: &str) -> Result<WireSegment, Box<dyn Error>> {
    let mut iter = segment.chars();
    let dir = iter.next().ok_or(NoDirection {})?;
    // Relying on that the line above advances the iterator to more easily
    // get the part of the string we need - if you use skip, you can't use
    // as_str anymore. We could also just get dir, and _also_ do
    // `iter = segment.chars().next()`, but... yeah, a bit of a waste.
    match dir {
        'U' => Ok(WireSegment::Up(iter.as_str().parse()?)),
        'D' => Ok(WireSegment::Down(iter.as_str().parse()?)),
        'R' => Ok(WireSegment::Right(iter.as_str().parse()?)),
        'L' => Ok(WireSegment::Left(iter.as_str().parse()?)),
        _ => Err(Box::new(NoDirection {})),
    }
}

/// Type to represent a point. Probably could have gotten away with just
/// (isize, isize) since I'm not doing anything more complicated than `add`,
/// but having that as a function _is_ useful...
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    /// Returns a new point with specified x and y coordinates.
    pub fn new(x: isize, y: isize) -> Self {
        Point { x: x, y: y }
    }

    /// Adds two points together. Not sure what this is called mathematically,
    /// but it's just a new point with each point being the sum of the two
    /// added points. Useful for "moving" a point by any amount with just one
    /// operation... which is exactly what I use it for.
    pub fn add(&self, other: &Point) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type Grid = Box<HashMap<Point, HashMap<usize, usize>>>;

/// Takes a set of wire segments and returns the grid
/// of wires the segments create.
fn process_wires(wires: Vec<Vec<WireSegment>>) -> Grid {
    let mut result = Box::new(HashMap::new());
    for (index, wire) in wires.iter().enumerate() {
        let mut total_len = 0;
        let mut loc = Point::new(0, 0);
        for segment in wire {
            let (increment, len) = match segment {
                WireSegment::Up(len) => (Point::new(0, 1), len),
                WireSegment::Down(len) => (Point::new(0, -1), len),
                WireSegment::Left(len) => (Point::new(-1, 0), len),
                WireSegment::Right(len) => (Point::new(1, 0), len),
            };

            for _ in 0..*len {
                loc = loc.add(&increment);
                total_len += 1;
                result
                    .entry(loc)
                    .or_insert_with(HashMap::new)
                    .insert(index, total_len.clone());
            }
        }
    }
    result
}

fn manhattan_distance_origin(point: &Point) -> isize {
    point.x.abs() + point.y.abs()
}

fn closest_manhattan_intersection(grid: &Grid, num_wires: usize) -> Option<(&Point, isize)> {
    //for line in visualize_grid(grid, 9) {
    //    println!("{}", line);
    //}
    let mut result = None;
    'grid: for (coord, coord_wires) in grid.iter() {
        for wire in 0..num_wires {
            if !coord_wires.contains_key(&wire) {
                continue 'grid;
            }
        }
        //println!("Intersection found: {:?} {:?}", coord, coord_wires);
        let new_distance = manhattan_distance_origin(coord);
        if let Some((_, old_distance)) = result {
            if new_distance < old_distance {
                result = Some((coord, new_distance))
            }
        } else {
            result = Some((coord, new_distance))
        }
    }
    result
}
// usize instead because we store a usize wire distance, instead of manhattan now
fn closest_latency_intersection(grid: &Grid, num_wires: usize) -> Option<(&Point, usize)> {
    let mut result = None;
    'grid: for (coord, coord_wires) in grid.iter() {
        for wire in 0..num_wires {
            if !coord_wires.contains_key(&wire) {
                continue 'grid;
            }
        }
        //println!("Intersection found: {:?} {:?}", coord, coord_wires);
        let new_distance = coord_wires.values().sum();
        if let Some((_, old_distance)) = result {
            if new_distance < old_distance {
                result = Some((coord, new_distance))
            }
        } else {
            result = Some((coord, new_distance))
        }
    }
    result
}

fn line_to_segments(line: &String) -> Vec<WireSegment> {
    line.split(',')
        .map(|seg| parse_segment(seg).unwrap())
        .collect()
}

// Returns a representation of the grid about the center. Not useful for the
// actual problem, but useful for visualizing the examples.
#[allow(dead_code)]
fn visualize_grid(grid: &Grid, radius: isize) -> Vec<String> {
    let side = 2 * radius + 1;
    let mut res = Vec::with_capacity(side as usize);
    for line in -radius..radius {
        let mut string: String = String::from("");
        for col in -radius..radius {
            let key = Point::new(line * -1, col); // lines are top to bottom, so this flips data
            string.push(if grid.contains_key(&key) {
                let set = grid.get(&key).unwrap();
                if set.contains_key(&0) && set.contains_key(&1) {
                    'X' // Collision
                } else if set.contains_key(&0) {
                    '0' // Only 0
                } else {
                    '1' // Else it must be 1
                }
            } else if key == Point::new(0, 0) {
                '#' // To visualize the origin
            } else {
                '.' // Nothing there
            });
        }
        res.push(string);
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let wires = read_input("input/day3.txt")?;
    let wires: Vec<Vec<WireSegment>> = wires.iter().map(line_to_segments).collect();
    // Map<coordinates, Set<wires>>
    let grid: Grid = process_wires(wires.clone());
    println!("Grid has {} entries", grid.len());
    //println!("{:?}", grid);
    let (intersection, distance) = closest_manhattan_intersection(&grid, wires.len()).unwrap();
    println!(
        "Day 3 part 1: intersection at {:?}, dist {}",
        intersection, distance
    );
    let (intersection, distance) = closest_latency_intersection(&grid, wires.len()).unwrap();
    println!(
        "Day 3 part 2: intersection at {:?}, dist {}",
        intersection, distance
    );
    Ok(())
}

#[test]
fn test_segment_parsing() {
    assert_eq!(
        line_to_segments(&"R8,U5,L5,D3".to_string()),
        vec![
            WireSegment::Right(8),
            WireSegment::Up(5),
            WireSegment::Left(5),
            WireSegment::Down(3)
        ]
    );
}

#[test]
fn test_manhattan_intersection() {
    assert_eq!(
        closest_manhattan_intersection(
            &process_wires(vec![
                line_to_segments(&"R8,U5,L5,D3".to_string()),
                line_to_segments(&"U7,R6,D4,L4".to_string())
            ]),
            2
        ),
        Some((&Point::new(3, 3), 6))
    );
}
