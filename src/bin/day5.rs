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


    
fn main() {
    task1();
}