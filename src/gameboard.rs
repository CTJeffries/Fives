// ------------------------------------------------------------------------------------------------

// Colby Jeffries
// gameboard.rs

// This file contains the logic for the gameboard in Fives. The board is 5x5, and can have empty
// slots, or the slots can be occupied by pieces that are 5 or multiples of 5. Also necessary
// functions to handle events for the game are included here as well.

// ------------------------------------------------------------------------------------------------

// Imports from External Dependencies
use rand::{thread_rng, Rng, random};
use piston::input::{Button, GenericEvent, Key};

// Constants
const SIZE: usize = 5;

// ------------------------------------------------------------------------------------------------

// Gameboard structure. Stores all necessary gameboard logic information.
pub struct Gameboard {
    // 2 Dimensional array that stores the cells. A 0 represents an empty cell.
    pub cells: [[u64; SIZE]; SIZE],
    // Score, the sum of all pieces on the board.
    pub score: u64,
}

impl Gameboard {
    // Initializer. Gameboard is initialized with all 0s.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
            score: 0,
        }
    }

    // Called when a new game is started. Clear the board, initialize 4 random cells.
    pub fn new_game(&mut self) {
        self.cells = [[0; SIZE]; SIZE];
        self.score = 0;
        let mut generated_cells = 0;
        // Select four random cells, and if they are empty put in 5 or 10, if they are not empty,
        // select another random cell.
        while generated_cells < 4 {
            let x = random::<usize>() % 5;
            let y = random::<usize>() % 5;
            let val = thread_rng().gen_range(1, 3) * 5;
            if self.cells[y][x] == 0 {
                self.set([x, y], val);
                generated_cells += 1
            }
        }
    }

    // Shift the board one tile left. Merge all merges, if no merges can happen, do not allow
    // shift. Generate a new random cell on the right side of the board.
    pub fn shift_board_up(&mut self) {
        let mut success  = false;
        for i in 0..SIZE {
            for j in 0..(SIZE - 1) {
                let allowed = self.merge([i, j + 1], [i, j]);
                if allowed {
                    success = true;
                }
            }
        }
        if success {
            let val = self.new_val();
            let mut available = vec![];
            for i in 0..SIZE {
                if self.cells[SIZE - 1][i] == 0 {
                    available.push(i)
                }
            }
            if !available.is_empty() {
                let x = thread_rng().choose(&available);
                if let Some(new_index) = x {
                    self.set([*new_index, SIZE - 1], val);
                }
            }
        }
    }

    // Shift the board one tile right. Merge all merges, if no merges can happen, do not allow
    // shift. Generate a new random cell on the left side of the board.
    pub fn shift_board_down(&mut self) {
        let mut success = false;
        for i in 0..SIZE {
            for j in (1..SIZE).rev() {
                let allowed = self.merge([i, j - 1], [i, j]);
                if allowed {
                    success = true;
                }
            }
        }
        if success {
            let val = self.new_val();
            let mut available = vec![];
            for i in 0..SIZE {
                if self.cells[0][i] == 0 {
                    available.push(i)
                }
            }
            if !available.is_empty() {
                let x = thread_rng().choose(&available);
                if let Some(new_index) = x {
                    self.set([*new_index, 0], val);
                }
            }
        }
    }

    // Shift the board one tile up. Merge all merges, if no merges can happen, do not allow
    // shift. Generate a new random cell on the bottom side of the board.
    pub fn shift_board_left(&mut self) {
        let mut success = false;
        for i in 0..(SIZE - 1) {
            for j in 0..SIZE {
                let allowed = self.merge([i + 1, j], [i, j]);
                if allowed {
                    success = true;
                }
            }
        }
        if success {
            let val = self.new_val();
            let mut available = vec![];
            for i in 0..SIZE {
                if self.cells[i][SIZE - 1] == 0 {
                    available.push(i)
                }
            }
            if !available.is_empty() {
                let x = thread_rng().choose(&available);
                if let Some(new_index) = x {
                    self.set([SIZE - 1, *new_index], val);
                }
            }
        }
    }

    // Shift the board one tile down. Merge all merges, if no merges can happen, do not allow
    // shift. Generate a new random cell on the top side of the board.
    pub fn shift_board_right(&mut self) {
        let mut success = false;
        for i in (1..SIZE).rev() {
            for j in 0..SIZE {
                let allowed = self.merge([i - 1, j], [i, j]);
                if allowed {
                    success = true;
                }
            }
        }
        if success {
            let val = self.new_val();
            let mut available = vec![];
            for i in 0..SIZE {
                if self.cells[i][0] == 0 {
                    available.push(i)
                }
            }
            if !available.is_empty() {
                let x = thread_rng().choose(&available);
                if let Some(new_index) = x {
                    self.set([0, *new_index], val);
                }
            }
        }
    }

    // Attempt a merge. If merge is possible, set value of new cell, shift all other cells
    // over, and return true. If merge is not possible, return false.
    fn merge(&mut self, send_cell: [usize; 2], recieve_cell: [usize; 2]) -> bool {
        let s_val = self.cells[send_cell[1]][send_cell[0]];
        let r_val = self.cells[recieve_cell[1]][recieve_cell[0]];
        if ((s_val != r_val) && (r_val != 0)) || s_val == 0 {
            return false
        }
        else {
            self.set(recieve_cell, s_val + r_val);
            self.set(send_cell, 0);
            return true
        }
    }

    // Return the string in a cell.
    pub fn get_string(&self, current_cell: [usize; 2]) -> Option<String> {
        if self.cells[current_cell[1]][current_cell[0]] != 0 {
            return Some(self.cells[current_cell[1]][current_cell[0]].to_string())
        }
        else {
            return None
        }

    }

    // Return the value of the cell.
    pub fn get_val(&self, current_cell: [usize; 2]) -> u64 {
        return self.cells[current_cell[1]][current_cell[0]]
    }

    // Set cell value.
    fn set(&mut self, current_cell: [usize; 2], val: u64) {
        self.cells[current_cell[1]][current_cell[0]] = val;
    }

    // Generate a new value for the board.
    fn new_val(&self) -> u64 {
        (2_i32.pow(thread_rng().gen_range(0, 3)) * 5) as u64
    }

    // Get the score as a string.
    pub fn get_score(&self) -> String {
        self.score.to_string()
    }

    // Recalculates the score. Score is simply the sum of all of the pieces on the board, plus the
    // the largest tile multiplied by 5.
    fn update_score(&mut self) {
        let mut score: u64 = 0;
        let mut max: u64 = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                score += self.cells[i][j];
                if self.cells[i][j] > max {
                    max = self.cells[i][j]
                }
            }
        }
        score += max * 5;
        self.score = score;
    }

    // Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        // If a keyboard key is pressed.
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => self.shift_board_up(),
                Key::Down => self.shift_board_down(),
                Key::Left => self.shift_board_left(),
                Key::Right => self.shift_board_right(),
                _ => {}
            }
        }
        self.update_score();
    }
}

// ------------------------------------------------------------------------------------------------
