use aoc_lib;

const NEW_FISH_AGE : usize = 8;
const BREED_AGE : usize = 6;

fn breed_fish(starting_fish : [u64; 9], days : usize) -> u64 {
    let mut fish = starting_fish.clone();

    for _ in 0..days {
        let birthing_fish = fish[0];
        for i in 0 .. fish.len() - 1 {
            fish[i] = fish[i + 1];
        }
        fish[NEW_FISH_AGE] = birthing_fish;
        fish[BREED_AGE] += birthing_fish;
    }

    fish.iter().sum()
}

fn main() {
    let data = aoc_lib::read_entries("day6.txt");
    let entries = data
        .split(',')
        .map(|v| v.parse::<usize>().unwrap());
    
    let mut fish = [0; NEW_FISH_AGE + 1];
    for fish_days in entries {
        fish[fish_days] += 1;
    }
    
    let number_of_fish_80 = breed_fish(fish, 80);
    println!("Number of lanternfish after 80 days: {}", number_of_fish_80);

    let number_of_fish_256 = breed_fish(fish, 256);
    println!("Number of lanternfish after 256 days: {}", number_of_fish_256);
}