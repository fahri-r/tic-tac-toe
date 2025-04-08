fn greeting() {
    println!(
        "\nWelcome to the Ultimate Tic-Tac-Toe Showdown!\n\
        Get ready to make your move and claim victory!"
    )
}

fn draw_board(squares: &mut [char; 9]) {
    println!();

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

    println!("-------------");
}

fn ask_player(squares: &mut [char; 9], player: char) {
    loop {
        let mut input = String::new();

        println!("Player '{}' please input the number:", player);

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

fn is_over(squares: &mut [char; 9]) -> bool {
    squares.iter().all(|&square| square == 'X' || square == 'O')
}

fn is_win(squares: &mut [char; 9]) -> bool {
    for i in 0..3 {
        //Check vertical
        if squares[i] == squares[i + 3] && squares[i] == squares[i + 6] {
            return true;
        }

        //Check horizontal
        let i = i * 3;
        if squares[i] == squares[i + 1] && squares[i] == squares[i + 2] {
            return true;
        }
    }

    //Check diagonal
    if (squares[0] == squares[4] && squares[0] == squares[8]) || (squares[2] == squares[4] && squares[2] == squares[6])
    {
        return true;
    }

    return false;
}

fn main() {
    let mut squares = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    greeting();

    loop {
        draw_board(&mut squares);
        ask_player(&mut squares, player);

        if is_win(&mut squares) {
            draw_board(&mut squares);
            println!("Player '{}' Win!", player);
            break;
        }

        if is_over(&mut squares) {
            draw_board(&mut squares);
            println!("All of the fields are filled");
            break;
        }

        //Switch the player
        player = if player == 'X' { 'O' } else { 'X' }; 
    }
}
