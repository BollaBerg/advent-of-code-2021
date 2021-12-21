use aoc_lib;

#[cfg(test)]
mod day5_test;

#[derive(Debug, PartialEq)]
struct Vent {
    start : (u16, u16),
    end : (u16, u16),
}

impl Vent {
    fn from_string(string : &str) -> Vent {
        let mut tuples = string.split(" -> ").to_owned();
        let mut start_iterator = tuples.next().unwrap().split(",");
        let start = (
            start_iterator.next().unwrap().parse::<u16>().unwrap(),
            start_iterator.next().unwrap().parse::<u16>().unwrap()
        );
        let mut end_iterator = tuples.next().unwrap().split(",");
        let end = (
            end_iterator.next().unwrap().parse::<u16>().unwrap(),
            end_iterator.next().unwrap().parse::<u16>().unwrap()
        );

        Vent {start, end}
    }
}

fn main() {

}