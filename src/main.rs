extern crate piston_window;

use piston_window::*;

const SQUARE_SIZE: f64 = 25.0;
const COLUMNS: i32 = 10;
const ROWS: i32 = 20;
const FALL_SPEED: f64 = 1.0; // Blocks fall 1 row per second

#[derive(Clone, Copy)]
struct Square {
    x: i32,
    y: i32,
}

struct Tetromino {
    squares: [Square; 4],
}

impl Tetromino {
    fn new(squares: [Square; 4]) -> Tetromino {
        Tetromino { squares }
    }

    fn rotate(&mut self, clockwise: bool) {
        let pivot = self.squares[1];
        for i in 0..4 {
            let x = self.squares[i].x - pivot.x;
            let y = self.squares[i].y - pivot.y;

            if clockwise {
                self.squares[i].x = pivot.x + y;
                self.squares[i].y = pivot.y - x;
            } else {
                self.squares[i].x = pivot.x - y;
                self.squares[i].y = pivot.y + x;
            }
        }
    }

    fn fall(&mut self) {
        for square in &mut self.squares {
            square.y += 1;
        }
    }
}

fn draw_square(square: &Square, color: [f32; 4], context: &Context, graphics: &mut G2d) {
    let x = (square.x as f64) * SQUARE_SIZE;
    let y = (square.y as f64) * SQUARE_SIZE;

    rectangle(
        color,
        [x, y, SQUARE_SIZE, SQUARE_SIZE],
        context.transform,
        graphics,
    );
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tetris", [SQUARE_SIZE * COLUMNS as f64, SQUARE_SIZE * ROWS as f64])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tetromino = Tetromino::new([
        Square { x: 4, y: 0 },
        Square { x: 5, y: 0 },
        Square { x: 6, y: 0 },
        Square { x: 6, y: 1 },
    ]);

    let mut elapsed_time = 0.0;

    while let Some(event) = window.next() {
        if let Some(args) = event.update_args() {
            elapsed_time += args.dt;
            if elapsed_time >= 1.0 / FALL_SPEED {
                tetromino.fall();
                elapsed_time = 0.0;
            }
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);

            for square in &tetromino.squares {
                draw_square(square, [1.0, 1.0, 1.0, 1.0], &context, graphics);
            }
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => {
                    tetromino.rotate(true);
                }
                _ => (),
            }
        }
    }
}
