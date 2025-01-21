## About
Performing various ways of solving the sudoku puzzle (We look for the first solution, it's best if the sudoku has a single solution). I try to improve the way of solving the puzzle and add the strategies to do so.

The baseline for comparison is pure backtracking solution where fields are solved for sequentialy left to right, top to bottom, for all the possible numbers. It uses recursion to backtrack until all possible numbers are exhausted.

## Tests
Please use the provided tests to run the solutions

### Usage
`cargo test --lib -- --show-output`
```
running 3 tests
test game_many ... ignored, time consuming
test backtrack_raw::game1 ... ok
test backtrack_lvn::game1 ... ok

successes:

---- backtrack_raw::game1 stdout ----
solution took 10037 solve steps

---- backtrack_lvn::game1 stdout ----
solution took 302 solve steps


successes:
    backtrack_lvn::game1
    backtrack_raw::game1

test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.02s
```