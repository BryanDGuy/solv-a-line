# solv-a-line

A Sudoku Solver Library written in Rust that uses back-tracking to solve.

## Basic Usage

Pass in a reference to a 2D vector to the SudokuBoard constructor. This will validate that all values are [0..9] inclusive.

```rust
let sudoku_board = SudokuBoard::new(&[
    0,0,0, 0,0,0, 0,0,0,
    0,0,2, 0,0,5, 0,4,0,
    1,0,8, 0,4,0, 0,0,0,
    0,0,0, 0,0,0, 4,0,3,
    0,0,6, 0,5,0, 0,0,1,
    0,0,0, 0,2,0, 0,0,6,
    3,0,1, 0,0,0, 0,8,0,
    2,0,7, 0,0,0, 6,0,0,
    0,0,0, 0,0,6, 1,3,9
]);
```

Pass in a reference to this SudokuBoard to the SudokuSolver constructor. This will validate that your starting 
configuration doesn't immediately break the rules of sudoku (multiple of the same values in a row/column/nonet).

```rust
let sudoku_solver = SudokuSolver::new(&sudoku_board);
```

You can now use the solve() function in the sudoku_solver. The first time the function is called will solve the puzzle
using a back-tracking algorithm. The result will get saved into memory of the solver object. This way on subsequent calls
of solve(), there will be no cost on memory and CPU usage.

```rust
let solved_board = sudoku_solver.solve();
println!("{}", solved_board);

### Output ###
┌                   ┐
│ 4 3 9 6 8 2 7 1 5 │
│ 6 7 2 1 3 5 9 4 8 │
│ 1 5 8 7 4 9 3 6 2 │
│ 8 1 5 9 6 7 4 2 3 │
│ 7 2 6 4 5 3 8 9 1 │
│ 9 4 3 8 2 1 5 7 6 │
│ 3 6 1 5 9 4 2 8 7 │
│ 2 9 7 3 1 8 6 5 4 │
│ 5 8 4 2 7 6 1 3 9 │
└                   ┘
```
