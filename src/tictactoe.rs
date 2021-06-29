pub struct TicTacToeGame {
    current_players_turn: Fields,
    pub board: [[Fields; 3]; 3],
    row_container: [i8; 3],
    column_container: [i8; 3],
    diagonal_container: i8,
    reverse_diagonal_container: i8,
    pub winner: Fields,
    pub num_of_moves: u8,
}

// Possible values a field can have: empty, a X or a O
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fields {
    X,
    O,
    Empty,
}

impl TicTacToeGame {
    // Initializing a new empty game with an empty board. Player X will always start;
    pub fn new() -> TicTacToeGame {
        TicTacToeGame {
            current_players_turn: Fields::X,
            board: [[Fields::Empty; 3]; 3],
            row_container: [0; 3],
            column_container: [0; 3],
            diagonal_container: 0,
            reverse_diagonal_container: 0,
            winner: Fields::Empty,
            num_of_moves: 0,
        }
    }

    // Function to determine who's turn it is
    pub fn whos_turn(&self) -> Fields {
        return self.current_players_turn;
    }

    // Function to make a move by giving the coordinates.
    // There are multiple error variants which are being checked before a move is done (NoEmptyFieldLeft, NotAValidField, FieldNotEmpty)
    // This function returns an optional winner (wrapped in an Option) or the error type.
    pub fn make_move(
        &mut self,
        coords: (usize, usize),
    ) -> Result<Option<Fields>, MoveNotPossibleError> {
        // Every field already taken so no more gameplay is possible
        if self.num_of_moves > 9 {
            return Err(MoveNotPossibleError::NoEmptyFieldLeft);
        }

        // Given coordinates are outside of the board and match no valid field
        if coords.0 > 2 || coords.1 > 2 {
            return Err(MoveNotPossibleError::NotAValidField);
        }
        let (x, y) = coords;
        if self.board[x][y] == Fields::Empty {
            self.board[x][y] = match self.current_players_turn {
                Fields::X => Fields::X,
                _ => Fields::O,
            };

            // Check if after the new move somebody won the game
            let result = self.check_winner(x, y);
            self.num_of_moves += 1;
            self.current_players_turn = match self.current_players_turn {
                Fields::X => Fields::O,
                _ => Fields::X,
            };
            return Ok(result);
        } else {
            // Field has already been taken by a player
            return Err(MoveNotPossibleError::FieldNotEmpty);
        }
    }

    // Function to determine wheter there is a winner by using the container algorithm. For every row and column and both diagonals you count the number of matching elements. If in one container there are more than 2 matching elements, there is a winner.
    fn check_winner(&mut self, x: usize, y: usize) -> Option<Fields> {
        if self.current_players_turn == Fields::X {
            self.row_container[x] += 1;
            self.column_container[y] += 1;
            if x == y {
                self.diagonal_container += 1;
                if self.diagonal_container > 2 {
                    self.winner = Fields::X;
                }
            }
            if x + y == 2 {
                self.reverse_diagonal_container += 1;
                if self.reverse_diagonal_container > 2 {
                    self.winner = Fields::X;
                }
            }
            if self.row_container.iter().any(|element| element > &2) {
                self.winner = Fields::X;
            }
            if self.column_container.iter().any(|element| element > &2) {
                self.winner = Fields::X;
            }
        } else {
            self.row_container[x] -= 1;
            self.column_container[y] -= 1;
            if x == y {
                self.diagonal_container -= 1;
                if self.diagonal_container < -2 {
                    self.winner = Fields::O;
                }
            }
            if x + y == 2 {
                self.reverse_diagonal_container -= 1;
                if self.reverse_diagonal_container < -2 {
                    self.winner = Fields::O;
                }
            }
            if self.row_container.iter().any(|element| element < &-2) {
                self.winner = Fields::O;
            }
            if self.column_container.iter().any(|element| element < &-2) {
                self.winner = Fields::O;
            }
        }
        if self.winner != Fields::Empty {
            println!("Winner is: {:?}!", self.winner);
            return Some(self.winner);
        } else {
            return None;
        }
    }

    // Function for printing out a prettified board as a multiline String
    pub fn format_board(&self) -> String {
        let mut res: String = "".to_string();
        for row in &self.board {
            for element in row {
                match element {
                    Fields::Empty => res.push_str("- "),
                    Fields::X => res.push_str("X "),
                    Fields::O => res.push_str("O "),
                }
            }
            res.pop();
            res.push_str("\n")
        }
        res.pop();
        return res;
    }
}

#[derive(Debug, Clone)]
pub enum MoveNotPossibleError {
    FieldNotEmpty,
    NoEmptyFieldLeft,
    NotAValidField,
}

impl std::fmt::Display for MoveNotPossibleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Move not possible")
    }
}
