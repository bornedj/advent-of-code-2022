fn highest_calorie_count() {
    // read file
    let calories_input_contents =
        read_to_string("./calories/calories_input.txt").expect("failed to read file");

    // iter through throws on carriage. Keep running total of highest value
    let mut highest_total: u64 = 0;
    let mut current_total: u64 = 0;
    let calories: Vec<&str> = calories_input_contents.split("\n").collect();
    for line in calories.iter() {
        // it's either a blank or a num
        if line == &"" {
            if current_total > highest_total {highest_total = current_total}
            current_total = 0;
        } else {
            let num = match line.parse::<u64>() {
                Ok(number) => number,
                Err(err) => panic!("Failed to parse unsigned integer. {:?}",err),
            };
            current_total += num;
        }
    }
}
