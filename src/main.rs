extern crate pancurses;

use pancurses::{curs_set, endwin, initscr, noecho, Input};

// Size of game board in both directions (x, y)
const BOARD_SIZE: usize = 10;

// Move the snake every TIMEOUT_IN_MS milliseconds or manually with arrow keys
const USE_TIMER: bool = true;

// Timeout in milliseconds for moving the snake
const TIMEOUT_IN_MS: i32 = 300;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
}

impl State {
    fn change_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.y > 0 {
                    self.y -= 1;
                } else {
                    self.y = BOARD_SIZE - 1
                }
            }
            Direction::Down => {
                if self.y < BOARD_SIZE - 1 {
                    self.y += 1;
                } else {
                    self.y = 0
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    self.x -= 1;
                } else {
                    self.x = BOARD_SIZE - 1
                }
            }
            Direction::Right => {
                if self.x < BOARD_SIZE - 1 {
                    self.x += 1;
                } else {
                    self.x = 0
                }
            }
        }
    }

    // Generate an empty board (2d vector filled with "_")
    fn gen_empty_board_vec(&self) -> Vec<Vec<&'static str>> {
        let mut board = vec![vec![""; BOARD_SIZE]; BOARD_SIZE];
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = "_";
            }
        }
        board
    }

    // Convert state into the board (2d array with characters)
    pub fn to_board_vec(&self) -> Vec<Vec<&str>> {
        // Generate an empty board
        let mut board = self.gen_empty_board_vec();

        // Place the food on the board
        board[self.y][self.x] = "*";
        board
    }

    // Convert state into string that renders the board
    pub fn to_board_str(&self) -> String {
        let board = self.to_board_vec();
        let mut board_str = String::new();

        for row in board.iter() {
            for cell in row.iter() {
                board_str.push_str(cell);
            }
            board_str.push_str("\n");
        }

        board_str
    }
}

fn main() {
    let mut state = State {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };

    let window = initscr();

    // For handling arrow keys
    window.keypad(true);

    // Don't print inputterd characters
    noecho();

    // Hide cursor
    curs_set(0);

    window.printw("Type things. Press 'q' to quit\n");

    // Render game board
    window.mvprintw(2, 0, state.to_board_str());

    window.timeout(TIMEOUT_IN_MS);

    loop {
        let input = window.getch();
        let output = &format!("Input: {:?}           ", input);
        window.mvprintw(1, 0, output);

        match input {
            Some(Input::KeyUp) => {
                state.change_direction(Direction::Up);
                if !USE_TIMER {
                    state.move_forward()
                }
            }
            Some(Input::KeyDown) => {
                state.change_direction(Direction::Down);
                if !USE_TIMER {
                    state.move_forward()
                }
            }
            Some(Input::KeyLeft) => {
                state.change_direction(Direction::Left);
                if !USE_TIMER {
                    state.move_forward()
                }
            }
            Some(Input::KeyRight) => {
                state.change_direction(Direction::Right);
                if !USE_TIMER {
                    state.move_forward()
                }
            }
            Some(Input::Character(c)) => {
                if c.to_string().to_lowercase() == "q" {
                    // Exit when "q" is pressed
                    break;
                }
            }
            _ => {
                if USE_TIMER {
                    state.move_forward()
                }
            }
        }

        // Render game board
        window.mvprintw(2, 0, state.to_board_str());

        window.mvprintw(
            (BOARD_SIZE + 3).try_into().unwrap(),
            0,
            &format!("state: {:?}     ", state),
        );
    }
    endwin();
}
