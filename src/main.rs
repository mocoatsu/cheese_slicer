use std::io;

use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let cheese_string = generate_random_cheese_string(10);
    println!(
        "Find and slice 'cheeses' from the string: {}",
        cheese_string
    );

    println!("Enter the start index of 'cheeses':");
    let start_index = read_user_input();

    println!("Enter the end index of 'cheeses':");
    let end_index = read_user_input();

    if is_correct(&cheese_string, start_index, end_index) {
        println!(
            "Correct! Here's your slice: {}",
            &cheese_string[start_index..end_index]
        );
    } else {
        println!("Oops! That's not right. Try again?");
    }
}

fn generate_random_cheese_string(length: usize) -> String {
    let mut rng = rand::thread_rng();

    let pre_length = rng.gen_range(0..=length);
    let post_length = length - "cheeses".len();

    let pre_cheese: String = (0..pre_length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    let post_cheese: String = (0..post_length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();

    format!("{}cheeses{}", pre_cheese, post_cheese)
}

fn is_correct(sentence: &str, start: usize, end: usize) -> bool {
    let correct_start = sentence.find("cheeses").unwrap_or(0) + 1;
    let correct_end = correct_start + "cheeses".len() - 1;

    println!(
        "correct_start:{},correct_end:{}",
        correct_start, correct_end
    );
    start == correct_start && end == correct_end
}

fn read_user_input() -> usize {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    input_string.trim().parse::<usize>().unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_generate_random_cheese_string {
        use super::*;
        #[test]

        fn it_contains_cheeses() {
            assert!(generate_random_cheese_string(10).contains("cheese"));
            assert!(generate_random_cheese_string(20).contains("cheese"));
        }
    }

    mod test_is_correct {
        use super::*;
        #[test]

        fn it_returns_true_when_the_answer_is_correct() {
            assert!(is_correct("acheesesb", 2, 8));
            assert!(is_correct("cheesescheeses", 1, 7));
            assert!(is_correct("mecheesesk", 3, 9));
        }
    }
}
