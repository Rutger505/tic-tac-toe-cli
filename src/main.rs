use std::io;
use std::io::Write;

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
    let mut board = vec![
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
    ];

    board[0][1] = FieldValue::Cross;

    let game_won = false;

    let total_cells = 3 * 3;
    let mut cells_filled = 0;
    while !game_won && cells_filled < total_cells {
        let field_value = if cells_filled % 2 == 0 {
            FieldValue::Cross
        } else {
            FieldValue::Circle
        };

        print_board(&board);

        let (mut x, mut y) = get_user_coordinate();

        while board[x - 1][y - 1] != FieldValue::Empty {
            println!("Location taken!");
            (x, y) = get_user_coordinate();
        }

        board[x - 1][y - 1] = field_value;

        cells_filled += 1;
    }

    println!("Hello, world!");
}

fn get_user_coordinate() -> (usize, usize) {
    (get_user_number("Enter row number", 1, 3).try_into().unwrap(),
     get_user_number("Enter column number", 1, 3).try_into().unwrap())
}

fn get_user_number(prompt: &str, min: i32, max: i32) -> i32 {
    let mut number= "".parse::<i32>();

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
