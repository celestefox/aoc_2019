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
use num_enum::TryFromPrimitive;
use std::fmt;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(i8)]
/// An IntCode instruction.
pub enum Instruction {
    /// opcode 1
    Add(ParameterMode, ParameterMode, ParameterMode),
    /// opcode 2
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    /// opcode 99
    Halt,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Instruction::Add(_,_,_) => "Add(1)",
                Instruction::Multiply(_,_,_) => "Multiply(2)",
                Instruction::Halt => "Halt(99)",
            }
        )
    }
}

impl TryFrom<isize> for Instruction {
    type Error = &'static str;
    fn try_from(value: isize)-> Result<Self, Self::Error> {
        let opcode = value % 100;
        let modes = value / 100;
        let m1 = modes % 10;
        let m2 = (modes % 100) / 10;
        let m3 = (modes % 1000) / 100;
        let m1 = ParameterMode::try_from(m1 as i8).map_err(|_| "Unknown parameter mode")?;
        let m2 = ParameterMode::try_from(m2 as i8).map_err(|_| "Unknown parameter mode")?;
        let m3 = ParameterMode::try_from(m3 as i8).map_err(|_| "Unknown parameter mode")?;
        match opcode {
            1 => Ok(Instruction::Add(m1,m2,m3)),
            2 => Ok(Instruction::Multiply(m1,m2,m3)),
            99 => Ok(Instruction::Halt),
            _ => Err("Unknown opcode")
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, TryFromPrimitive)]
#[repr(i8)]
/// An intcode parameter mode.
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl fmt::Display for ParameterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ParameterMode::Position => "Position(0)",
                ParameterMode::Immediate => "Immediate(1)",
            }
        )
    }
}
