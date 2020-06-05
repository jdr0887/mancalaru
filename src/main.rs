use std::io;

#[derive(Eq, PartialEq, Debug)]
enum Player {
    ONE,
    TWO,
}

fn prompt_for_starting_cell(player: &Player) -> io::Result<usize> {
    println!("Player {:?} - select cell [1-6]: ", player);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse::<usize>().unwrap())
}

fn main() {
    let mut bins = vec![4; 14];
    bins[6] = 0;
    bins[13] = 0;

    let mut current_player = Player::ONE;
    let winner = loop {
        let mut starting_cell = loop {
            let mut tmp_cell = prompt_for_starting_cell(&current_player).unwrap();

            if tmp_cell > 6 {
                println!("Goofball...this time choose a number LESS THAN 7:");
                continue;
            }

            if current_player == Player::TWO {
                tmp_cell = tmp_cell + 7;
            }

            let tokens_at_cell = bins[tmp_cell - 1];
            match tokens_at_cell == 0 {
                true => println!("Dummy...this time choose a bin WITH tokens in it:"),
                _ => break tmp_cell,
            }
        };
        'asdf: loop {
            let mut cell_idx = starting_cell - 1;
            let tokens_at_cell = bins[cell_idx];
            println!("board before move: {:?}", bins);

            bins[cell_idx] = 0;
            cell_idx = cell_idx + 1;

            if current_player == Player::ONE && cell_idx == 13 {
                cell_idx = cell_idx + 1 as usize;
            }

            if current_player == Player::TWO && cell_idx == 6 {
                cell_idx = cell_idx + 1 as usize;
            }

            if cell_idx > 13 {
                cell_idx = 0;
            }

            for i in 0..tokens_at_cell {
                if current_player == Player::ONE && cell_idx == 13 {
                    cell_idx = 0;
                }

                if current_player == Player::TWO && cell_idx == 6 {
                    cell_idx = 7;
                }

                bins[cell_idx] = bins[cell_idx] + 1;

                if i == tokens_at_cell - 1 {
                    if bins[cell_idx] == 1 {
                        println!("board after move: {:?}", bins);
                        match current_player == Player::ONE {
                            true => current_player = Player::TWO,
                            _ => current_player = Player::ONE,
                        }
                        break 'asdf;
                    }

                    if (current_player == Player::TWO && cell_idx == 13) || (current_player == Player::ONE && cell_idx == 6) {
                        println!("last token dropped in score bin...you get to go again");
                        println!("board after move: {:?}", bins);
                        break 'asdf;
                    }
                }

                cell_idx = cell_idx + 1 as usize;

                if cell_idx > 13 {
                    cell_idx = 0;
                }
            }
            starting_cell = cell_idx;
            println!("board after move: {:?}", bins);
        }

        if bins.clone().drain(0..5).all(|x| x == 0) || bins.clone().drain(7..12).all(|x| x == 0) {
            match bins[6] > bins[13] {
                true => break Player::ONE,
                _ => break Player::TWO,
            }
        }
    };

    println!("Winner is: {:?}", winner);
}
