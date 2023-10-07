use crate::constants::*;
use ::holdem_hand_evaluator_rs::Hand;
use fastrand::choose_multiple;

pub struct MonteCarloSimulation {
    my_cards: Vec<usize>,
    common_cards: Vec<usize>,
    unseen_cards: Vec<usize>,
    other_player_count: usize,
    n_rounds: i32,
    cards_to_deal: usize,
}

impl MonteCarloSimulation {
    pub fn new(
        my_cards: &String,
        common_cards: &String,
        other_player_count: usize,
        n_rounds: i32,
    ) -> MonteCarloSimulation {
        let my_cards_vector = Self::parse_cards(&my_cards);
        let common_cards_vector = Self::parse_cards(&common_cards);
        let mut unseen_cards: Vec<usize> = vec![];
        for card in 0..52 {
            let usize_card = card as usize;
            if my_cards_vector.contains(&usize_card) {
                continue;
            }
            if common_cards_vector.contains(&usize_card) {
                continue;
            }
            unseen_cards.push(usize_card);
        }

        let for_me: usize = PLAYER_CARDS - my_cards_vector.len();
        let for_players: usize = other_player_count * PLAYER_CARDS;
        let for_common: usize = COMMON_CARDS - common_cards_vector.len();
        let cards_to_deal: usize = for_me + for_players + for_common;

        MonteCarloSimulation {
            my_cards: my_cards_vector,
            common_cards: common_cards_vector,
            unseen_cards,
            other_player_count,
            n_rounds,
            cards_to_deal,
        }
    }

    fn parse_cards(cards: &String) -> Vec<usize> {
        return cards
            .split_whitespace()
            .map(|x| Self::convert_str_card_to_usize(x).expect("Error"))
            .collect();
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
        let mut wins: i32 = 0;
        for _ in 0..self.n_rounds {
            let result = self.run_simulation_round();
            wins += result as i32;
        }
        return wins as f32 / self.n_rounds as f32;
    }

    pub fn run_simulation_round(&self) -> i8 {
        // "Shuffle" the cards and take out just as many as we need
        let mut deck: Vec<usize> =
            choose_multiple(self.unseen_cards.iter().cloned(), self.cards_to_deal);

        // Deal common cards up to COMMON_CARDS cards
        let mut common_cards = self.common_cards.clone();
        for _ in 0..COMMON_CARDS - common_cards.len() {
            common_cards.push(deck.pop().expect("Didn't get enough cards in deck"));
        }

        // Deal ourselves up to PLAYER_CARDS cards
        let mut my_cards = self.my_cards.clone();
        for _ in 0..PLAYER_CARDS - my_cards.len() {
            my_cards.push(deck.pop().expect("Didn't get enough cards in deck"));
        }
        my_cards.extend_from_slice(common_cards.as_slice());

        let mut other_players_cards: Vec<Vec<usize>> = vec![];
        for _ in 0..self.other_player_count {
            let mut player_cards: Vec<usize> = vec![];
            for _ in 0..PLAYER_CARDS {
                player_cards.push(deck.pop().expect("Didn't get enough cards in deck"));
            }
            player_cards.extend_from_slice(common_cards.as_slice());
            other_players_cards.push(player_cards);
        }

        if Self::is_win_for_me(my_cards, other_players_cards) {
            return 1;
        }
        return 0;
    }

    fn is_win_for_me(my_cards: Vec<usize>, other_players_cards: Vec<Vec<usize>>) -> bool {
        let my_hand = Hand::from_slice(my_cards.as_slice());
        let my_rank = my_hand.evaluate();

        for player_cards in other_players_cards {
            let player_hand = Hand::from_slice(player_cards.as_slice());
            let player_rank = player_hand.evaluate();
            if player_rank > my_rank {
                return false;
            }
        }
        return true;
    }
}
