use holdem_hand_evaluator_rs::Hand;

#[derive(Clone)]
pub struct Cards {
    mask: u64,
}

impl Cards {
    const TOTAL_CARDS: usize = 52;

    pub fn new() -> Self {
        Cards { mask: 0 }
    }

    fn check_bit(&self, i: usize) -> bool {
        let bit_value = self.mask >> i & 1;
        return bit_value != 0;
    }

    fn set_bit(&mut self, i: usize) {
        self.mask = self.mask | (1 << i);
    }

    fn clear_bit(&mut self, i: usize) {
        self.mask = self.mask & !(1 << i);
    }

    pub fn from_cards_iter(cards: impl Iterator<Item = usize>) -> Self {
        let mut new_cards = Self::new();
        for card in cards {
            new_cards.set_bit(card);
        }
        return new_cards;
    }

    // This uses up the mask in the process, but that's fine
    pub fn convert_to_hand(&mut self) -> Hand {
        let mut hand = Hand::new();
        while self.mask != 0 {
            let index = self.mask.trailing_zeros() as usize;
            self.clear_bit(index);
            hand = hand.add_card(index);
        }
        return hand;
    }

    pub fn take_n_cards(&mut self, n: usize) -> Self {
        let mut cards_left = n;
        let mut new_cards = Self::new();

        while cards_left > 0 {
            if self.mask == 0 {
                panic!("Cannot take anymore cards, 0 cards left");
            }

            let index = fastrand::usize(0..Self::TOTAL_CARDS);

            // Will take quite long if there are few cards left, but it's ok because
            // typically there are 32 (8 players, last card) to 50 (first card) cards left
            if !self.check_bit(index) {
                continue;
            }

            self.clear_bit(index);
            new_cards.set_bit(index);
            cards_left -= 1;
        }

        return new_cards;
    }

    pub fn inverted(&self) -> Self {
        let mut new_cards = Self::new();
        new_cards.mask = !self.mask;
        return new_cards;
    }
    pub fn combined(&self, other_cards: &Self) -> Self {
        let mut new_cards = Self::new();
        new_cards.mask = self.mask | other_cards.mask;
        return new_cards;
    }

    pub fn combine(&mut self, other_cards: &Self) {
        self.mask = self.mask | other_cards.mask;
    }
    pub fn len(&self) -> usize {
        return self.mask.count_ones() as usize;
    }
}
