use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SudokuBoard {
    pub board_configuration: Vec<Vec<u8>>
}

impl SudokuBoard {
    fn new(sudoku_puzzle: &Vec<Vec<u8>>) -> SudokuBoard {
        return SudokuBoard {
            board_configuration: sudoku_puzzle.clone()
        }
    }

    fn copy(other: &SudokuBoard) -> SudokuBoard {
        return SudokuBoard {
            board_configuration: other.board_configuration.clone()
        }
    }

    fn get_unsolved_spaces(&self) -> Vec<(usize, usize)> {
        let mut unsolved_spaces = Vec::new();
        for row in 0..=8 {
            for column in 0..=8 {
                if self.board_configuration[row][column] == 0 {
                    unsolved_spaces.push((row, column));
                }
            }
        }
        return unsolved_spaces;
    }

    fn all_spaces_valid(&self) -> bool {
        // All values in a row/column/nonet must be unique, otherwise this breaks the rules of Sudoku

        for row_index in 0..=8 {
            let row = self.get_row(row_index);
            let row_without_unsolved_spaces = row.iter().filter(|&&value| value != 0).map(|value| *value).collect_vec();
            if row_without_unsolved_spaces.iter().unique().collect_vec().len() != row_without_unsolved_spaces.len() {
                return false;
            }
        }

        for column_index in 0..=8 {
            let column = self.get_column(column_index);
            let column_without_unsolved_spaces = column.iter().filter(|&&value| value != 0).map(|value| *value).collect_vec();
            if column_without_unsolved_spaces.iter().unique().collect_vec().len() != column_without_unsolved_spaces.len() {
                return false;
            }
        }

        for nonet_index in 0..=8 {
            let nonet = self.get_nonet(nonet_index);
            let nonet_without_unsolved_spaces = nonet.iter().filter(|&&value| value != 0).map(|value| *value).collect_vec();
            if nonet_without_unsolved_spaces.iter().unique().collect_vec().len() != nonet_without_unsolved_spaces.len() {
                return false;
            }
        }

        return true;
    }

    fn all_spaces_solved(&self) -> bool {
        return !self.board_configuration.iter().any(|row| row.iter().any(|value| *value == 0));
    }

    fn get_row(&self, row_index: usize) -> Vec<u8> {
        let mut row = Vec::new();
        for column_index in 0..=8 {
            row.push(self.board_configuration[row_index][column_index]);
        }
        return row;
    }

    fn get_column(&self, column_index: usize) -> Vec<u8> {
        let mut column = Vec::new();
        for row_index in 0..=8 {
            column.push(self.board_configuration[row_index][column_index]);
        }
        return column;
    }

    fn get_nonet(&self, nonet_index: usize) -> Vec<u8> {
        let starting_row;
        let starting_column;
        match nonet_index {
            0 => { starting_row = 0; starting_column = 0; },
            1 => { starting_row = 0; starting_column = 3; },
            2 => { starting_row = 0; starting_column = 6; },
            3 => { starting_row = 3; starting_column = 0; },
            4 => { starting_row = 3; starting_column = 3; },
            5 => { starting_row = 3; starting_column = 6; },
            6 => { starting_row = 6; starting_column = 0; },
            7 => { starting_row = 6; starting_column = 3; },
            8 => { starting_row = 6; starting_column = 6; },
            _ => { panic!("An invalid nonet_index was passed into 'get_nonet', it was {}", nonet_index)}
        }

        let mut nonet = Vec::new();
        for row_index in starting_row..=(starting_row+2) {
            for column_index in starting_column..=(starting_column+2) {
                nonet.push(self.board_configuration[row_index][column_index]);
            }
        }
        return nonet;
    }
}

pub struct SudokuSolver {
    pub sudoku_puzzle: SudokuBoard,
    pub unsolved_spaces: Vec<(usize, usize)>,
    pub percent_solved: f32,
    solved_board: RefCell<Option<SudokuBoard>>
}

