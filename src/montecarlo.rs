use crate::bitmask_cards::Cards;
use crate::constants::*;
use rayon::prelude::*;

pub struct MonteCarloSimulation {
    my_cards: Cards,
    common_cards: Cards,
    unseen_cards: Cards,
    other_player_count: usize,
    n_rounds: i32,
}

impl MonteCarloSimulation {
    pub fn new(
        my_cards_string: &String,
        common_cards_string: &String,
        other_player_count: usize,
        n_rounds: i32,
    ) -> MonteCarloSimulation {
        let my_cards = Self::parse_cards(&my_cards_string);
        let common_cards = Self::parse_cards(&common_cards_string);
        let seen_cards = my_cards.combined(&common_cards);
        let unseen_cards = seen_cards.inverted();
        MonteCarloSimulation {
            my_cards,
            common_cards,
            unseen_cards,
            other_player_count,
            n_rounds,
        }
    }

    fn parse_cards(cards: &String) -> Cards {
        let cards_iter = cards
            .split_whitespace()
            .map(|x| Self::convert_str_card_to_usize(x).expect("Error"));
        return Cards::from_cards_iter(cards_iter);
    }

    fn convert_str_card_to_usize(str_card: &str) -> Result<usize, String> {
        let rank_char = str_card.chars().nth(0).expect("Error");
        let suit_char = str_card.chars().nth(1).expect("Error");

        let rank_id = match rank_char.to_ascii_uppercase() {
            '2' => Ok(0),
            '3' => Ok(1),
            '4' => Ok(2),
            '5' => Ok(3),
            '6' => Ok(4),
            '7' => Ok(5),
            '8' => Ok(6),
            '9' => Ok(7),
            'T' => Ok(8),
            'J' => Ok(9),
            'Q' => Ok(10),
            'K' => Ok(11),
            'A' => Ok(12),
            ch => Err(format!(
                "parse failed: expected rank character, but got '{}'",
                ch
            )),
        }?;
        let suit_id = match suit_char.to_ascii_lowercase() {
            'c' => Ok(0),
            'd' => Ok(1),
            'h' => Ok(2),
            's' => Ok(3),
            ch => Err(format!(
                "parse failed: expected suit character, but got '{}'",
                ch
            )),
        }?;
        return Ok(rank_id * 4 + suit_id);
    }

    pub fn run_simulation(&self) -> f32 {
        let wins: f32 = (0..self.n_rounds)
            .collect::<Vec<i32>>()
            .par_iter()
            .map(|_| self.run_simulation_round())
            .sum();
        return wins / self.n_rounds as f32;
    }

    pub fn run_simulation_round(&self) -> f32 {
        let mut deck = self.unseen_cards.clone();

        // Deal common cards up to COMMON_CARDS cards
        let mut common_cards = self.common_cards.clone();
        common_cards.combine(&deck.take_n_cards(COMMON_CARDS - common_cards.len()));

        // Deal ourselves up to PLAYER_CARDS cards
        let mut my_cards = self.my_cards.clone();
        my_cards.combine(&deck.take_n_cards(PLAYER_CARDS - my_cards.len()));
        my_cards.combine(&common_cards);

        let mut other_players_cards: Vec<Cards> = vec![];
        for _ in 0..self.other_player_count {
            let mut player_cards = deck.take_n_cards(PLAYER_CARDS);
            player_cards.combine(&common_cards);
            other_players_cards.push(player_cards);
        }

        return Self::my_pot_share(my_cards, other_players_cards);
    }

    fn my_pot_share(mut my_cards: Cards, other_players_cards: Vec<Cards>) -> f32 {
        let my_rank = my_cards.convert_to_hand().evaluate();
        let mut drawn_hands = 0;
        for mut player_cards in other_players_cards {
            let player_rank = player_cards.convert_to_hand().evaluate();
            if player_rank > my_rank {
                return 0f32;
            } else if player_rank == my_rank {
                drawn_hands += 1;
            }
        }

        if drawn_hands != 0 {
            return 1f32 / drawn_hands as f32;
        }

        return 1f32;
    }
}
