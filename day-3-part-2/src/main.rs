use std::collections::HashMap;
use day_3::{self, get_priority_hash_map};

fn main() {
    let file_contents = day_3::read_input();

    let mut count = 0usize;
    let priority_mapping = get_priority_hash_map();

    let file_vec = file_contents.split("\n").collect::<Vec<&str>>();
    for i in (0..file_vec.len()).step_by(3) {
        if i + 1 >= file_vec.len() || i + 2 > file_vec.len() {
            break;
        }
        println!("{}", i);
        let elf_1 = file_vec[i];
        let elf_2 = file_vec[i+1];
        let elf_3 = file_vec[i+2];
        // let mut common_char: Vec<char> = vec![];
        let mut common_char: HashMap<char, bool> = HashMap::new();
        for char in elf_1.chars() {
            elf_2.chars().for_each(|c| {
                if c == char {
                    common_char.entry(char).or_insert(true);
                }
            });
        }
        for char in common_char.keys() {
            if let Some(c) = elf_3.chars().find(|&x| x == *char) {
                let val = priority_mapping.get(&c).unwrap(); 
                count += val;
                println!("char: {}, val: {}, sum: {}", c, val, count);
                break;
            }
        }
    }
    println!("results: {}", count);
}
