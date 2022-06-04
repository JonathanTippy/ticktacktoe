use std::io;
fn main() {
    println!("how big board?");
    let board_size = asks_user_for_number();
    let mut board = vec![];
        for _ in 0..(board_size as u16 * board_size as u16) {
            board.push(0)
        }
    println!("how many players?");
    let players = asks_user_for_number();
    let mut characters = vec![];
    characters.push(" ".to_string());
    for player in 0..players {
        println!("please enter a character, player {}",(player + 1));
        characters.push(asks_user_for_character());
    }
    render(&board,board_size,&characters);
    loop {
        let mut board_full:bool = true;
        for player in 0..players {
            let character = &characters[player as usize];
            let mut real_place:u16 = 0;
            loop {
                println!("your move, player {} please enter row followed by collumn",(player + 1));
                let row = asks_user_for_number();
                let collumn = asks_user_for_number();
                if row <= board_size {
                    if collumn <= board_size {
                        let real_row:u16 = (row - 1) as u16 * board_size as u16;
                        real_place = real_row + (collumn as u16 - 1);
                        if board[real_place as usize] == 0 {
                            board.remove(real_place.into());
                            board.insert(real_place.into(), player + 1);
                            println!("{}",real_place);
                            println!("{}",board_size);

                            break
                        }
                        else {
                            println!("space already taken")
                        }
                    }
                    else {
                        println!("collumn is off the board!")
                    }
                }
                else {
                    println!("row is off the board!")
                }
            }
            render(&board,board_size,&characters);
            let win = checks_for_win(&board,board_size,real_place);
            if win {
                println!("congrajilashins player {}, you is winner!",player);
                break
            }
            for spot in &board {
                if *spot != 0 {
                    board_full = board_full & true
                }
                else {
                    board_full = board_full & false
                }
            }
            if board_full {
                println!("game over noone wins");
                break
            }
        }
        if board_full {
            break
        }
    }
}

fn checks_for_win(vector:&Vec<u8>,board_size:u8,last_move:u16) -> bool {
    let mut win = false;
    let mut vertical_win = true;
    let mut horizontal_win = true;
    let mut slash_win = true;
    let mut backslash_win = true;
    for distance in 0..board_size {
        if vector[usize::from((last_move % (board_size as u16)) + board_size as u16 * distance as u16)] == vector[usize::from(last_move)] {
            vertical_win = vertical_win & true
        }
        else {
            vertical_win = vertical_win & false
        }
    }
    for distance in 0..board_size {
        if vector[usize::from((last_move - (last_move % (board_size as u16))) + (distance as u16))] == vector[usize::from(last_move)] {
            horizontal_win = horizontal_win & true
        }
        else {
            horizontal_win = horizontal_win & false
        }
    }
    for distance in 0..board_size {
        if vector[usize::from((((distance as u16) + 1) * (board_size as u16)) - ((distance as u16) + 1))] == vector[usize::from(last_move)] {
            slash_win = slash_win & true
        }
        else {
            slash_win = slash_win & false
        }
    }
    for distance in 0..board_size {
        if vector[usize::from((((distance as u16) + 1) * (board_size as u16)) - ((board_size as u16) - (distance as u16)))] == vector[usize::from(last_move)] {
            backslash_win = backslash_win & true
        }
        else {
            backslash_win = backslash_win & false
        }
    }
    if horizontal_win {
        return true
    }
    if vertical_win {
        return true
    }
    if slash_win {
        return true
    }
    if backslash_win {
        return true
    }
    return false
}

fn render(vector:&Vec<u8>,line_length:u8,characters:&Vec<String>) {
    let mut final_string = String::new();
    for _ in 0..line_length {
        final_string.push_str(format!(" _").as_str())
    }
    final_string.push_str(format!("\n").as_str());
    for place in 0..vector.len() {
        if (place + 1) % (line_length as usize) == 0 {
            final_string.push_str(format!("|̲{}|\n",characters[vector[place] as usize]).as_str());
        }
        else {
            final_string.push_str(format!("|̲{}",characters[vector[place] as usize]).as_str())
        }
    }
    println!("{}",final_string)
}

fn asks_user_for_number() -> u8 {
    loop {
        let mut string_to_receive_input = String::new();
            io::stdin()
                .read_line(&mut string_to_receive_input)
                .expect("Failed to read input");
            let user_input:u8 = match string_to_receive_input.trim().parse::<u8>() {
                Ok(num) => num,
                Err(_) => 0,
            };
        if user_input > 0 {
            return user_input
        }
        else {
            println!("number too big!!!!");
        }
    }
}

fn asks_user_for_character() -> String {
    loop {
        let mut string_to_receive_input = String::new();
            io::stdin()
                .read_line(&mut string_to_receive_input)
                .expect("Failed to read input");
            let user_input:String = string_to_receive_input.trim().to_string();
                if user_input.chars().count() == 1 {
            return user_input
        }
        else {
            println!("Not a character try again")
        }
    }

}
