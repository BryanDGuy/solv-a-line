use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::sudoku_board::SudokuBoard;

pub struct SudokuSolver {
    pub board: SudokuBoard,
    pub unsolved_spaces: Vec<(usize, usize)>,
    pub percent_solved: f32,
    solved_board: RefCell<Option<SudokuBoard>>
}

impl SudokuSolver {
    pub fn new(sudoku_board: &SudokuBoard) -> SudokuSolver {
        if !sudoku_board.all_spaces_valid() {
            panic!("An invalid starting board configuration was passed.");
        }

        let unsolved_spaces = sudoku_board.get_unsolved_spaces();
        let unsolved_length: f32 = unsolved_spaces.len() as f32;

        return SudokuSolver {
            board: SudokuBoard::copy(sudoku_board),
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
        let mut solved_board = SudokuBoard::copy(&self.board);
        let mut attempted_values: HashMap<(usize, usize), Vec<u8>> = HashMap::new();
        let mut unsolved_spaces_index = 0;

        while !solved_board.all_spaces_solved() {
            let row_index = self.unsolved_spaces[unsolved_spaces_index].0;
            let column_index = self.unsolved_spaces[unsolved_spaces_index].1;
            let nonet_index = 3 * ((9 * row_index + column_index) / 27) + ((9 * row_index + column_index) / 3 % 3);

            solved_board.configuration[row_index][column_index] = 0; // Set back to 0 in the case this was a back-tracked space
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
                solved_board.configuration[row_index][column_index] = *valid_value_candidates[0];
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
        let valid_board = SudokuBoard::new(&vec![
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
        
        let solver = SudokuSolver::new(&valid_board);
        assert_eq!(solver.board.configuration, valid_board.configuration);
        assert_eq!(solver.unsolved_spaces, vec![
            (0, 0),
            (6, 3),
            (8, 8)
        ]);
        assert_eq!(solver.percent_solved, 96.296295);
        assert_eq!(solver.solved_board.into_inner().is_none(), true);
    }

    #[test]
    #[should_panic]
    fn constructor_works_invalid_board() {
        let invalid_board_spaces = SudokuBoard::new(&vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 9,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ]);

        SudokuSolver::new(&invalid_board_spaces);
    }

    #[test]
    fn solve_easy_works() {
        let valid_board = SudokuBoard::new(&vec![
            vec![ 0,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 0,0,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 0,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,0 ]
        ]);
        
        let solver = SudokuSolver::new(&valid_board);
        let solved_board = solver.solve();

        assert_eq!(solved_board.configuration, vec![
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
        let valid_board = SudokuBoard::new(&vec![
            vec![ 7,8,0, 4,0,0, 1,2,0 ],
            vec![ 6,0,0, 0,7,5, 0,0,9 ],
            vec![ 0,0,0, 6,0,1, 0,7,8 ],
            vec![ 0,0,7, 0,4,0, 2,6,0 ],
            vec![ 0,0,1, 0,5,0, 9,3,0 ],
            vec![ 9,0,4, 0,6,0, 0,0,5 ],
            vec![ 0,7,0, 3,0,0, 0,1,2 ],
            vec![ 1,2,0, 0,0,7, 4,0,0 ],
            vec![ 0,4,9, 2,0,6, 0,0,7 ]
        ]);

        let solver = SudokuSolver::new(&valid_board);
        let solved_board = solver.solve();

        assert_eq!(solved_board.configuration, vec![
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
        let valid_board = SudokuBoard::new(&vec![
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

        let solver = SudokuSolver::new(&valid_board);
        let solved_board = solver.solve();

        assert_eq!(solved_board.configuration, vec![
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
        let valid_board = SudokuBoard::new(&vec![
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
        assert_eq!(solved_board_first.configuration, solved_board_second.configuration);
        assert!(duration_second < duration_first);
    }
}