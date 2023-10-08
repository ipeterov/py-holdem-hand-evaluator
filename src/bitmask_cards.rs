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

    pub fn as_cards_iter(&self) -> impl Iterator<Item = usize> + '_ {
        // All indexes of bits where value is 1
        return (0..Self::TOTAL_CARDS).filter(|i| self.check_bit(*i));
    }

    pub fn as_hand(&self) -> Hand {
        let mut hand = Hand::new();
        for card in self.as_cards_iter() {
            hand = hand.add_card(card);
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

            // Will take quite long if there are few cards left
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
