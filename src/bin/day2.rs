use aoc_2019::*;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
/// An interpreter error. These indicate unrecoverable failure of the IntCode interpreter.
enum InterpreterError {
    /// Tried to fetch from an invalid address in memory. This should only be
    /// possible with an address that points outside the memory space.
    EndOfMemory,
    /// Tried to execute an invalid (unknown) opcode. Holds the address at
    /// which the invalid opcode was encountered and the value of the opcode.
    InvalidOpcode {
        address: usize,
        code: i64, // Cannot use Opcode because not being a valid Opcode *is* the error
    },
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err: String = match self {
            InterpreterError::EndOfMemory => "Fetch past end of memory".to_string(),
            InterpreterError::InvalidOpcode { address, code } => {
                format!("Invalid opcode: {}({})", code, address)
            }
        };
        write!(f, "Interpreter error: {}", err)
    }
}

impl Error for InterpreterError {}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, TryFromPrimitive)]
#[repr(i64)]
/// An IntCode opcode.
enum Opcode {
    Add = 1, // 3 parameters
    Multiply = 2, // 3 parameters
    Halt = 99, // 0 parameters
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Opcode::Add => "Add(1)",
                Opcode::Multiply => "Multiply(2)",
                Opcode::Halt => "Halt(99)",
            }
        )
    }
}

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
fn fetch_address(memory: &Vec<i64>, inst_addr: usize) -> Result<i64, InterpreterError> {
    memory.get(inst_addr).ok_or(InterpreterError::EndOfMemory).map(|&x| x)
}

fn fetch_parameter(memory: &Vec<i64>, inst_addr: usize, parameter: usize) -> Result<i64, InterpreterError> {
    let dest = fetch_address(memory, inst_addr + parameter)? as usize;
    fetch_address(memory, dest)
}

fn intcode_interpreter(memory: &mut Vec<i64>) -> Result<&mut Vec<i64>, InterpreterError> {
    let mut ip = 0;
    let mut halt = false;
    while !halt {
        //println!("New ip: {}", ip);
        let inst = fetch_address(memory, ip)?;
        let inst = Opcode::try_from(inst).map_err(|_| InterpreterError::InvalidOpcode {
            address: ip,
            code: inst,
        })?;
        //println!("opcode at {} is: {}", ip, inst);
        match inst {
            Opcode::Add => {
                //println!("add: lhsaddr {} rhsaddr {} destaddr {}", memory[ip+1], memory[ip+2], memory[ip+3]);
                let lhs = fetch_parameter(&memory, ip, 1)?;
                let rhs = fetch_parameter(&memory, ip, 2)?;
                let dest = fetch_address(memory, ip + 3)? as usize; // Need the actual address
                let dest = memory.get_mut(dest).ok_or(InterpreterError::EndOfMemory)?;
                *dest = lhs + rhs;
                ip += 4;
            }
            Opcode::Multiply => {
                let lhs = fetch_parameter(&memory, ip, 1)?;
                let rhs = fetch_parameter(&memory, ip, 2)?;
                let dest = fetch_address(memory, ip + 3)? as usize; // Need the actual address
                let dest = memory.get_mut(dest).ok_or(InterpreterError::EndOfMemory)?;
                *dest = lhs * rhs;
                ip += 4;
            }
            Opcode::Halt => halt = true,
        }
    }
    // Halted safely, return the state of the memory, for now, to determine results with
    Ok(memory)
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_input("input/day2.txt")?;
    let input = lines.first().expect("no input");
    let input: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    let input_1 = &mut input.clone();
    input_1[1] = 12;
    input_1[2] = 2;
    let output_1 = intcode_interpreter(input_1)?;
    println!("Day 1 part 1 result: {:?}", output_1[0]);
    Ok(())
}

#[test]
fn test_simple_programs() {
    //assert_eq!(*intcode_interpreter(&mut vec![]).unwrap(), vec![]);
    assert_eq!(*intcode_interpreter(&mut vec![1,0,0,0,99]).unwrap(), vec![2,0,0,0,99]);
    assert_eq!(*intcode_interpreter(&mut vec![2,3,0,3,99]).unwrap(), vec![2,3,0,6,99]);
    assert_eq!(*intcode_interpreter(&mut vec![2,4,4,5,99,0]).unwrap(), vec![2,4,4,5,99,9801]);
    assert_eq!(*intcode_interpreter(&mut vec![1,1,1,4,99,5,6,0,99]).unwrap(), vec![30,1,1,4,2,5,6,0,99]);
}
