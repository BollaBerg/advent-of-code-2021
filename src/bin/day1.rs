use std::fs;

/// Count times the value of entries increases
fn count_depth_increase(entries: &Vec<u16>) -> usize {
    entries.windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

/// Count times the sum of 3-width sliding windows increases
fn count_sliding_depth_increase(entries: &Vec<u16>) -> usize {
    let windows = entries.windows(3)
        .map(|entry| entry.iter().sum())
        .collect();
    count_depth_increase(&windows)
}

fn main() {
    let filename = "inputs/day1.txt";

    let entries = fs::read_to_string(filename)
        .expect("Something went wrong trying to read the file");
    let entries_num : Vec<u16> = entries
        .lines().
        map(|x| x.parse::<u16>().unwrap())
        .collect();
    
    // Task 1
    let depth_increases = count_depth_increase(&entries_num);
    println!("Times depth increases: {}", depth_increases);

    // Task 2
    let sliding_depth_increases = 
        count_sliding_depth_increase(&entries_num);
    println!(
        "Times sliding depth increases: {}",sliding_depth_increases
    );
}