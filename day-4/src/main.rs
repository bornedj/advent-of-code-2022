use std::fs;

fn main() {
    let file_contents = read_input();

    let mut count = 0usize;
    let file_vec = file_contents.split("\n").collect::<Vec<&str>>();
    for line in file_vec.iter() {
        if line != &"" {
            let (min_1, max_1, min_2, max_2) = get_mins_and_maxs(line);

            if min_1 <= max_2 && min_2 <= max_1 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

pub fn read_input() -> String {
    let file_contents = fs::read_to_string("day-4/input.txt").expect("Failed to read file");
    file_contents
}

pub fn get_mins_and_maxs(line: &str) -> (u16, u16, u16, u16) {
        let mut line_iter = line.split(",");
        let elf_1 = line_iter.next().unwrap();
        let elf_2 = line_iter.next().unwrap();

        let elf_1_nums = elf_1.split("-").collect::<Vec<&str>>();
        let elf_2_nums = elf_2.split("-").collect::<Vec<&str>>();


        let min_1 = elf_1_nums[0].parse::<u16>().unwrap();
        let max_1 = elf_1_nums[1].parse::<u16>().unwrap();
        let min_2 = elf_2_nums[0].parse::<u16>().unwrap();
        let max_2 = elf_2_nums[1].parse::<u16>().unwrap();
        (min_1, max_1, min_2, max_2)
}

pub fn part_one(file_vec: Vec<&str>, mut count: usize) {
    for line in file_vec.iter() {
        if line != &"" {
            let (min_1, max_1, min_2, max_2) = get_mins_and_maxs(line);
            println!("min1: {} max1: {} min2: {} max2:{}",
                   min_1, max_1, min_2, max_2);
            if (min_1 <= min_2) && max_1 >= max_2 {
                println!("adding one");
                count += 1;
            } else if min_2 <= min_1 && max_2 >= max_1 {
                count += 1;
            }
        }
    }
}
