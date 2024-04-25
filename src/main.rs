use rand::Rng;
use std::io;

fn main() {
    println!("File permission guiz! Randomized version");

    let permissions = [
        ("---", 0), ("--x", 1), ("-w-", 2), ("-wx", 3),
        ("r--", 4), ("r-x", 5), ("rw-", 6), ("rwx", 7),
    ];
    let mut score = 0;
    let mut total_questions = 0;

    loop {
        let mut rng = rand::thread_rng();
        let question_type: bool = rng.gen();
        let perm_number: usize = rng.gen_range(0..=7);
        let mode_number: usize = rng.gen_range(0..=7);
        let user_number: usize = rng.gen_range(0..=7);

        let file_perm = format!("{}{}{}", permissions[perm_number].0, permissions[mode_number].0, permissions[user_number].0);
        let numerical_value = format!("{:o}", permissions[perm_number].1 * 64 + permissions[mode_number].1 * 8 + permissions[user_number].1);

        println!("Identify the {} for: {}", if question_type { "permission string" } else { "numerical value" }, if question_type { &numerical_value } else { &file_perm });

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        let answer = answer.trim();

        let correct_answer = if question_type {
            &file_perm
        } else {
            &numerical_value
        };

        if answer == correct_answer {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect! The correct answer was {}", correct_answer);
        }

        total_questions += 1;

        println!("Continue playing? (yes/no)");
        let mut decision = String::new();
        io::stdin().read_line(&mut decision).expect("Failed to read line");

        if decision.trim().eq("no") {
            break;
        }
    }

    println!("Game Over. Your score was {}/{}", score, total_questions);
    println!("Thanks for playing!");
}
