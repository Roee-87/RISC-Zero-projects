Code for tax-collector puzzle backend.

Tax Collector Puzzle with Pause/Resume.

Rules: https://mathforlove.com/wp-content/uploads/2023/02/Beat-the-Tax-Collector.pdf

Explanation: https://www.notion.so/risczero/Solutions-Engineering-Project-Ideas-4ba3ddf6f22b46c3b23a4ba87c1cc152?p=795c05ee85a04e7e8057773400baa474&pm=s&pvs=31

Try to beat the tax collector for array [1..N].



`RUST_LOG=info cargo run` or just `cargo run` if you don't want to inspect the log.

To start the game, use this POST request:
`POST http://127.0.0.1:8080/start
Content-Typ:  application/json

{
    "password": "TEMP",
    "puzzle_size": 22
}`

Then click `Send Request` in the `dev.http` window.  This will enable submission requests.

You can change the puzzle_size to any integer.

All of the HTTP requests along with sample entries are available in the `dev.http`.  You just need to comment out the request in order to enable it. 
You can change the submission entries as well.  

Here are sample submissions:

Legal and top scoring submission: 19, 21, 18, 16, 15, 22, 20

Legal example: 19, 20, 12, 16

Illegal example: 20, 19, 12, 25

Another illegal example: 5, 1

You can submit as many entries as you like using:
`POST http://127.0.0.1:8080/submission
Content-Typ:  application/json

{
    "strategy": [19, 21, 18, 16, 15, 22, 20],
    "name": "Mike"
}`

When you are ready to end the game, send the following request:

`POST http://127.0.0.1:8080/end
Content-Typ:  application/json

{
    "password": "TEMP"
}`

Lastly, to determine the winner, submit a GET Request:
`GET http://127.0.0.1:8080/winner`


Scores are the sum of the integers that the user was able to legally select from the puzzle array.  Illegal submissions get a score of zero.  Beating the tax colector means that the user sum exceeds the sum of the numbers remaining in the puzzle array plus the numbers collected by the tax collector. 