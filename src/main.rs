use std::{
    fs,
    io,
    io::BufRead,
};

use failure;

fn main() {
    match run() {
        Ok(()) => {},
        Err(ref err) => eprintln!("{:?}", err),
    }
}

fn run() -> Result<(), failure::Error> {
    let file = fs::File::open("input-1.txt")?;
    let input = io::BufReader::new(file);

    let mut freqs: Vec<i32> = vec![];
    for line in input.lines() {
        let line = line?;
        freqs.push(line.parse()?);
    }
    let freq_change: i32 = freqs.iter().sum();
    println!("{}", freq_change);

    Ok(())
}
