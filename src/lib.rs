use std::fs;

/// Read entries to a string
/// 
/// Note that filename should be without any directory, i.e.:
/// `day1.txt`
/// NOT `inputs/day1.txt`
pub fn read_entries(filename: &str) -> String{
    let dir_filename = "inputs/".to_owned() + filename;
    fs::read_to_string(dir_filename)
        .expect("Something went wrong trying to read the file")
}