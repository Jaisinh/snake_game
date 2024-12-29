use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: VecDeque<Point>,
    food: Point,
    direction: Direction,
    score: u32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            snake: VecDeque::new(),
            food: Point { x: 0, y: 0 },  
            direction: Direction::Right,
            score: 0,
            game_over: false,
        };
        game.snake.push_back(Point { x: 5, y: 5 });
        game.spawn_food(); 
        game
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake.back().unwrap();

        let new_head = match self.direction {
            Direction::Up => Point { x: head.x, y: head.y - 1 },
            Direction::Down => Point { x: head.x, y: head.y + 1 },
            Direction::Left => Point { x: head.x - 1, y: head.y },
            Direction::Right => Point { x: head.x + 1, y: head.y },
        };

        if new_head.x < 0 || new_head.x >= WIDTH as i32 || new_head.y < 0 || new_head.y >= HEIGHT as i32 {
            self.game_over = true;
            return;
        }

        if self.snake.iter().any(|p| p.x == new_head.x && p.y == new_head.y) {
            self.game_over = true;
            return;
        }

        self.snake.push_back(new_head);

        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 1;
            self.spawn_food(); 
        } else {
            self.snake.pop_front();
        }
    }

    fn spawn_food(&mut self) {
        let seed = get_seed();
        loop {
            let x = custom_random(seed, WIDTH as u32) as i32;
            let y = custom_random(seed.wrapping_add(1), HEIGHT as u32) as i32; 

            let new_food = Point { x, y };

            if !self.snake.iter().any(|p| p.x == new_food.x && p.y == new_food.y) {
                self.food = new_food;
                break;
            }
        }
    }

    fn display(&self) -> String {
        let mut display = String::new();

        display.push_str(&format!("Score: {}\n", self.score));
        display.push_str("Controls: w=up, s=down, a=left, d=right, q=quit\n\n");

        display.push_str(&"+".repeat(WIDTH + 2));
        display.push('\n');

        for y in 0..HEIGHT {
            display.push('|');
            for x in 0..WIDTH {
                let point = Point { x: x as i32, y: y as i32 };

                if self.snake.iter().any(|p| p.x == point.x && p.y == point.y) {
                    if point.x == self.snake.back().unwrap().x
                        && point.y == self.snake.back().unwrap().y
                    {
                        display.push('@');
                    } else {
                        display.push('o');
                    }
                } else if self.food.x == point.x && self.food.y == point.y {
                    display.push('*');
                } else {
                    display.push(' ');
                }
            }
            display.push('|');
            display.push('\n');
        }

        display.push_str(&"+".repeat(WIDTH + 2));
        display.push('\n');

        if self.game_over {
            display.push_str("Game Over! Final score: ");
            display.push_str(&self.score.to_string());
            display.push('\n');
        }

        display
    }

    fn handle_input(&mut self, command: &str) {
        self.direction = match (command, &self.direction) {
            ("w", Direction::Down) | ("s", Direction::Up) | ("a", Direction::Right) | ("d", Direction::Left) => self.direction.clone(),
            ("w", _) => Direction::Up,
            ("s", _) => Direction::Down,
            ("a", _) => Direction::Left,
            ("d", _) => Direction::Right,
            _ => self.direction.clone(),
        };
    }
}

fn custom_random(seed: u64, max: u32) -> u32 {
    let mut state = seed;
    state ^= state >> 21;
    state ^= state << 35;
    state ^= state >> 4;
    state = state.wrapping_mul(2685821657736338717);  
    (state as u32) % max
}

fn get_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

fn main() {
    let mut game = Game::new();

    println!("Welcome to Snake!");
    println!("@ = snake head");
    println!("o = snake body");
    println!("* = food");
    println!("Controls: w=up, s=down, a=left, d=right, q=quit");
    println!("Press Enter after each command.");
    println!();

    loop {
        print!("{}", game.display());
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim().to_lowercase();

        if command == "q" {
            break;
        }

        game.handle_input(&command);
        game.update();

        if game.game_over {
            print!("{}", game.display());
            break;
        }
    }

    println!("Thanks for playing!");
}
