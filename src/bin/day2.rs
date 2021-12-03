fn task1(entries: &String) {
    let mut depth = 0;
    let mut horizontal = 0;

    for line in entries.lines() {
        let mut splitline = line.split(' ');
        let command = splitline.next().unwrap();
        let value = splitline
            .next()
            .unwrap()
            .parse::<u16>()
            .unwrap();
        
        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unknown command!")
        }
    };
    
    println!("Depth after instructions: {}", depth);
    println!("Horizontal after instructions: {}", horizontal);
    println!("Multiplying the two: {}", u32::from(depth) * u32::from(horizontal));
}

fn task2(entries: &String) {
    let mut depth : u32 = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in entries.lines() {
        let mut splitline = line.split(' ');
        let command = splitline.next().unwrap();
        let value = splitline
            .next()
            .unwrap()
            .parse::<u16>()
            .unwrap();
        
        match command {
            "forward" => {
                horizontal += value;
                depth += u32::from(aim * value);
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Unknown command!")
        }
    };
    
    println!("Depth after instructions: {}", depth);
    println!("Horizontal after instructions: {}", horizontal);
    println!("Multiplying the two: {}", u32::from(depth) * u32::from(horizontal));
}

fn main() {
    let entries = aoc_lib::read_entries("day2.txt");

    println!("Task 1");
    task1(&entries);

    println!("Task 2");
    task2(&entries);
}