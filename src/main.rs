mod state;

extern crate pancurses;

use pancurses::{curs_set, endwin, initscr, noecho, Input};
use std::env;

#[derive(Debug)]
pub struct EnvConfig {
    pub use_timer: bool,
    pub size: usize,
    pub timeout: i32,
}

// Parse environment variables passed to a program
// TIMER (enabled by default) - setting it to "0" or "false" or "off" will disable timer, other values will enable timer
// SIZE (5 by default) - size of the game board
// TIMEOUT (300 by default) - timeout for timer
pub fn parse_env() -> EnvConfig {
    let use_timer = match env::var("TIMER")
        .unwrap_or_default()
        .to_lowercase()
        .as_str()
    {
        "0" | "false" | "off" => false,
        _ => true,
    };

    let size = env::var("SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&s| s > 1)
        .unwrap_or(5);

    let timeout = env::var("TIMEOUT")
        .ok()
        .and_then(|t| t.parse::<i32>().ok())
        .filter(|&t| t > 1)
        .unwrap_or(300);

    EnvConfig {
        use_timer,
        size,
        timeout,
    }
}

fn main() {
    let config = parse_env();

    let mut state = state::State::new(false, config.size);

    let window = initscr();

    // For handling arrow keys
    window.keypad(true);

    // Don't print inputterd characters
    noecho();

    // Hide cursor
    curs_set(0);

    if config.use_timer {
        window.timeout(config.timeout);
    }

    window.mvprintw(
        0,
        0,
        "Move snake with arrow keys. Press 'q' to quit. Press 'r' to restart\n",
    );

    // Render game board
    window.mvprintw(4, 0, state.to_board_str());

    loop {
        let input = window.getch();
        let output = &format!("Input: {:?}           ", input);
        window.mvprintw(2, 0, output);

        match input {
            Some(Input::KeyUp) => {
                state.on_key_press(state::AllowedKeys::Up);
            }
            Some(Input::KeyDown) => {
                state.on_key_press(state::AllowedKeys::Down);
            }
            Some(Input::KeyLeft) => {
                state.on_key_press(state::AllowedKeys::Left);
            }
            Some(Input::KeyRight) => {
                state.on_key_press(state::AllowedKeys::Right);
            }
            Some(Input::Character(c)) => {
                if c.to_string().to_lowercase() == "q" {
                    // Exit when "q" is pressed
                    break;
                } else if c.to_string().to_lowercase() == "r" {
                    state.on_key_press(state::AllowedKeys::Reset);
                }
            }
            None => {
                if config.use_timer {
                    state.move_snake()
                }
            }
            _ => {
                // Some other key was pressed that we don't care about
            }
        }

        // Render game board
        window.mvprintw(4, 0, state.to_board_str());

        let game_status = if state.did_win {
            "You won         "
        } else {
            "Game in progress"
        };
        window.mvprintw((config.size + 6).try_into().unwrap(), 0, game_status);

        window.mvprintw(
            (config.size + 8).try_into().unwrap(),
            0,
            &format!("Debug output: {:?}     ", state),
        );
    }
    endwin();
}
