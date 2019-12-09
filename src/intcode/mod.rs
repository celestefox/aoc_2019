// Copyright (C) 2019 Glowpelt <glowpelt@chakat.space>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

pub mod instruction;
use instruction::{Instruction, ParameterMode};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
/// An interpreter error. These indicate unrecoverable failure of the IntCode interpreter.
pub enum InterpreterError {
    /// Tried to fetch from an invalid address in memory. This should only be
    /// possible with an address that points outside the memory space.
    EndOfMemory,
    /// Tried to execute an invalid (unknown) instruction. Holds the address at
    /// which the invalid instruction was encountered and the value of the instruction.
    InvalidInstruction {
        address: usize,
        code: i64, // Cannot use Instruction because not being a valid Instruction *is* the error
    },
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err: String = match self {
            InterpreterError::EndOfMemory => "Fetch past end of memory".to_string(),
            InterpreterError::InvalidInstruction { address, code } => {
                format!("Invalid instruction: {}({})", code, address)
            }
        };
        write!(f, "Interpreter error: {}", err)
    }
}

impl Error for InterpreterError {}

/// Fetch the value at an address in IntCode memory.
///
/// # Examples
///
/// ```
/// let memory = vec![1,2,3];
///
/// assert_eq!(fetch_address(memory, 0), Ok(1));
/// assert_eq!(fetch_address(memory, 3), Err(InterpreterError::EndOfMemory));
/// ```
fn fetch_address(memory: &Vec<i64>, addr: usize) -> Result<&i64, InterpreterError> {
    memory.get(addr).ok_or(InterpreterError::EndOfMemory)
}

fn fetch_parameter(
    memory: &Vec<i64>,
    inst_addr: usize,
    parameter: usize,
    mode: ParameterMode,
) -> Result<&i64, InterpreterError> {
    let dest = fetch_address(memory, inst_addr + parameter)?;
    match mode {
        ParameterMode::Immediate => Ok(dest),
        ParameterMode::Position => fetch_address(memory, *dest as usize),
    }
}

fn fetch_parameter_mut(
    memory: &mut Vec<i64>,
    inst_addr: usize,
    parameter: usize,
    mode: ParameterMode,
) -> Result<&mut i64, InterpreterError> {
    match mode {
        ParameterMode::Immediate => memory
            .get_mut(inst_addr + parameter)
            .ok_or(InterpreterError::EndOfMemory),
        ParameterMode::Position => {
            let dest = *fetch_address(memory, inst_addr + parameter)?;
            memory
                .get_mut(dest as usize)
                .ok_or(InterpreterError::EndOfMemory)
        }
    }
}

pub fn intcode_interpreter(memory: &mut Vec<i64>) -> Result<&mut Vec<i64>, InterpreterError> {
    let mut ip = 0;
    let mut halt = false;
    while !halt {
        //println!("New ip: {}", ip);
        let inst = fetch_address(memory, ip)?;
        let inst = Instruction::try_from(*inst as isize).map_err(|_| {
            InterpreterError::InvalidInstruction {
                address: ip,
                code: *inst,
            }
        })?;
        //println!("instruction at {} is: {}", ip, inst);
        match inst {
            Instruction::Add(m1, m2, m3) => {
                //println!("add: lhsaddr {} rhsaddr {} destaddr {}", memory[ip+1], memory[ip+2], memory[ip+3]);
                let lhs = *fetch_parameter(memory, ip, 1, m1)?;
                let rhs = *fetch_parameter(memory, ip, 2, m2)?;
                let dest = fetch_parameter_mut(memory, ip, 3, m3)?;
                *dest = lhs + rhs;
                ip += 4;
            }
            Instruction::Multiply(m1, m2, m3) => {
                let lhs = *fetch_parameter(memory, ip, 1, m1)?;
                let rhs = *fetch_parameter(memory, ip, 2, m2)?;
                let dest = fetch_parameter_mut(memory, ip, 3, m3)?;
                *dest = lhs * rhs;
                ip += 4;
            }
            Instruction::Halt => halt = true,
        }
    }
    // Halted safely, return the state of the memory, for now, to determine results with
    Ok(memory)
}

#[test]
fn test_fetch_address() {
    assert_eq!(
        fetch_address(&vec![], 0),
        Err(InterpreterError::EndOfMemory)
    );
    assert_eq!(fetch_address(&vec![1, 2, 3], 1), Ok(&2));
    assert_eq!(
        fetch_address(&vec![4, 5, 6], 3),
        Err(InterpreterError::EndOfMemory)
    );
}

#[test]
fn test_fetch_parameter() {
    assert_eq!(
        fetch_parameter(&vec![1, 0, 0, 0, 99], 0, 1, ParameterMode::Position),
        Ok(&1)
    );
    assert_eq!(
        fetch_parameter(&vec![1, 2, 0, 0, 99], 0, 1, ParameterMode::Position),
        Ok(&0)
    );
    assert_eq!(
        fetch_parameter(&vec![1, 2, 42, 0, 99], 0, 1, ParameterMode::Position),
        Ok(&42)
    );
    assert_eq!(
        fetch_parameter(&vec![1, 0, 0, 0, 99], 0, 4, ParameterMode::Position),
        Err(InterpreterError::EndOfMemory)
    );
    assert_eq!(
        fetch_parameter(&vec![1, 0, 0, 0, 99], 0, 4, ParameterMode::Immediate),
        Ok(&99)
    );
    assert_eq!(
        fetch_parameter(&vec![1, 0, 0, 0, 99], 4, 1, ParameterMode::Immediate),
        Err(InterpreterError::EndOfMemory)
    );
}
