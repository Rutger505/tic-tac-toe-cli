use std::io;
use std::io::Write;

#[derive(Debug, PartialEq)]
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

    print_board(board);

    get_user_coordinate();

    println!("Hello, world!");
}

fn get_user_coordinate() {
    let number = get_user_number(1, 3);

    println!("User choose: {number}");
}

fn get_user_number(min: i32, max: i32) -> i32 {
    let mut input = String::new();

    while input.is_empty() || input.parse::<i32>().is_err() {
        input.clear();

        print!("Enter a number in range {min}-{max}: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        input = input.trim_end().to_string();
    }

    input.parse().expect("While loop should validate")
}

fn print_board(board: Vec<Vec<FieldValue>>) {
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
