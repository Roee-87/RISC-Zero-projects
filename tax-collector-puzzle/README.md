# Multiplayer Tax Collector Puzzle Game.

Try to beat the tax collector for array [1..N].

## Introduction and Rules

Full Explanation of the Rules: https://mathforlove.com/wp-content/uploads/2023/02/Beat-the-Tax-Collector.pdf

The Admin sets the size of the puzzle array. Players select a number from the array that has at least one of its factors still present in the array. If no factors are present, that number is inaccessible and must remain in the array. For example, if a player select the number `6` from an array `[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`, the player then collects the number `6` but must pay the tax collector any present factors of the number `6`. Selecting `6` would result in a new array `[4, 5, 7, 8, 9, 10]`. The player can now only select `8` or `10` for the next move because only those two numbers have any remaining factors present in the new array.

Any remaining numbers left in the array are awarded to the tax collector. The player wins if his score is higher than the tax collector's score. The webserver allows for multiple players to play against each other. In a multi-player game, the earliest highest scoring submission will be the winner.

## RISC Zero zkVM

The server backend uses the RISC zero-knowledge virtual machine to generate two types of proofs:

1.  Proof that a user submission has been succesffuly submitted and scored correctly.
2.  Proof that the winning submission has been determined correctly.

Proofs for user submission are verified in the backend (in future proofs will be verfified in the browser using WASM). Similarly, the final proof attesting to the correct winner is verified in the backend as well.

## Playing the game

There are two ways to interface with the backend. You have the option of using a very basic React front end or make http requests directly in the dev.http window in the zkvm directory. Instructions for making HTTP requests directly can be found in the ZKVM README.

To start the backed server, navigate to the zkvm directory and run `RUST_LOG=info cargo run` or just `cargo run`. If you'd like to use the front end, navigate to web-folder/puzzle-app, install the dependencies with `npm install`, and run `nmp start`. To begin the game loop, the Admin enters credentials (password is "TEMP") and sets the size of the puzzle array `N` using the admin page http://localhost/3000/admin. After clicking the start game button, users will be allowed to enter submissions at http://localhost:3000/user-submission. The Admin can end the game by clicking the same button, which should have now toggled to "end game". Once the admin has ended the game, users can press the "View Winner" button to see who won the game.

The game loop resets when the admin starts a new game.
