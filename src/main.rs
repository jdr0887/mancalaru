#[macro_use]
extern crate log;
extern crate simple_logger;

use log::Level;
use std::fmt::Display;
use std::io;

#[derive(Eq, PartialEq, Debug)]
enum Player {
    ONE,
    TWO,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Player {:?}", self)
    }
}

#[derive(Default, Debug)]
struct Board {
    bins: Vec<i32>,
}

impl Board {
    pub fn new(tokens_per_bin: i32) -> Board {
        let init_bins = vec![tokens_per_bin; 14];
        Board { bins: init_bins }
    }
}

fn prompt_for_starting_cell(player: &Player) -> io::Result<usize> {
    println!("{} - select cell [1-6]: ", player);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let starting_cell = input.trim().parse::<usize>().unwrap();
    Ok(starting_cell)
}

fn main() -> io::Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();

    let mut board = Board::new(4);

    let mut current_player = Player::ONE;
    loop {
        let mut starting_cell = 0;
        loop {
            starting_cell = prompt_for_starting_cell(&current_player)?;

            if starting_cell > 6 {
                warn!("Goofball...this time chose a number LESS THAN 7:");
                continue;
            }

            if current_player == Player::TWO {
                starting_cell = starting_cell + 7;
            }

            let tokens_at_cell = board.bins[starting_cell - 1];
            if tokens_at_cell == 0 {
                warn!("Dummy...this time chose a bin WITH tokens in it:");
            } else {
                break;
            }
        }

        'asdf: loop {
            debug!("starting_cell: {}", starting_cell);

            let mut cell_idx = starting_cell - 1;
            let tokens_at_cell = board.bins[cell_idx];
            debug!("cell_idx: {}: tokens_at_cell: {}, current_player: {}", cell_idx, tokens_at_cell, current_player);
            info!("before move: {:?}", board);

            board.bins[cell_idx] = 0;
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

                board.bins[cell_idx] = board.bins[cell_idx] + 1;
                debug!("cell_idx: {}, move: {:?}", cell_idx, board);

                if i == tokens_at_cell - 1 {
                    if board.bins[cell_idx] == 1 {
                        info!("after move: {:?}", board);
                        debug!("turn over for {}", current_player);
                        match current_player == Player::ONE {
                            true => current_player = Player::TWO,
                            _ => current_player = Player::ONE,
                        }
                        break 'asdf;
                    }

                    if (current_player == Player::TWO && cell_idx == 13) || (current_player == Player::ONE && cell_idx == 6) {
                        info!("last token dropped in score bin...you get to go again");
                        info!("after move: {:?}", board);
                        break 'asdf;
                    }
                }

                cell_idx = cell_idx + 1 as usize;

                if cell_idx > 13 {
                    cell_idx = 0;
                }
            }
            starting_cell = cell_idx;
            info!("after move: {:?}", board);
        }

        let player1_bins = board.bins.clone().drain(0..5).all(|x| x == 0);
        debug!("are player one bins empty: {}", player1_bins);

        let player2_bins = board.bins.clone().drain(7..12).all(|x| x == 0);
        debug!("are player two bins empty: {}", player2_bins);

        if player1_bins || player2_bins {
            info!("game over: {:?}", board);

            let player1_score = board.bins[6];
            let player2_score = board.bins[13];

            match player1_score > player2_score {
                true => println!("Player 1 won!"),
                _ => println!("Player 2 won!"),
            }
            break;
        }
    }

    Ok(())
}
