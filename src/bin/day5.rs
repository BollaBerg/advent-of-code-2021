use aoc_lib;
use std::cmp;

#[cfg(test)]
mod day5_test;

#[derive(Debug, PartialEq)]
struct Vent {
    start : (usize, usize),
    end : (usize, usize),
}

impl Vent {
    fn from_string(string : &str) -> Vent {
        let mut tuples = string.split(" -> ").to_owned();
        let mut start_iterator = tuples.next().unwrap().split(",");
        let start = (
            start_iterator.next().unwrap().parse::<usize>().unwrap(),
            start_iterator.next().unwrap().parse::<usize>().unwrap()
        );
        let mut end_iterator = tuples.next().unwrap().split(",");
        let end = (
            end_iterator.next().unwrap().parse::<usize>().unwrap(),
            end_iterator.next().unwrap().parse::<usize>().unwrap()
        );

        Vent {start, end}
    }

    fn update_only_horizontal_and_vertical(&self, board : &mut [[u8 ; 1000] ; 1000]) {
        if self.start.0 == self.end.0 {
            let x = self.start.0;
            let min = cmp::min(self.start.1, self.end.1);
            let max = cmp::max(self.start.1, self.end.1);
            for y in min ..= max {
                board[x][y] += 1;
            }
        }
        else if self.start.1 == self.end.1 {
            let y = self.start.1;
            let min = cmp::min(self.start.0, self.end.0);
            let max = cmp::max(self.start.0, self.end.0);
            for x in min ..= max {
                board[x][y] += 1;
            }
        }
    }

    fn update_all(&self, board : &mut [[u8; 1000] ; 1000]) {
        let delta_x : i16 = self.end.0 as i16 - self.start.0 as i16;
        let delta_y : i16 = self.end.1 as i16 - self.start.1 as i16;

        if delta_x.abs() == delta_y.abs() {
            let x_add = if delta_x > 0 {true} else {false};
            let y_add = if delta_y > 0 {true} else {false};
            let mut x = self.start.0;
            let mut y = self.start.1;
            loop {
                board[x][y] += 1;
                if x == self.end.0 { break; }
                if x_add {x += 1;} else {x -= 1;};
                if y_add {y += 1;} else {y -= 1;};
            }
        } else {
            self.update_only_horizontal_and_vertical(board);
        }
    }
}


fn count(array : &[[u8; 1000] ; 1000]) -> usize {
    array
        .iter()
        .flatten()
        .filter(|&&val| val >= 2)
        .count()
}


fn task1() {
    println!("#####   TASK 1   #####");

    let mut map : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    for line in aoc_lib::read_entries("day5.txt").lines() {
        let vent = Vent::from_string(line);
        vent.update_only_horizontal_and_vertical(&mut map);
    }

    let map_count = count(&map);
    
    println!("Points where two or more lines overlap: {}", map_count);
}


fn task2() {
    println!("#####   TASK 2   #####");

    let mut map : [[u8; 1000] ; 1000] = [[0; 1000] ; 1000];
    for line in aoc_lib::read_entries("day5.txt").lines() {
        let vent = Vent::from_string(line);
        vent.update_all(&mut map);
    }

    let map_count = count(&map);
    
    println!("Points where two or more lines overlap: {}", map_count);
}


fn main() {
    task1();
    task2();
}