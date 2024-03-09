use std::io;

fn main() {
    let cheese_string = "bxzeacheesesae";
    println!(
        "Find and slice 'cheeses' from the string: {}",
        cheese_string
    );

    let correct_start = cheese_string.find("cheeses").unwrap_or(0);
    let correct_end = correct_start + "cheeses".len();

    println!("Enter the start index of 'cheeses':");
    let start_index = read_user_input();

    println!("Enter the end index of 'cheeses':");
    let end_index = read_user_input();

    if start_index == correct_start && end_index == correct_end {
        println!(
            "Correct! Here's your slice: {}",
            &cheese_string[start_index..end_index]
        );
    } else {
        println!("Oops! That's not right. Try again?");
    }
}

fn read_user_input() -> usize {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    input_string.trim().parse::<usize>().unwrap_or(0)
}
