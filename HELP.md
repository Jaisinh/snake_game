### Start the Game:
Run the game by executing the following:

```bash
cargo run
```

### Movement:
Use the following keys to control the direction of the snake:

- `w` to move up.
- `s` to move down.
- `a` to move left.
- `d` to move right.

### Game Over:
The game ends if the snake collides with the boundaries of the game board or itself.

### Quit:
Press `q` to quit the game at any time.

### Score:
Your score increases each time you successfully eat food (`*`). The game will show your current score at the top.

---

# Controls

- `w`: Move Up
- `s`: Move Down
- `a`: Move Left
- `d`: Move Right
- `q`: Quit the game

---

# How to Play
- When the game starts, the snake will appear in the center of the screen.
- The snake grows longer as it eats food, which randomly spawns on the grid.
- Avoid hitting walls or the snake's body; otherwise, the game will end.
- The score increases each time the snake eats food.

---

# Dependencies
This game is built in pure Rust with no external dependencies. The random food placement is done using `SystemTime` to ensure randomness without the need for external crates.

---

# Building and Running the Game
To build and run the game, follow these steps:

1. Clone or download the repository.
```bash
git clone https://github.com/Jaisinh/snake_game
```
2. Open a terminal in the project folder.
3. Run the following command to build and execute the game:

```bash
cargo run
```

If you'd like to contribute or modify the game, feel free to fork the repository and make your changes!

---

# License
This project is licensed under the MIT License.
