use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    // A,X = Rock
    // B,Y = Paper
    // C,Z = Scissors
    let vals = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3)
    ]);
    let beats = HashMap::from([
        ("A", "Z"),
        ("B", "X"),
        ("C", "Y")
    ]);
    let ties = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z")
    ]);
    

    let input_path = &args[1];

    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(ip) = line {
                let picks = ip.split_whitespace();
                if picks[0]
            }
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}