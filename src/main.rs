use rand::Rng;
use std::io;

fn main() {
    println!("Linux File Perms Quiz!");
    println!("Choose mode: 1 for Relevant, 2 for Generated");

    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");

    let permissions = [
        ("---", "0", "No permissions"),
        ("--x", "1", "Execute only"),
        ("-w-", "2", "Write only"),
        ("-wx", "3", "Write and execute"),
        ("r--", "4", "Read only"),
        ("r-x", "5", "Read and execute"),
        ("rw-", "6", "Read and write"),
        ("rwx", "7", "Read, write, and execute"),
    ];

    let predefined_mappings = vec![
        ("rwx------", "700", "Owner can read, write, and execute. No permissions for group and others."),
        ("rw-r--r--", "644", "Owner can read and write. Group can read only. No permissions for others."),
        ("rwxr-xr-x", "755", "Owner can read, write, and execute. Group and others can read and execute."),
        ("rw-rw-r--", "664", "Owner and group can read and write. No permissions for others."),
        ("r--r--r--", "444", "Owner, group, and others can only read."),
        ("rwx--x--x", "711", "Owner can read, write, and execute. Group and others can only execute."),
        ("rwxrwxrwx", "777", "Owner, group, and others can read, write, and execute."),
        ("rwxrwx---", "770", "Owner and group can read, write, and execute. No permissions for others."),
        ("rwxr-----", "740", "Owner can read, write, and execute. Group can only read. No permissions for others."),
        ("rwx-----x", "701", "Owner can read, write, and execute. Others can only execute. No permissions for group."),
        ("rw-------", "600", "Owner can read and write. No permissions for group and others."),
        ("rwxr-x---", "750", "Owner can read, write, and execute. Group can read and execute. No permissions for others."),
        ("r-xr-xr-x", "555", "Owner, group, and others can only execute."),
        ("r-x------", "500", "Owner can read and execute. No permissions for group and others."),
        ("r--------", "400", "Owner can only read. No permissions for group and others."),
        ("--xr-xr-x", "055", "Group and others can read and execute. No permissions for owner."),
        ("--xrwxrwx", "077", "Group and others can read, write, and execute. No permissions for owner."),
        ("rwsr-xr-x", "4755", "Owner can read, write, and execute with setuid. Group and others can read and execute."),
        ("rwxrws---", "2770", "Owner and group can read, write, and execute with setgid. No permissions for others."),
        ("rwxrwxrwt", "1777", "Owner, group, and others can read, write, and execute with sticky bit."),
    ];


    let mut rng = rand::thread_rng();
    let mut score = 0;
    let mut total_questions = 0;

    loop {
        let question_type: bool = rng.gen();
        let perm_index = rng.gen_range(0..permissions.len());
        let predefined_index = rng.gen_range(0..predefined_mappings.len());

        let (perm_str, perm_num, explanation) = if mode.trim() == "1" {
            let predefined = &predefined_mappings[predefined_index];
            (predefined.0.to_string(), predefined.1.to_string(), predefined.2)
        } else {
            let perm = &permissions[perm_index];
            (perm.0.repeat(3), format!("{:o}", perm_index * 64 + perm_index * 8 + perm_index), perm.2)
        };

        println!("Identify the {} for: {}", if question_type { "permission string" } else { "numerical value" }, if question_type { &perm_num } else { &perm_str });

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read line");
        let answer = answer.trim();

        let correct_answer = if question_type { &perm_str } else { &perm_num };

        if answer == correct_answer {
            println!("Correct! {}", explanation);
            score += 1;
        } else {
            println!("Incorrect! The correct answer was {}. {}", correct_answer, explanation);
        }

        total_questions += 1;

        println!("Continue playing? (yes/no)");
        let mut decision = String::new();
        io::stdin().read_line(&mut decision).expect("Failed to read line");

        if decision.trim().eq("no") {
            break;
        }
    }

    println!("GG! Your score was {}/{}", score, total_questions);
    println!("Thanks for playing!");
}
