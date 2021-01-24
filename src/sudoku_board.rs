use itertools::Itertools;

#[derive(Debug)]
pub struct SudokuBoard {
    pub configuration: Vec<Vec<u8>>
}

impl SudokuBoard {
    pub fn new(sudoku_puzzle: &Vec<Vec<u8>>) -> SudokuBoard {
        if sudoku_puzzle.len() != 9 || sudoku_puzzle.iter().any(|row| row.len() != 9) {
            panic!("The board must be 9x9.");
        }

        if sudoku_puzzle.iter().any(|row| row.iter().any(|value| *value > 9)) { // Values will not be negative because `u8` is used
            panic!("All values must be [0..9] inclusive");
        }

        return SudokuBoard {
            configuration: sudoku_puzzle.clone()
        }
    }

    pub fn copy(other: &SudokuBoard) -> SudokuBoard {
        return SudokuBoard {
            configuration: other.configuration.clone()
        }
    }

    pub fn get_unsolved_spaces(&self) -> Vec<(usize, usize)> {
        let mut unsolved_spaces = Vec::new();
        for row in 0..=8 {
            for column in 0..=8 {
                if self.configuration[row][column] == 0 {
                    unsolved_spaces.push((row, column));
                }
            }
        }
        return unsolved_spaces;
    }

    pub fn all_spaces_valid(&self) -> bool {
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

    pub fn all_spaces_solved(&self) -> bool {
        return !self.configuration.iter().any(|row| row.iter().any(|value| *value == 0));
    }

    pub fn get_row(&self, row_index: usize) -> Vec<u8> {
        let mut row = Vec::new();
        for column_index in 0..=8 {
            row.push(self.configuration[row_index][column_index]);
        }
        return row;
    }

    pub fn get_column(&self, column_index: usize) -> Vec<u8> {
        let mut column = Vec::new();
        for row_index in 0..=8 {
            column.push(self.configuration[row_index][column_index]);
        }
        return column;
    }

    pub fn get_nonet(&self, nonet_index: usize) -> Vec<u8> {
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
                nonet.push(self.configuration[row_index][column_index]);
            }
        }
        return nonet;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constructor_works_valid_board() {
        let valid_configuration = vec![
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

        let valid_board = SudokuBoard::new(&valid_configuration);

        assert_eq!(valid_board.configuration, valid_configuration);
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
        SudokuBoard::new(&invalid_board_rows);
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
        SudokuBoard::new(&invalid_board_columns);
    }

    #[test]
    #[should_panic]
    fn constructor_works_invalid_board_invalid_value() {
        let invalid_board_value = vec![
            vec![ 0,0,0, 0,0,0, 0,0,0 ],
            vec![ 0,0,2, 0,0,5, 0,4,0 ],
            vec![ 1,0,8, 0,4,0, 0,0,0 ],
            vec![ 0,0,0, 0,0,0, 4,0,3 ],
            vec![ 0,0,6, 0,5,0, 0,0,10 ],
            vec![ 0,0,0, 0,2,0, 0,0,6 ],
            vec![ 3,0,1, 0,0,0, 0,8,0 ],
            vec![ 2,0,7, 0,0,0, 6,0,0 ],
            vec![ 0,0,0, 0,0,6, 1,3,9 ]
        ];
        SudokuBoard::new(&invalid_board_value);
    }

    #[test]
    fn get_unsolved_spaces_works() {
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

        let unsolved_spaces = board_with_zeroes.get_unsolved_spaces();

        assert_eq!(unsolved_spaces, vec![
            (0, 0),
            (6, 3),
            (8, 8)
        ]);
    }

    #[test]
    fn all_spaces_valid_works() {
        let invalid_board_spaces = vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 9,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ];
        let valid_board_spaces = vec![
            vec![ 6,7,3, 8,9,4, 5,1,2 ],
            vec![ 9,1,2, 7,3,5, 4,8,6 ],
            vec![ 8,4,5, 6,1,2, 9,7,3 ],
            vec![ 7,9,8, 2,6,1, 3,5,4 ],
            vec![ 5,2,6, 4,7,3, 8,9,1 ],
            vec![ 1,3,4, 5,8,9, 2,6,7 ],
            vec![ 4,6,9, 1,2,8, 7,3,5 ],
            vec![ 2,8,7, 3,5,6, 1,4,9 ],
            vec![ 3,5,1, 9,4,7, 6,2,8 ]
        ];

        let invalid_board = SudokuBoard::new(&invalid_board_spaces);
        let valid_board = SudokuBoard::new(&valid_board_spaces);

        assert_eq!(invalid_board.all_spaces_valid(), false);
        assert_eq!(valid_board.all_spaces_valid(), true);
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
}