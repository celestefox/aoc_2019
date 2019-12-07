use aoc_2019::*;
use std::error::Error;

fn fuel_needed(module_masses: Vec<i64>) -> i64 {
    module_masses.iter().map(|mass| mass/3-2).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<i64> = read_input("input/day1.txt")?
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();
    println!("part 1 total: {}", fuel_needed(lines));
    Ok(())
}

#[test]
fn fuel_needed_demos() {
    assert_eq!(fuel_needed(vec![12]), 2);
    assert_eq!(fuel_needed(vec![14]), 2);
    assert_eq!(fuel_needed(vec![1969]), 654);
    assert_eq!(fuel_needed(vec![100756]), 33583);
}