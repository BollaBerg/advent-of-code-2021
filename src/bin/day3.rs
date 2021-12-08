fn task1(entries : &String) {
    let total_count = entries.lines().count() as u32;
    let mut counts = vec![0; 12];

    for line in entries.lines() {
        for (i, value) in line.chars().enumerate() {
            counts[i] += value.to_digit(2).unwrap();
        }
    }

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

    println!("Value of gamma    : {}", gamma);
    println!("Value of epsilon  : {}", epsilon);
    println!("Multiplied value  : {}", gamma * epsilon);
}

// Task 2 stuff
enum BitImportance {
    MostCommon,
    LeastCommon
}

fn find_most_common_bit(counts: &Vec<Vec<u32>>, position: usize) -> u32 {
    let mut search_value = 0;
    for line in counts {
        search_value += line[position];
    };
    (search_value as f32 / counts.len() as f32).round() as u32
}

fn find_target_value(entry_list: &Vec<Vec<u32>>, importance: BitImportance) -> Result<Vec<u32>, &'static str> {
    let mut search_list = entry_list.to_owned();
    for i in 0..search_list[0].len() {
        let most_common_bit = find_most_common_bit(&search_list, i);
        let search_bit = match importance {
            BitImportance::MostCommon => most_common_bit,
            BitImportance::LeastCommon => 1 - most_common_bit
        };

        let previous_first_element = search_list[0].clone();
        search_list
            .retain(|entry| entry[i] == search_bit);
        
        match search_list.len() {
            0 => return Ok(previous_first_element),
            1 => return Ok(search_list[0].clone()),
            _ => {}
        };
    }
    Err("No target value found")
}

fn task2(entries : &String) {
    let entry_lines = entries.lines().collect::<Vec<_>>();
    let entry_list : Vec<Vec<u32>>= entry_lines
        .iter()
        .map(|line|
            line.chars()
            .map(|entry| entry.to_digit(2).unwrap())
            .collect::<Vec<_>>())
        .collect();

    let oxygen_rating = find_target_value(&entry_list, BitImportance::MostCommon).unwrap();
    let co2_rating = find_target_value(&entry_list, BitImportance::LeastCommon).unwrap();

    let mut oxygen_value = 0;
    let mut co2_value = 0;
    let factor: i32 = 2;

    for (i, value) in oxygen_rating.iter().rev().enumerate() {
        if *value > 0 { oxygen_value += factor.pow(i as u32); }
    }
    for (i, value) in co2_rating.iter().rev().enumerate() {
        if *value > 0 { co2_value += factor.pow(i as u32); }
    }
    
    println!("Oxygen rating     : {}", oxygen_value);
    println!("CO2 rating        : {}", co2_value);
    println!("Multiplied value  : {}", oxygen_value * co2_value);
}

fn main() {
    let entries = aoc_lib::read_entries("day3.txt");

    println!("Task 1");
    task1(&entries);

    println!("Task 2");
    task2(&entries);

}