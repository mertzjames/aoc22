use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cal_vals: Vec<i32> = Vec::new();
    let mut cals = 0;


    let file_path = &args[1];

    // 
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.eq("") {
                    cal_vals.push(cals);
                    // println!("Total Cals: {}", cals);
                    cals = 0;
                } else {
                    // let t_str = ip.replace("\n", "");
                    let t_val = ip.parse::<i32>().unwrap();
                    cals += t_val;
                }
                
            }
        }
        let max_val = cal_vals.iter().max();
        match max_val {
            Some(max) => println!("Max Value: {max}"),
            None            => println!("Empty Vector"),
        }

        cal_vals.sort();
        cal_vals.reverse();

        let mut top_cals = 0;
        for v in cal_vals[..3].iter() {
            top_cals += v;
        }
        println!("Top 3 Cal Val: {top_cals}");
    }

    println!("In file {}", file_path);

    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}