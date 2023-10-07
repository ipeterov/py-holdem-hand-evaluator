use crate::constants::*;
use ::holdem_hand_evaluator::Hand;
use rand::seq::IteratorRandom;

pub struct MonteCarloSimulation {
    my_cards: Vec<String>,
    common_cards: Vec<String>,
    unseen_cards: Vec<String>,
    other_player_count: i8,
    n_rounds: i32,
    cards_to_deal: i8,
}

impl MonteCarloSimulation {
    fn parse_cards(cards: &String) -> Vec<String> {
        return cards.split_whitespace().map(|x| String::from(x)).collect();
    }

    pub fn new(
        my_cards: &String,
        common_cards: &String,
        other_player_count: i8,
        n_rounds: i32,
    ) -> MonteCarloSimulation {
        let my_cards_vector = Self::parse_cards(&my_cards);
        let common_cards_vector = Self::parse_cards(&common_cards);
        let seen_cards: Vec<String> =
            [my_cards_vector.as_slice(), common_cards_vector.as_slice()].concat();
        let mut unseen_cards: Vec<String> = vec![];
        for card in ALL_CARDS {
            let card_string = String::from(card);
            if !seen_cards.contains(&card_string) {
                unseen_cards.push(card_string);
            }
        }

        let for_me = PLAYER_CARDS - my_cards_vector.len() as i8;
        let for_players = other_player_count * PLAYER_CARDS;
        let for_common = COMMON_CARDS - common_cards_vector.len() as i8;
        let cards_to_deal = for_me + for_players + for_common;

        MonteCarloSimulation {
            my_cards: my_cards_vector,
            common_cards: common_cards_vector,
            unseen_cards,
            other_player_count,
            n_rounds,
            cards_to_deal,
        }
    }

    pub fn run_simulation(&self) -> f32 {
        let mut wins: i64 = 0;
        for _ in 0..self.n_rounds {
            let result = self.run_simulation_round();
            wins += result as i64;
        }
        return wins as f32 / self.n_rounds as f32;
    }

    pub fn run_simulation_round(&self) -> i8 {
        let mut rng = rand::thread_rng();
        // "Shuffle" the cards and take out just as many as we need
        let mut deck: Vec<String> = self
            .unseen_cards
            .iter()
            .cloned()
            .choose_multiple(&mut rng, self.cards_to_deal as usize);

        // Deal ourselves up to PLAYER_CARDS cards
        let mut my_cards = self.my_cards.clone();
        for _ in 0..PLAYER_CARDS - my_cards.len() as i8 {
            my_cards.push(deck.pop().expect("Didn't get enough cards in deck"));
        }

        // Deal common cards up to COMMON_CARDS cards
        let mut common_cards = self.common_cards.clone();
        for _ in 0..COMMON_CARDS - common_cards.len() as i8 {
            common_cards.push(deck.pop().expect("Didn't get enough cards in deck"));
        }

        let mut other_player_cards: Vec<Vec<String>> = vec![];
        for _ in 0..self.other_player_count {
            let mut player_cards: Vec<String> = vec![];
            for _ in 0..PLAYER_CARDS {
                player_cards.push(deck.pop().expect("Didn't get enough cards in deck"));
            }
            other_player_cards.push(player_cards);
        }

        if Self::is_win_for_me(my_cards, common_cards, other_player_cards) {
            return 1;
        } else {
            return 0;
        };
    }

    fn is_win_for_me(
        my_cards: Vec<String>,
        common_cards: Vec<String>,
        other_player_cards: Vec<Vec<String>>,
    ) -> bool {
        let my_hand_str = [my_cards.as_slice(), common_cards.as_slice()]
            .concat()
            .join("");
        let my_hand = my_hand_str.parse::<Hand>().unwrap();
        let my_rank = my_hand.evaluate();

        for player_cards in other_player_cards {
            let player_hand_str = [player_cards.as_slice(), common_cards.as_slice()]
                .concat()
                .join("");
            let player_hand = player_hand_str.parse::<Hand>().unwrap();
            let player_rank = player_hand.evaluate();
            if player_rank > my_rank {
                return false;
            }
        }
        return true;
    }
}
