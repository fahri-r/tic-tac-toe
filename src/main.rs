fn greeting() {
    println!(
        "\nWelcome to the Ultimate Tic-Tac-Toe Showdown!\n\
        Get ready to make your move and claim victory!\n"
    )
}

fn draw_board(squares: [char; 9]) {
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
    print!("-------------\n| ");
}

fn main() {
    let squares = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    greeting();

    draw_board(squares);
}
