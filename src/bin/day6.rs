use aoc_lib;
use std::iter;

const DAYS : u8 = 80;
const NEW_FISH_AGE : u8 = 8;
const BREED_AGE : u8 = 6;

fn main() {
    let data = aoc_lib::read_entries("day6.txt");
    let entries = data
        .split(',')
        .map(|v| v.parse::<u8>().unwrap());
    
    let mut fish : Vec<u8> = entries.collect::<Vec<u8>>();
    let mut to_add : u8;
    
    for day in 0..DAYS {
        to_add = 0;
        fish = fish.iter().map(|v| {
            if *v == 0 {to_add += 1; BREED_AGE} else {v - 1}
        }).collect::<Vec<u8>>();

        let fishes_to_add : Vec<u8> = iter::repeat(NEW_FISH_AGE).take(to_add.into()).collect();
        fish.extend(fishes_to_add);
    }

    println!("Number of lanternfish after 80 days: {}", fish.len());
}