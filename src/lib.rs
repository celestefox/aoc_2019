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
