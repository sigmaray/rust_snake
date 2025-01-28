# Rust Snake Game

This is a simple snake game implemented in Rust using the `pancurses` library for terminal-based graphics.

## Features

- Move the snake using arrow keys
- Automatic movement with a configurable timeout

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
