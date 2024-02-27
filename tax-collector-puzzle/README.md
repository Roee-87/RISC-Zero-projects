Tax Collector Puzzle with Pause/Resume.

Rules: https://mathforlove.com/wp-content/uploads/2023/02/Beat-the-Tax-Collector.pdf

Explanation: https://www.notion.so/risczero/Solutions-Engineering-Project-Ideas-4ba3ddf6f22b46c3b23a4ba87c1cc152?p=795c05ee85a04e7e8057773400baa474&pm=s&pvs=31

Try to beat the tax collector for array [1..N].

There are two ways to interface with the program. You have the option of using a very basic front end, or you can make http requests directly in the dev.http window in the zkvm directory. Instructions for making HTTP requests directly can be found in the ZKVM ReadMe.

To start the backed server, navigate to the ZKVM directory and run `RUST_LOG=info cargo run` or just `cargo run`. If you'd like to use the front end, navigate to web-folder/puzzle-app, install the dependencies with `npm install`, and run `nmp start`. To begin the game loop, the Admin enters credentials (password is "TEMP") and sets the size of the puzzle array using the admin page http://localhost/3000/admin. After clicking the start game button, users will be allowed to enter submissions at http://localhost:3000/user-submission. The Admin can end the game by clicking the same button, which should have now toggled to "end game". Once the admin has ended the game, users can press the "View Winner" button to see who won the game.
