fn task1(counts : Vec<u32>, total_count : u32) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let factor : i32 = 2;
    
    for (i, count) in counts.iter().rev().enumerate() {
        if *count > total_count / 2 {
            gamma += factor.pow(i as u32);
        }
        else {
            epsilon += factor.pow(i as u32);
        }
    }

    println!("Value of gamma  : {}", gamma);
    println!("Value of epsilon: {}", epsilon);
    println!("Multiplied value: {}", gamma * epsilon);
}

fn main() {
    let entries = aoc_lib::read_entries("day3.txt");

    let total_count = entries.lines().count() as u32;
    let mut counts = vec![0; 12];

    for line in entries.lines() {
        for (i, value) in line.chars().enumerate() {
            counts[i] += value.to_digit(2).unwrap();
        }
    }

    // Task 1
    task1(counts, total_count);

    // Task 2

}