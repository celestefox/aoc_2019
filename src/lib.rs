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

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn read_input(name: &str) -> io::Result<Vec<String>> {
    let file = File::open(name)?;
    let file = BufReader::new(file);
    Ok(file.lines().map(|x| x.unwrap()).collect())
}

pub fn read_simple_input(name: &str) -> io::Result<String> {
    read_input(name)?
        .first()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "No first line in input",
        ))
        .map(|l| l.clone())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
