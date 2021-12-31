use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<Vec<Vec<char>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let x = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|line| line.chars().collect())
        .collect();

    Ok(x)
}