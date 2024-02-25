mod admin;
pub use admin::{AdminData, EndGame, Entry, PuzzleSize, Winner};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const N: u32 = 22;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Submission {
    puzzle_array: Vec<u32>,
    puzzle_sum: u32,
    strategy: Vec<u32>,
    legal_flag: bool,
    score: u32,
    victory_flag: bool,
    entry_val: i32,
    name: String,
}

impl Submission {
    pub fn new(strat: Vec<u32>, player: String, n: u32) -> Self {
        let starting_array: Vec<u32> = (1..n + 1).collect();
        let total: u32 = starting_array.iter().sum();
        Submission {
            puzzle_array: starting_array,
            puzzle_sum: total,
            strategy: strat,
            legal_flag: true,
            score: 0,
            victory_flag: false,
            entry_val: 0,
            name: player,
        }
    }

    pub fn calculate_score(&mut self, factors_table: &HashMap<u32, Vec<u32>>) -> Result<(), ()> {
        let mut flag = false;
        for num in &self.strategy {
            if self.puzzle_array.contains(num) {
                let divs = factors_table.get(&num).unwrap();
                for val in divs {
                    flag = false;
                    if self.puzzle_array.contains(&val) {
                        flag = true;
                        let index = self.puzzle_array.iter().position(|x| *x == *val).unwrap();
                        self.puzzle_array.remove(index);
                    }
                }
                if flag == false {
                    self.legal_flag = false;
                    self.score = 0;
                    return Ok(());
                }
                // this if/else block mitigates a bug when the first value in the strategy is 1.
                // 1 is its own factor...which causes a crash if this scenario isn't accounted for manually.
                // There is probably a better way to handle this... todo!() later.
                if self.strategy[0] == 1u32 && *num == 1u32 {
                    self.score = 1;
                } else {
                    let index = self.puzzle_array.iter().position(|x| *x == *num).unwrap();
                    let _ = self.puzzle_array.remove(index);
                    self.score += *num as u32;
                }
            } else {
                self.legal_flag = false;
                self.score = 0;
                return Ok(());
            }
            // println!("puzzle array 2: {:?}", self.puzzle_array);
        }
        if self.score > self.puzzle_sum - self.score {
            self.victory_flag = true;
        }
        Ok(())
    }

    pub fn set_entry_val(&mut self, num: &i32) {
        self.entry_val = *num;
    }

    pub fn is_legal(&self) -> bool {
        self.legal_flag
    }

    pub fn is_victorious(&self) -> bool {
        self.victory_flag
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_entry_val(&self) -> i32 {
        self.entry_val
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn print_result(&self) {
        // These printouts are not necessary, but are useful to see if the submissions are calculated correctly.
        if self.legal_flag == false {
            println!("You submission was invalid. \n");
        } else {
            if self.is_victorious() == true {
                println!(
                    "Huzzah!  You beat the tax collector with a score of {}.\n",
                    self.score
                );
            } else {
                println!("You have suffered defeat at the hands of the Tax Man.\n");
            }
        }
    }
}

impl PartialOrd for Submission {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.score.cmp(&other.score) {
            std::cmp::Ordering::Equal => Some(self.entry_val.cmp(&other.entry_val)),
            ordering => Some(ordering),
        }
    }
}

impl Ord for Submission {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.score.cmp(&other.score) {
            std::cmp::Ordering::Equal => self.entry_val.cmp(&other.entry_val),
            ordering => ordering,
        }
    }
}

fn get_divisors(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    let n_sqrt = (n as f32).sqrt() as u32 + 1;

    for i in 2..n_sqrt {
        if n % i == 0 {
            if n / i == i {
                v.push(i);
            } else {
                v.push(i);
                v.push(n / i);
            }
        }
    }
    v.push(1u32);
    v.sort();
    v
}

pub fn get_divisor_hashmap(n: u32) -> HashMap<u32, Vec<u32>> {
    let factors_table: HashMap<u32, Vec<u32>> = (1..n + 1).map(|x| (x, get_divisors(x))).collect();
    factors_table
}

// pub fn enter_new_submission() -> Submission {
//     // read in a strategy from the command line
//     print!("Enter your strategy:  ");
//     io::stdout().flush().expect("Cannot flush stdout");

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     // Split the line into individual numbers
//     let strat: Vec<u32> = input
//         .split_whitespace()
//         .filter_map(|s| s.parse().ok())
//         .collect();

//     Submission::new(strat)
// }
