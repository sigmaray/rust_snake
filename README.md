# Rust Snake Game

This is a simple snake game implemented in Rust using the `pancurses` library for terminal-based graphics.

## Features

* Move the snake using arrow keys
* Automatic movement with a configurable timeout
* Random food generation
* You win when snake fills all game board cells
* After you win, you can restart the game by pressing 'r' key
* The snake can intersect with itself (this was implemented intentionally)
* The snake can go beyond the playing field (this was implemented intentionally)

## Requirements

Rust and Cargo installed

## Installation

1. Clone the repository
2. Install deps: `apt-get update && apt-get install libncurses5-dev libncursesw5-dev`
2. Compile and run the game: `cargo run`

## Docker

You can also run the game using Docker. First, build the Docker image:
```
docker build -t rust_snake .
```

Then, run the Docker container:
```
docker run -it rust_snake
```

Or run with command:
```
docker run -it "$(docker build -q .)"
```

## Usage

- Use the arrow keys to change the direction of the snake.
- The snake will move automatically if `USE_TIMER` is set to true.

# Environment Variables

You can configure the game using the following environment variables:

* TIMER (enabled by default) - setting it to "0", "false", or "off" will disable the timer; other values will enable the timer
* SIZE (default: 5) - size of the game board
* TIMEOUT (default: 300) - timeout for the timer in milliseconds
Examples:
```
TIMER=off SIZE=10 cargo run
```
```
TIMEOUT=500 cargo run
```

# Demo

![til](https://raw.githubusercontent.com/sigmaray/rust_snake/master/rust_snake.png)
