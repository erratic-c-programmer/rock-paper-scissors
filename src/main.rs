// use chrono::Utc;
// use oorandom::Rand32;
use rand::Rng;
use std::io;

fn main() {
    let mut human_score = 0;
    let mut computer_score = 0;
    const SCORE_END: u32 = 10;

    while human_score < SCORE_END && computer_score < SCORE_END {
        let mut human_in = String::new();
        io::stdin()
            .read_line(&mut human_in)
            .expect("failed to read line");
        human_in = String::from(human_in.trim());
        if !valid_rps_p(&human_in[..]) {
            println!(
                "Please input one of 'rock', 'paper' or 'scissors', not {}.",
                human_in
            );
            continue;
        }
        let computer_guess = compgen();
        let cur_score = score_rps(&human_in[..], &computer_guess[..]);
        print!(
            "You chose {}, and the computer chose {}. ",
            human_in, computer_guess
        );
        if cur_score.0 {
            human_score += 1;
            println!("You scored a point!");
        } else if cur_score.1 {
            computer_score += 1;
            println!("The computer scored a point.");
        } else {
            println!("Draw...");
        }
        println!(
            "SCORES: {} (you) -- {} (computer)",
            human_score, computer_score
        );
    }
}
fn compgen() -> String {
    // let mut rng = Rand32::new(Utc::now().timestamp_millis() as u64);
    // return String::from(["rock", "paper", "scissors"][(rng.rand_u32() % 3) as usize]);
    let mut rng = rand::thread_rng();
    return String::from(["rock", "paper", "scissors"][rng.gen::<usize>() % 3]);
}

fn valid_rps_p(s: &str) -> bool {
    if s == "rock" || s == "paper" || s == "scissors" {
        return true;
    } else {
        return false;
    }
}

fn score_rps(p1: &str, p2: &str) -> (bool, bool) {
    if p1 == p2 {
        return (false, false);
    } else if (p1 == "rock" && p2 == "scissors")
        || (p1 == "scissors" && p2 == "paper")
        || (p1 == "paper" && p2 == "rock")
    {
        return (true, false);
    } else {
        return (false, true);
    }
}
