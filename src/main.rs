use std::io;

fn greeting() {
    println!(
        "\nWelcome to the Ultimate Tic-Tac-Toe Showdown!\n\
        Get ready to make your move and claim victory!"
    )
}

fn draw_board(squares: &mut [char; 9]) {
    println!("\n");

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

fn ask_player(squares: &mut [char; 9], player: char) {
    loop {
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read the line! Try again.");
        }

        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The number must be between 1 and 9");
                continue;
            }

            let index = number - 1;

            if squares[index] == 'X' || squares[index] == 'O' {
                println!("This field is already taken by {}", squares[index]);
                continue;
            }
            
            //Assign value to square
            squares[index] = player;
            break;

        } else {
            println!("Only number allowed");
            continue;
        };
    }
}

fn main() {
    let mut squares = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    greeting();

    loop {
        draw_board(&mut squares);
        ask_player(&mut squares, player);

        //Switch the player
        player = if player == 'X' { 'O' } else { 'X' }; 
    }
}
