use std::io;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use termios::*;

#[derive(PartialEq, Clone, Copy)]
enum FieldValue {
    Empty,
    Cross,
    Circle,
}
impl std::fmt::Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldValue::Empty => write!(f, ""),
            FieldValue::Cross => write!(f, "X"),
            FieldValue::Circle => write!(f, "O"),
        }
    }
}

fn main() {
    let stdin = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin).unwrap();

    // Input, no line buffering
    println!("{}", termios.c_lflag & ICANON);

    return;

    let mut input: [u8; 1] = [0];
    io::stdin().read(&mut input).unwrap();
    println!("{:?}", input);

    return;

    let mut board = vec![
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
    ];

    let mut field_value_won: Option<FieldValue> = None;

    let total_cells = 3 * 3;
    let mut cells_filled = 0;

    while field_value_won.is_none() && cells_filled < total_cells {
        let field_value = if cells_filled % 2 == 0 {
            FieldValue::Cross
        } else {
            FieldValue::Circle
        };

        print_board(&board);

        println!("{}'s Turn!", field_value.to_string().to_uppercase());
        let (mut x, mut y) = get_user_coordinate();
        while board[x - 1][y - 1] != FieldValue::Empty {
            println!("Location taken!");
            (x, y) = get_user_coordinate();
        }

        board[x - 1][y - 1] = field_value;

        field_value_won = get_game_won(&board);

        cells_filled += 1;
    }

    print_board(&board);

    if let Some(won) = field_value_won {
        println!("{} Won!!", won.to_string().to_uppercase());
    } else {
        println!("Tie..");
    }
}

fn get_user_coordinate() -> (usize, usize) {
    (
        get_user_number("Enter row number", 1, 3)
            .try_into()
            .unwrap(),
        get_user_number("Enter column number", 1, 3)
            .try_into()
            .unwrap(),
    )
}

fn get_user_number(prompt: &str, min: i32, max: i32) -> i32 {
    let mut number = "".parse::<i32>();

    loop {
        match number {
            Ok(n) if n >= min && n <= max => break,
            _ => {
                let mut input = String::new();

                print!("{prompt} ({min}-{max}): ");
                io::stdout().flush().unwrap();

                io::stdin().read_line(&mut input).unwrap();
                input = input.trim_end().to_string();

                number = input.parse::<i32>();
            }
        }
    }

    number.unwrap()
}

fn get_game_won(board: &Vec<Vec<FieldValue>>) -> Option<FieldValue> {
    let rows = (0..3).map(|r| [board[r][0], board[r][1], board[r][2]]);
    let cols = (0..3).map(|c| [board[0][c], board[1][c], board[2][c]]);
    let diags = [
        [board[0][0], board[1][1], board[2][2]],
        [board[0][2], board[1][1], board[2][0]],
    ];

    rows.chain(cols)
        .chain(diags)
        .find_map(|line| check_slice(&line))
}

fn check_slice(slice: &[FieldValue]) -> Option<FieldValue> {
    let first = slice[0];
    if first == FieldValue::Empty {
        return None;
    }
    slice[1..].iter().all(|&v| v == first).then_some(first)
}

fn print_board(board: &Vec<Vec<FieldValue>>) {
    for row in 0..3 {
        for column in 0..3 {
            print!(" {:^1}", board[row][column].to_string());
            if column != 2 {
                print!(" |");
            }
        }
        println!();
        if row != 2 {
            println!("-----------");
        }
    }
}
