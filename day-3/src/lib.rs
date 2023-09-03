use std::{fs, collections::HashMap};
pub fn read_input() -> String {
    let file_contents = fs::read_to_string("day-3/input.txt").expect("Failed to read file");
    file_contents
}

pub fn get_priority_hash_map() -> HashMap<char, usize> {
    let mut priority_mapping = HashMap::new();
    for (i,char) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate() {
        priority_mapping.insert(char, (i + 1) as usize);
    }
    priority_mapping
}
