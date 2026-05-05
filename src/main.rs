use std::io;
use std::io::Write;

#[derive(Debug)]
enum FieldValue {
    Empty,
    Cross,
    Circle,
}

fn main() {
    let mut board = vec![
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty],
        vec![FieldValue::Empty, FieldValue::Empty, FieldValue::Empty]
    ];

    board[0][1] = FieldValue::Cross;

    println!("{:?}", board);

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
