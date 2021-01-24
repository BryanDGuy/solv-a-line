# solv-a-line

A Sudoku Solver Library written in Rust that uses back-tracking to solve.

## Basic Usage

Pass in a reference to a 2D vector to the SudokuBoard constructor. This will validate that you are
sending in a valid starting configuration such as it being 9x9 and that all values are [0..9] inclusive.

```rust
let sudoku_board = SudokuBoard::new(&vec![
    vec![ 0,0,0, 0,0,0, 0,0,0 ],
    vec![ 0,0,2, 0,0,5, 0,4,0 ],
    vec![ 1,0,8, 0,4,0, 0,0,0 ],
    vec![ 0,0,0, 0,0,0, 4,0,3 ],
    vec![ 0,0,6, 0,5,0, 0,0,1 ],
    vec![ 0,0,0, 0,2,0, 0,0,6 ],
    vec![ 3,0,1, 0,0,0, 0,8,0 ],
    vec![ 2,0,7, 0,0,0, 6,0,0 ],
    vec![ 0,0,0, 0,0,6, 1,3,9 ]
]);
```

Pass in a reference to this SudokuBoard to the SudokuSolver constructor. This will validate that your
starting configuration doesn't immediately break the rules of sudoku (such as multiple of the same values
in a row/column/nonet).

```rust
let sudoku_solver = SudokuSolver::new(&sudoku_board);
```

You can now use the solve() function in the sudoku_solver. The first time the function is called will solve the puzzle
using a back-tracking algorithm. The result will get saved into memory of the solver object. This way on subsequent calls
of solve(), there will be no cost on memory and CPU usage.

```rust
let solved_board = sudoku_solver.solve();
println!(solved_board.configuration);
```
