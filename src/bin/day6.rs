use aoc_lib;

const DAYS : usize = 80;
const NEW_FISH_AGE : usize = 8;
const BREED_AGE : usize = 6;

fn main() {
    let data = aoc_lib::read_entries("day6.txt");
    let entries = data
        .split(',')
        .map(|v| v.parse::<usize>().unwrap());
    
    let mut fish = [0; NEW_FISH_AGE + 1];
    for fish_days in entries {
        fish[fish_days] += 1;
    }
    
    for _ in 0..DAYS {
        let birthing_fish = fish[0];
        for i in 0 .. fish.len() - 1 {
            fish[i] = fish[i + 1];
        }
        fish[NEW_FISH_AGE] = birthing_fish;
        fish[BREED_AGE] += birthing_fish;
    }

    let number_of_fish : u32 = fish.iter().sum();
    println!("Number of lanternfish after 80 days: {}", number_of_fish);
}