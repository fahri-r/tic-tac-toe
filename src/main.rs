use std::io;

fn greeting() {
    println!(
        "\nWelcome to the Ultimate Tic-Tac-Toe Showdown!\n\
        Get ready to make your move and claim victory!"
    )
}

fn draw_board(squares: &mut [char; 9]) {
    print!("\n");
    for i in 0..3 {
        let offset = i * 3;

        print!("-------------\n| ");
        print!("{}", squares[offset]);
        print!(" | ");
        print!("{}", squares[offset + 1]);
        print!(" | ");
        print!("{}", squares[offset + 2]);
        println!(" |");
    }
    print!("-------------\n");
}

fn main() {
    let mut squares = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    greeting();

    draw_board(&mut squares);


    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        println!("Couldn't read line! Try again.");
    }

    if let Ok(number) = input.trim().parse::<usize>() {
        if number < 1 || number > 9 {
            println!("have to be between 1 to 9 {}", input);
            return;
        }

        let index = number - 1;
        
        squares[index] = 'X';

    } else {
        println!("only number allowed");
    };

    draw_board(&mut squares);
}
