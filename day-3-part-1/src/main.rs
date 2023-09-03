use day_3::{self, get_priority_hash_map};
use unicode_segmentation::UnicodeSegmentation; // need to evaluate half of string graphemes

fn main() {
    let file_contents = day_3::read_input();

    let mut count = 0usize;
    let priority_mapping = get_priority_hash_map();
    for row in file_contents.split("\n") {
        // final row will be empty
        // if row != "" {
        let mut compartment_1 = UnicodeSegmentation::graphemes(row , true).collect::<Vec<&str>>();
        let compartment_2 = compartment_1.split_off(compartment_1.len() / 2);

        // filter the halves for matching characters
        let mut characters_1 = compartment_1.iter();
        for char in compartment_2.iter() {
            if let Some(c) = characters_1.find(|&x| x == char) {
                print!("{},", c);
                if let Some(curr_priority) = priority_mapping.get(&c.chars().next().unwrap()) {
                    count += curr_priority;
                    println!("{}, {}", curr_priority, count);
                }
            }
        }
        // }
    }
    println!("{}", count);
}
