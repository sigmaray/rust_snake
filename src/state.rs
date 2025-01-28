use rand::Rng;

pub const CELL_EMPTY: &str = "-";
pub const CELL_SNAKE_HEAD: &str = "o";
pub const CELL_SNAKE_TAIL: &str = "*";
pub const CELL_FOOD: &str = "@";

// Coordinates of food and snake segments
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

// Direction of snake
#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum AllowedKeys {
    Up,
    Down,
    Left,
    Right,
    Reset,
}

// Game state
#[derive(Debug)]
pub struct State {
    pub board_size: usize,
    pub use_timer: bool,
    pub snake: Vec<Point>,
    pub food: Point,
    pub direction: Direction,
    pub did_win: bool,
}

impl State {
    pub fn new(use_timer: bool, board_size: usize) -> State {
        let mut state = State {
            board_size: board_size,
            use_timer: use_timer,
            snake: vec![Point { x: 0, y: 0 }],
            food: Point { x: 0, y: 0 },
            direction: Direction::Right,
            did_win: false,
        };
        state.food = state.gen_random_food_position();
        state
    }

    // Render game state into 2d array that represents game board
    pub fn to_board_vec(&self) -> Vec<Vec<&str>> {
        // Generate 2d array that represents empty game board
        fn gen_empty_board(board_size: usize) -> Vec<Vec<&'static str>> {
            let mut board = vec![vec![""; board_size]; board_size];
            for row in board.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = CELL_EMPTY;
                }
            }
            board
        }

        // Generate empty game board
        let mut board = gen_empty_board(self.board_size);

        // Place the food on the board
        board[self.food.y][self.food.x] = CELL_FOOD;

        // Place the snake on the board
        for (_i, s) in self.snake.iter().enumerate() {
            board[s.y][s.x] = CELL_SNAKE_TAIL; // Body of the snake
        }

        let head = self.snake[0];
        board[head.y][head.x] = CELL_SNAKE_HEAD; // Head of the snake

        board
    }

    // Render game board into multiline string that can be printed to the terminal
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

    // Change direction of snake after user input if direction is not forbidden
    pub fn change_direction(&mut self, new_direction: Direction) -> bool {
        if self.did_win {
            return false;
        }

        if self.direction == Direction::Up && new_direction == Direction::Down
            || self.direction == Direction::Down && new_direction == Direction::Up
            || self.direction == Direction::Left && new_direction == Direction::Right
            || self.direction == Direction::Right && new_direction == Direction::Left
        {
            // These switches are forbidden
            return false;
        }

        self.direction = new_direction;
        return true;
    }

    pub fn on_key_press(&mut self, key: AllowedKeys) {
        let mut can_move = false;
        match key {
            AllowedKeys::Up => {
                can_move = self.change_direction(Direction::Up);
            }
            AllowedKeys::Down => {
                can_move = self.change_direction(Direction::Down);
            }
            AllowedKeys::Left => {
                can_move = self.change_direction(Direction::Left);
            }
            AllowedKeys::Right => {
                can_move = self.change_direction(Direction::Right);
            }
            AllowedKeys::Reset => {
                *self = State::new(self.use_timer, self.board_size);
            }
        }

        if can_move && !self.use_timer {
            self.move_snake()
        }
    }

    // Move game snake per one step (after timer was triggerd or user input was received)
    pub fn move_snake(&mut self) {
        if self.did_win {
            return;
        }

        let head = self.snake[0];
        let mut new_head = head;

        match self.direction {
            Direction::Up => {
                if head.y > 0 {
                    new_head.y -= 1;
                } else {
                    new_head.y = self.board_size - 1;
                }
            }
            Direction::Down => {
                if head.y < self.board_size - 1 {
                    new_head.y += 1;
                } else {
                    new_head.y = 0;
                }
            }
            Direction::Left => {
                if head.x > 0 {
                    new_head.x -= 1;
                } else {
                    new_head.x = self.board_size - 1;
                }
            }
            Direction::Right => {
                if head.x < self.board_size - 1 {
                    new_head.x += 1;
                } else {
                    new_head.x = 0;
                }
            }
        }

        self.snake.insert(0, new_head);

        if new_head.x != self.food.x || new_head.y != self.food.y {
            self.snake.pop();
        } else {
            if self.snake.len() == self.board_size * self.board_size {
                self.did_win = true;
                return;
            }
            self.food = self.gen_random_food_position();
        }
    }

    // Generate random position for food on the game board that is not occupied by the snake
    fn gen_random_food_position(&self) -> Point {
        let board = self.to_board_vec();
        let mut free_cells = vec![];

        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == CELL_EMPTY {
                    free_cells.push(Point { x, y });
                }
            }
        }

        let random_index = rand::rng().random_range(0..free_cells.len());
        free_cells[random_index]
    }
}
