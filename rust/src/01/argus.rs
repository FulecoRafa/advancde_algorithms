use std::cmp::Ordering;
use std::io;
use std::str::FromStr;
use std::collections::BinaryHeap;
use std::num::ParseIntError;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Query {
    num: i32,
    period: i32,
    next_time: i32,
}

impl Query {
    fn new(num: i32, period: i32) -> Query {
        Query {
            num,
            period,
            next_time: period,
        }
    }

    fn next_time(&mut self) {
        self.next_time += self.period;
    }
}

impl FromStr for Query {
    type Err = ParseIntError;

    /// Parse a query from a string.
    /// String format: "Register <id> <period">
    /// Example: "Register 1 5"
    ///
    /// ```
    /// use std::str::FromStr;
    /// use std::io;
    ///
    /// let query = "Register 1 5".parse::<Query>().unwrap();
    /// assert_eq!(query.num, 1);
    /// assert_eq!(query.period, 5);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        split.next(); // Skip "Register"
        let num = split.next().unwrap_or("No number found").parse::<i32>()?;
        let period = split.next().unwrap_or("No period found").parse::<i32>()?;
        Ok(Query::new(num, period))
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.next_time == other.next_time {
            return other.num.cmp(&self.num);
        }
        other.next_time.cmp(&self.next_time)
    }
}

fn main() -> io::Result<()> {
    let mut queries = BinaryHeap::new();
    let mut line = String::new();
    let stdin = io::stdin();
    loop {
        line.clear();
        stdin.read_line(&mut line)?;
        let line = line.trim();

        // println!("Read line: `{}` | starts with #: {}", line, line.starts_with("#"));

        if line.starts_with('#') {
            break;
        }

        let q = Query::from_str(line).unwrap();

        queries.push(q);
    }

    line.clear();
    stdin.read_line(&mut line)?;
    let n_queries = line.trim().parse::<i32>().unwrap();

    for _i in 0..n_queries {
        let mut q = queries.pop().unwrap();
        println!("{}", q.num);
        q.next_time();
        queries.push(q);
    }

    Ok(())
}