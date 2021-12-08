use aoc_lib;

fn main() {
    let data = aoc_lib::read_entries("day4.txt");
    let entries : Vec<u16> = data
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|entry| entry.parse::<u16>().unwrap())
        .collect();
    
    
}