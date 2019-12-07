use std::io::{self, BufReader, prelude::*};
use std::fs::File;

pub fn read_input(name: &str) -> io::Result<Vec<String>> {
    let file = File::open(name)?;
    let file = BufReader::new(file);
    Ok(file.lines().map(|x| x.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
