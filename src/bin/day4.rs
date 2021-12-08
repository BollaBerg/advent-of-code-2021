use aoc_lib;

const ROW_SIZE : usize = 5;

#[derive(Clone)]
struct Board {
    entries : [u8; ROW_SIZE * ROW_SIZE],
    called_entries : [bool; ROW_SIZE * ROW_SIZE],
}

impl Board {
    fn new(entries : [u8; ROW_SIZE * ROW_SIZE]) -> Board{
        Board {entries: entries, called_entries: [false; 5*5]}
    }

    fn from_string(s: &str) -> Board {
        let entries : Vec<u8> = s.split_ascii_whitespace().map(|v| v.parse::<u8>().unwrap()).collect();
        let mut entries_input : [u8; ROW_SIZE*ROW_SIZE] = [0; ROW_SIZE*ROW_SIZE];
        entries_input.copy_from_slice(&entries[..]);
        Board::new(entries_input)
    }

    fn pick_number(&mut self, picked_number : u8) -> bool{
        let index = match self.entries.iter().position(|&v| v == picked_number) {
            Some(x) => x,
            _ => return false
        };
        self.called_entries[index] = true;
        self.check_win(index)
    }

    fn check_win(&self, index: usize) -> bool {
        // Check row
        let mut row_win = true;
        for row in 0..ROW_SIZE {
            let element_index = row * ROW_SIZE + index % ROW_SIZE;
            if self.called_entries[element_index] == false {row_win = false}
        }
        if row_win {return true};

        // Check column
        let mut col_win = true;
        let row_start = index - (index % 5);
        for col in 0..ROW_SIZE {
            let element_index = row_start + col;
            if self.called_entries[element_index] == false {col_win = false}
        }
        if col_win {return true};

        false
    }

    fn score(&self) -> u32 {
        let mut score : u32 = 0;
        for (i, picked) in self.called_entries.iter().enumerate() {
            if !*picked {score += self.entries[i] as u32}
        }
        score
    }
}

fn task1(entries: &Vec<u8>, boards: &mut Vec<Board>) {
    for &entry in entries {
        for board in &mut *boards {
            if board.pick_number(entry) {
                println!("BINGO");
                println!("Picked number: {}", entry);
                println!("Board score: {}", board.score());
                println!("Final score: {}", board.score() * entry as u32);
                return;
            }
        }
    }
}

fn main() {
    let data = aoc_lib::read_entries("day4.txt");
    let entries : Vec<u8> = data
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|entry| entry.parse::<u8>().unwrap())
        .collect();
    
    let mut boards = Vec::<Board>::new();
    for board in data
        .split("\n\n")
        .skip(1)    // Skip 'entries'
        {
            boards.push(Board::from_string(board));
        }
    
    println!("Task 1");
    task1(&entries, &mut boards);
}