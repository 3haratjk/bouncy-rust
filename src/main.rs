use std::fmt;
use std::fmt::{Display, Formatter, Error};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: i32,
    height: i32,
}

struct Game {
    ball: Ball,
    frame: Frame,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 63,
                height: 31,
            },
            ball: Ball {
                x: 20,
                y: 10,
                horiz_dir: HorizDir::Right,
                vert_dir: VertDir::Down,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x >= frame.width {
            self.horiz_dir = HorizDir::Left;
        }
        if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y >= frame.height {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.vert_dir {
            VertDir::Down => self.y += 1,
            VertDir::Up => self.y -= 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "x");
        for _ in 0..self.frame.width {
            write!(fmt, "-");
        }
        write!(fmt, "x\n");

        for y in 0..self.frame.height {
            write!(fmt, "|");
            for x in 0..self.frame.width {
                if self.ball.x == x && self.ball.y == y {
                    write!(fmt, "0");
                } else {
                    write!(fmt, " ");
                }
            }
            write!(fmt, "|\n");
        }

        write!(fmt, "x");
        for _ in 0..self.frame.width {
            write!(fmt, "-");
        }
        write!(fmt, "x\n");

        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    let sleep_time = std::time::Duration::from_millis(33);

    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_time);
        println!("{} {}", game.ball.x, game.ball.y);
    }
}