impl SudokuSolver {
    pub fn new(sudoku_puzzle: &Vec<Vec<u8>>) -> SudokuSolver {
        if sudoku_puzzle.len() != 9 || sudoku_puzzle.iter().any(|row| row.len() != 9) {
            panic!("The board must be 9x9.");
        }

        let board = SudokuBoard::new(&sudoku_puzzle);

        if !board.all_spaces_valid() {
            panic!("An invalid starting board configuration was passed.");
        }

        let unsolved_spaces = board.get_unsolved_spaces();
        let unsolved_length: f32 = unsolved_spaces.len() as f32;

        return SudokuSolver {
            sudoku_puzzle: board,
            unsolved_spaces,
            percent_solved: (1.0 - (unsolved_length / (9.0 * 9.0))) * 100.0,
            solved_board: RefCell::new(None)
        }
    }

    pub fn solve(&self) -> SudokuBoard {
        // Back-tracking Algo
        // 1. Check if board is solved. If it is, end.
        // 2. Get Row at current space.
        // 3. Get Column at current space.
        // 4. Get Nonet at current space.
        // 5. Get previously attempted values.
        // 5. Get values [1, 9] that are not in the union of these 4 sets.
        // 6. If there is/are valid value(s), plug in the first valid and move onto step 1 for the next space to solve.
        // 7. If not, move back to the previous space that was solved and plug in the next valid value.


        // Optimization 1: Keep solved board stored in private variable for cached access
        if self.solved_board.borrow().is_some() {
            return SudokuBoard::copy(self.solved_board.borrow().as_ref().unwrap());
        }

        let all_value_candidates = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut solved_board = SudokuBoard::copy(&self.sudoku_puzzle);
        let mut attempted_values: HashMap<(usize, usize), Vec<u8>> = HashMap::new();
        let mut unsolved_spaces_index = 0;

        while !solved_board.all_spaces_solved() {
            let row_index = self.unsolved_spaces[unsolved_spaces_index].0;
            let column_index = self.unsolved_spaces[unsolved_spaces_index].1;
            let nonet_index = 3 * ((9 * row_index + column_index) / 27) + ((9 * row_index + column_index) / 3 % 3);

            solved_board.board_configuration[row_index][column_index] = 0; // Set back to 0 in the case this was a back-tracked space
            let previously_attempted_values = attempted_values.entry((row_index, column_index)).or_default();
            let row = solved_board.get_row(row_index);
            let column = solved_board.get_column(column_index);
            let nonet = solved_board.get_nonet(nonet_index);

            let mut invalid_value_candidates = Vec::new();
            invalid_value_candidates.extend(previously_attempted_values.iter());
            invalid_value_candidates.extend(row.iter().filter(|&&value| value != 0));
            invalid_value_candidates.extend(column.iter().filter(|&&value| value != 0));
            invalid_value_candidates.extend(nonet.iter().filter(|&&value| value != 0));
            invalid_value_candidates = invalid_value_candidates.iter().unique().map(|value| *value).collect_vec();

            let valid_value_candidates = all_value_candidates.iter().filter(|value| !invalid_value_candidates.contains(value)).collect_vec();
            if valid_value_candidates.len() > 0 { // Found a valid value to use
                solved_board.board_configuration[row_index][column_index] = *valid_value_candidates[0];
                attempted_values.entry((row_index, column_index)).or_default().push(*valid_value_candidates[0]);
                unsolved_spaces_index += 1;
            }
            else { // Need to backtrack
                attempted_values.remove(&(row_index, column_index));
                unsolved_spaces_index -= 1;
            }
        };

        self.solved_board.replace(Some(solved_board));
        return SudokuBoard::copy(self.solved_board.borrow().as_ref().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn constructor_works_valid_board() {
        let valid_board = vec![
            vec![ 0,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 0,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,0 ]
        ];
        
        let solver = SudokuSolver::new(&valid_board);
        assert_eq!(solver.sudoku_puzzle.board_configuration, valid_board);
        assert_eq!(solver.unsolved_spaces, vec![
            (0, 0),
            (6, 3),
            (8, 8)
        ]);
        assert_eq!(solver.percent_solved, 96.296295);
        // assert_eq!(solver.solved_board.into_inner(), None);
    }

    #[test]
    #[should_panic]
    fn constructor_works_invalid_board_invalid_rows() {
        let invalid_board_rows = vec![
            vec![ 0,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 0,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ]
        ];
        SudokuSolver::new(&invalid_board_rows);
    }

    #[test]
    #[should_panic]
    fn constructor_works_invalid_board_invalid_columns() {
        let invalid_board_columns = vec![
            vec![ 0,7,3, 8,9,4, 5,1 ],
            vec![ 9,1,2, 7,3,5, 4,8 ],
            vec![ 8,4,5, 6,1,2, 9,7 ],
            vec![ 7,9,8, 2,6,1, 3,5 ],
            vec![ 5,2,6, 4,7,3, 8,9 ],
            vec![ 1,3,4, 5,8,9, 2,6 ],
            vec![ 4,6,9, 0,2,8, 7,3 ],
            vec![ 2,8,7, 3,5,6, 1,4 ],
            vec![ 3,5,1, 9,4,7, 6,2 ]
        ];
        SudokuSolver::new(&invalid_board_columns);
    }

    #[test]
    #[should_panic]
    fn constructor_works_invalid_board_invalid_spaces() {
        let invalid_board_spaces = vec![
            vec![ 0,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 9,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 0,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,0 ]
        ];
        SudokuSolver::new(&invalid_board_spaces);
    }

    #[test]
    fn all_spaces_solved_works() {
        let board_with_zeroes = SudokuBoard::new(&vec![
            vec![ 0,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 0,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,0 ]
        ]);
        let board_without_zeroes = SudokuBoard::new(&vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ]);

        assert_eq!(board_with_zeroes.all_spaces_solved(), false);
        assert_eq!(board_without_zeroes.all_spaces_solved(), true);
    }

    #[test]
    fn get_row_works() {
        let valid_board = SudokuBoard::new(&vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ]);

        let mut all_rows: Vec<Vec<u8>> = Vec::new();
        for row_index in 0..=8 {
            all_rows.push(valid_board.get_row(row_index));
        }

        assert_eq!(all_rows, vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ]);
    }

    #[test]
    fn get_column_works() {
        let valid_board = SudokuBoard::new(&vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ]);

        let mut all_columns: Vec<Vec<u8>> = Vec::new();
        for column_index in 0..=8 {
            all_columns.push(valid_board.get_column(column_index));
        }

        assert_eq!(all_columns, vec![
            vec![ 6,9,8, 7,5,1, 4,2,3 ],
            vec![ 7,1,4, 9,2,3, 6,8,5 ],
            vec![ 3,2,5, 8,6,4, 9,7,1 ],
            vec![ 8,7,6, 2,4,5, 1,3,9 ],
            vec![ 9,3,1, 6,7,8, 2,5,4 ],
            vec![ 4,5,2, 1,3,9, 8,6,7 ],
            vec![ 5,4,9, 3,8,2, 7,1,6 ],
            vec![ 1,8,7, 5,9,6, 3,4,2 ],
            vec![ 2,6,3, 4,1,7, 5,9,8 ]
        ]);
    }

    #[test]
    fn get_nonet_works() {
        let valid_board = SudokuBoard::new(&vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ]);

        let mut all_nonets: Vec<Vec<u8>> = Vec::new();
        for nonet_index in 0..=8 {
            all_nonets.push(valid_board.get_nonet(nonet_index));
        }

        assert_eq!(all_nonets, vec![
            vec![ 6,7,3, 9,1,2, 8,4,5 ],
            vec![ 8,9,4, 7,3,5, 6,1,2 ],
            vec![ 5,1,2, 4,8,6, 9,7,3 ],
            vec![ 7,9,8, 5,2,6, 1,3,4 ],
            vec![ 2,6,1, 4,7,3, 5,8,9 ],
            vec![ 3,5,4, 8,9,1, 2,6,7 ],
            vec![ 4,6,9, 2,8,7, 3,5,1 ],
            vec![ 1,2,8, 3,5,6, 9,4,7 ],
            vec![ 7,3,5, 1,4,9, 6,2,8 ]
        ]);
    }

    #[test]
    fn solve_easy_works() {
        let valid_board = vec![
            vec![ 0,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 0,0,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 0,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,0 ]
        ];
        
        let solver = SudokuSolver::new(&valid_board);
        let solved_board = solver.solve();

        assert_eq!(solved_board.board_configuration, vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ]);
    }

    #[test]
    fn solve_medium_works() {
        let valid_board = vec![
            vec![ 7,8,0, 4,0,0, 1,2,0 ],
            vec![ 6,0,0, 0,7,5, 0,0,9 ],
            vec![ 0,0,0, 6,0,1, 0,7,8 ],
            vec![ 0,0,7, 0,4,0, 2,6,0 ],
            vec![ 0,0,1, 0,5,0, 9,3,0 ],
            vec![ 9,0,4, 0,6,0, 0,0,5 ],
            vec![ 0,7,0, 3,0,0, 0,1,2 ],
            vec![ 1,2,0, 0,0,7, 4,0,0 ],
            vec![ 0,4,9, 2,0,6, 0,0,7 ]
        ];

        let solver = SudokuSolver::new(&valid_board);
        let solved_board = solver.solve();

        assert_eq!(solved_board.board_configuration, vec![
            vec![ 7,8,5, 4,3,9, 1,2,6 ],
            vec![ 6,1,2, 8,7,5, 3,4,9 ],
            vec![ 4,9,3, 6,2,1, 5,7,8 ],
            vec![ 8,5,7, 9,4,3, 2,6,1 ],
            vec![ 2,6,1, 7,5,8, 9,3,4 ],
            vec![ 9,3,4, 1,6,2, 7,8,5 ],
            vec![ 5,7,8, 3,9,4, 6,1,2 ],
            vec![ 1,2,6, 5,8,7, 4,9,3 ],
            vec![ 3,4,9, 2,1,6, 8,5,7 ]
        ]);
    }

    #[test]
    fn solve_hard_works() {
        let valid_board = vec![
            vec![ 0,0,0, 0,0,0, 0,0,0 ],
            vec![ 0,0,2, 0,0,5, 0,4,0 ],
            vec![ 1,0,8, 0,4,0, 0,0,0 ],
            vec![ 0,0,0, 0,0,0, 4,0,3 ],
            vec![ 0,0,6, 0,5,0, 0,0,1 ],
            vec![ 0,0,0, 0,2,0, 0,0,6 ],
            vec![ 3,0,1, 0,0,0, 0,8,0 ],
            vec![ 2,0,7, 0,0,0, 6,0,0 ],
            vec![ 0,0,0, 0,0,6, 1,3,9 ]
        ];

        let solver = SudokuSolver::new(&valid_board);
        let solved_board = solver.solve();

        assert_eq!(solved_board.board_configuration, vec![
            vec![ 4,3,9, 6,8,2, 7,1,5 ],
            vec![ 6,7,2, 1,3,5, 9,4,8 ],
            vec![ 1,5,8, 7,4,9, 3,6,2 ],
            vec![ 8,1,5, 9,6,7, 4,2,3 ],
            vec![ 7,2,6, 4,5,3, 8,9,1 ],
            vec![ 9,4,3, 8,2,1, 5,7,6 ],
            vec![ 3,6,1, 5,9,4, 2,8,7 ],
            vec![ 2,9,7, 3,1,8, 6,5,4 ],
            vec![ 5,8,4, 2,7,6, 1,3,9 ]
        ]);
    }

    #[test]
    fn solve_caching_works() {
        let valid_board = vec![
            vec![ 0,0,0, 0,0,0, 0,0,0 ],
            vec![ 0,0,2, 0,0,5, 0,4,0 ],
            vec![ 1,0,8, 0,4,0, 0,0,0 ],
            vec![ 0,0,0, 0,0,0, 4,0,3 ],
            vec![ 0,0,6, 0,5,0, 0,0,1 ],
            vec![ 0,0,0, 0,2,0, 0,0,6 ],
            vec![ 3,0,1, 0,0,0, 0,8,0 ],
            vec![ 2,0,7, 0,0,0, 6,0,0 ],
            vec![ 0,0,0, 0,0,6, 1,3,9 ]
        ];

        let solver = SudokuSolver::new(&valid_board);

        let start_first = Instant::now();
        let solved_board_first = solver.solve();
        let end_first = Instant::now();
        let duration_first = end_first.duration_since(start_first).as_millis();

        let start_second = Instant::now();
        let solved_board_second = solver.solve();
        let end_second = Instant::now();
        let duration_second = end_second.duration_since(start_second).as_millis();

        println!("Caching test took {}ms to solve in the first iteration and {}ms in the second iteration.", duration_first, duration_second);
        assert_eq!(solved_board_first.board_configuration, solved_board_second.board_configuration);
        assert!(duration_second < duration_first);
    }
}
