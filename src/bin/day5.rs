use aoc_lib;

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
            for y in self.start.1 ..= self.end.1 {
                board[x][y] += 1;
            }
        }
        if self.start.1 == self.end.1 {
            let y = self.start.1;
            for x in self.start.0 ..= self.end.0 {
                board[x][y] += 1;
            }
        }
    }
}

fn main() {
    
}