#![no_main]
use puzzle_core::{Submission, get_divisor_hashmap};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let puzzle_size: u32 = env::read();
    let game_over: bool = env::read();
    
    let divisor_hashmap = get_divisor_hashmap(puzzle_size);

    let mut entries: Vec<Submission> = Vec::new();
    let mut contestant_val: i32 = 0;
    //env::commit(&contestant_val);
    // verified with debugging that we reach this point.
    env::pause(0);

    while !game_over {
        let game_over: bool = env::read();
        if game_over == false {
            break
        }
        let mut submission_next: Submission = env::read();
        submission_next.calculate_score(&divisor_hashmap).unwrap();

        // assign entry value.  Decrementing by 1 so earliest entries will be ranked higher in tiebreaking scnario.
        contestant_val -= 1;

        submission_next.set_entry_val(&contestant_val);

        env::commit(&submission_next);
        entries.push(submission_next);

        env::pause(0);
        }

    println!("Sorting the contestants by score...\n");
    entries.sort();   // Sort the scores to determine the winner.  Tiebreaker is the earlier submission.

    let winner: Submission = entries[(entries.len() -1) as usize].clone();
    let winning_score = winner.get_score();
    let winning_contestant = winner.get_name();

    // commit to the winner
    env::commit(&(&winning_score, &winning_contestant));
}

