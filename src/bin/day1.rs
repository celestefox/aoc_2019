use aoc_2019::*;
use std::error::Error;

fn simple_fuel_needed(module_masses: Vec<i64>) -> i64 {
    module_masses.iter().map(|mass| mass/3-2).sum()
}

fn mass_with_fuel(mass: &i64) -> i64 {
    // TODO: Feel like there should be a more elegant solution to this...
    let mut total = 0;
    let mut fuel = mass/3-2;
    while fuel > 0 {
        total = total + fuel;
        fuel = fuel/3-2;
    }
    total
}

fn realistic_fuel_needed(module_masses: Vec<i64>) -> i64 {
    module_masses.iter().map(mass_with_fuel).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines: Vec<i64> = read_input("input/day1.txt")?
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();
    println!("part 1 total: {}", simple_fuel_needed(lines.clone()));
    println!("part 2 total: {}", realistic_fuel_needed(lines));
    Ok(())
}

#[test]
fn test_simple_fuel() {
    assert_eq!(simple_fuel_needed(vec![12]), 2);
    assert_eq!(simple_fuel_needed(vec![14]), 2);
    assert_eq!(simple_fuel_needed(vec![1969]), 654);
    assert_eq!(simple_fuel_needed(vec![100756]), 33583);
}

#[test]
fn test_full_fuel() {
    assert_eq!(mass_with_fuel(&14), 2);
    assert_eq!(mass_with_fuel(&1969), 966);
    assert_eq!(mass_with_fuel(&100756), 50346);
}