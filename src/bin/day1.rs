use std::fs;

fn count_depth_increase(entries: &str) -> usize {
    let line_nums : Vec<u16> = entries
        .lines().
        map(|x| x.parse::<u16>().unwrap())
        .collect();
    line_nums.windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
}

fn main() {
    let filename = "inputs/day1.txt";

    let entries = fs::read_to_string(filename)
        .expect("Something went wrong trying to read the file");
    
    let depth_increases = count_depth_increase(&entries);

    println!("Times depth increases: {}", depth_increases)
}