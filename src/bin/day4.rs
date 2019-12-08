// Copyright (C) 2019 Glowpelt <glowpelt@chakat.space>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use aoc_2019::*;
use std::error::Error;

fn has_digit_pair(password: &String) -> bool {
    let mut prev = ' ';
    for char in password.chars() {
        if char == prev {
            return true;
        }
        prev = char;
    }
    false
}

fn has_unique_digit_pair(password: &String) -> bool {
    //println!("str: `{}`", password);
    let mut prev = ' ';
    let mut iter = " "
        .chars()
        .chain(password.chars())
        .zip(password.chars().chain(" ".chars()))
        .peekable();
    while let Some((c1, c2)) = iter.next() {
        /*println!(
            "p {} c1 {} c2 {} n {}",
            prev,
            c1,
            c2,
            iter.peek().unwrap_or(&('!', '!')).1
        );*/
        if c1 == c2 && c1 != prev {
            if let Some((_, next)) = iter.peek() {
                if c2 != *next {
                    return true;
                }
            } else {
                return true;
            }
        }
        prev = c1;
    }
    false
}

fn digits_always_increase(password: &String) -> bool {
    let mut prev = 0u8;
    for byte in password.bytes() {
        if byte < prev {
            return false;
        }
        prev = byte;
    }
    true
}

// Preconditions: 6 digits, within the puzzle input range
fn check_password_simple(password: usize) -> bool {
    let string: String = password.to_string();
    has_digit_pair(&string) && digits_always_increase(&string)
}

fn check_password_full(password: usize) -> bool {
    let string = password.to_string();
    has_unique_digit_pair(&string) && digits_always_increase(&string)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_simple_input("input/day4.txt")?;
    let mut iter = input.split('-').map(|n| n.parse::<usize>());
    let start = iter.next().unwrap()?;
    let end = iter.next().unwrap()?;
    let mut valid_passwords: Vec<usize> = Vec::new();
    for guess in start..=end {
        if check_password_simple(guess) {
            valid_passwords.push(guess);
        }
    }
    println!("Day 4 part 1: {} possible passwords", valid_passwords.len());
    let mut valid_passwords: Vec<usize> = Vec::new();
    for guess in start..=end {
        if check_password_full(guess) {
            valid_passwords.push(guess);
        }
    }
    println!("Day 4 part 2: {} possible passwords", valid_passwords.len());
    Ok(())
}

#[test]
fn test_check_password_simple() {
    assert!(check_password_simple(111111));
    assert!(!check_password_simple(223450));
    assert!(!check_password_simple(123789));
}

#[test]
fn test_check_password_full() {
    assert!(check_password_full(112233));
    assert!(!check_password_full(123444));
    assert!(check_password_full(111122));
}
