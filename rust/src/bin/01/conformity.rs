use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    loop {
        // Read lines from stdin
        let mut line = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut line)?;
        let line = line.trim();

        let lines: i32 = line.parse::<i32>().unwrap();

        // Exit condition
        if lines == 0 {
            break;
        }

        let mut frosh: HashMap<Vec<i32>, i32> = HashMap::new();

        for _i in 0..lines {
            let mut line = String::new();
            stdin.read_line(&mut line)?;
            let line = line.trim();

            let mut this_line: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.trim().parse::<i32>().expect(&format!("Could not convert to integer: {x}")))
                .collect();

            this_line.sort();

            *frosh.entry(this_line).or_insert(0) += 1;
        }

        let &max = frosh.values().max().unwrap();
        let count: i32 = frosh.values().filter(|&x| x == &max).sum();
        println!("{}", count);
    }
    Ok(())
}