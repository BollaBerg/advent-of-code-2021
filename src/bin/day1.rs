use std::fs;

/// Count times the value of entries increases
fn count_depth_increase(entries: Vec<u16>) -> usize {
    entries.windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
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
    let depth_increases = count_depth_increase(entries_num);

    println!("Times depth increases: {}", depth_increases)

    // Task 2
}