 use std::io;

use thiserror::Error; // 1.0.40

#[derive(Error, Debug)]
pub enum GameError {
    #[error("this card is not available")]
    CardDoesNotExist,
    #[error("this card has no factors and cannot be taken")]
    NoFactors,
}

pub struct GameState {
    pub player_pts: u8,
    pub enemy_pts: u8,
    pub cards: Vec<u8>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player_pts: 0,
            enemy_pts: 0,
            cards: (1..=23).collect(),
        }
    }
    
    pub fn play(&mut self, player_card: u8) -> Result<(), GameError> {
        if player_card < 1 || player_card > 23 || !self.cards.contains(&player_card) {
            return Err(GameError::CardDoesNotExist);
        }
        let mut factors = (1..player_card).into_iter().filter(|&x| player_card % x == 0).collect::<Vec<u8>>();
        factors.retain(|x| self.cards.contains(x));
        if factors.is_empty() {
            return Err(GameError::NoFactors);
        } else {
            self.player_pts += player_card;
            self.cards.retain(|&x| x != player_card);
            for factor in factors {
                self.cards.retain(|&x| x != factor);
                self.enemy_pts += factor;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut input = String::new();
    let mut game_state = GameState::new();
    loop {
        println!("Which card will you pick?");

        io::stdin().read_line(&mut input).expect("Failed to read input.");
        if let Ok(x) = input.trim().parse::<u8>() {
            if let Err(e) = game_state.play(x) {
                eprintln!("{:?}", e);
            }
            println!("Your pts: {}\nEnemy pts: {}\nCards left in deck: {:?}", game_state.player_pts, game_state.enemy_pts, game_state.cards);
        } else {
            if input.trim() == "q" {
                println!("Final point total:\nPlayer: {}\nEnemy: {}", game_state.player_pts, game_state.enemy_pts);
                if game_state.player_pts > game_state.enemy_pts {
                    println!("Player wins!");
                } else {
                    println!("Enemy wins!");
                }
                break;
            } else {
                println!("invalid input");
            }
        };

        input.clear();

    }
}