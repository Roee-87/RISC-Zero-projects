use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminData {
    pub password: String, // will hash the password on the Server
    pub puzzle_size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndGame {
    pub password: String, // will hash the password on the Server
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PuzzleSize {
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Winner {
    pub score: u32,
    pub winner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub strategy: Vec<u32>,
    pub name: String,
}
