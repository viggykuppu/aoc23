use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    pub hand: Vec<char>,
    pub bid: u32,
    pub hand_kind: HandKind,
}

impl Hand {
    pub fn new(hand: Vec<char>, bid: u32) -> Hand {
        let hand_kind = Self::get_hand_kind(&hand);
        Hand {
            hand: hand,
            bid: bid,
            hand_kind: hand_kind,
        }
    }

    fn get_hand_kind(hand: &Vec<char>) -> HandKind {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut hand_kind = HandKind::High;
        hand.iter().for_each(|card| {
        if let Some(val) = char_map.get_mut(card) {
            *val += 1_usize;
        } else {
            char_map.insert(*card, 1);
        }
        });
        for key in char_map.keys() {
            let num_occurrences = *char_map.get(key).unwrap();
            if num_occurrences == 5_usize {
                hand_kind = HandKind::Five;
            }
            if num_occurrences == 4_usize {
                hand_kind = HandKind::Four;
            }
            if num_occurrences == 3_usize {
                if hand_kind == HandKind::OnePair {
                    hand_kind = HandKind::Full;
                } else {
                    hand_kind = HandKind::Three
                }
            }
            if num_occurrences == 2_usize {
                if hand_kind == HandKind::Three {
                    hand_kind = HandKind::Full;
                } else if hand_kind == HandKind::OnePair {
                    hand_kind = HandKind::TwoPair;
                } else {
                    hand_kind = HandKind::OnePair;
                }
            }
        }
        return hand_kind;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return if self.hand_kind == other.hand_kind {
            for (self_hand_card, other_hand_card) in self.hand.iter().zip(other.hand.iter()) {
                if self_hand_card != other_hand_card {
                    return self_hand_card.partial_cmp(other_hand_card);
                }
            }
            Some(std::cmp::Ordering::Equal)
        } else {
            self.hand_kind.partial_cmp(&other.hand_kind)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hand2 {
    pub hand: Vec<char>,
    pub bid: u32,
    pub hand_kind: HandKind
}

impl Hand2{
    pub fn new(hand: Vec<char>, bid: u32) -> Hand2{
        let hand_kind = Self::get_hand_kind(&hand);
        Hand2 {
            hand: hand,
            bid: bid,
            hand_kind: hand_kind
        }
    }

    fn get_hand_kind(hand: &Vec<char>) -> HandKind {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut hand_kind = HandKind::High;
        let mut num_jokers = 0_usize;
        hand.iter().for_each(|card| {
            if *card == '1' {
                num_jokers += 1_usize;
                return;
            }
            if let Some(val) = char_map.get_mut(card) {
                *val += 1_usize;
            } else {
                char_map.insert(*card, 1);
            }
        });
        for key in char_map.keys() {
            let num_occurrences = *char_map.get(key).unwrap();
            if num_occurrences == 5_usize {
                hand_kind = HandKind::Five;
            }
            if num_occurrences == 4_usize {
                hand_kind = HandKind::Four;
            }
            if num_occurrences == 3_usize {
                if hand_kind == HandKind::OnePair {
                    hand_kind = HandKind::Full;
                } else {
                    hand_kind = HandKind::Three
                }
            }
            if num_occurrences == 2_usize {
                if hand_kind == HandKind::Three {
                    hand_kind = HandKind::Full;
                } else if hand_kind == HandKind::OnePair {
                    hand_kind = HandKind::TwoPair;
                } else {
                    hand_kind = HandKind::OnePair;
                }
            }
        }
        if hand_kind == HandKind::Four && num_jokers > 0 {
            hand_kind = HandKind::Five;
        } else if hand_kind == HandKind::Three && num_jokers > 0 {
            if num_jokers == 2 {
                hand_kind = HandKind::Five;
            } else if num_jokers == 1 {
                hand_kind = HandKind::Four;
            }
        } else if hand_kind == HandKind::TwoPair && num_jokers > 0 {
            hand_kind = HandKind::Full;
        } else if hand_kind == HandKind::OnePair && num_jokers > 0 {
            if num_jokers == 3 {
                hand_kind = HandKind::Five;
            } else if num_jokers == 2 {
                hand_kind = HandKind::Four;
            } else if num_jokers == 1 {
                hand_kind = HandKind::Three;
            }
        } else if hand_kind == HandKind::High && num_jokers > 0 {
            if num_jokers == 5 || num_jokers == 4 {
                hand_kind = HandKind::Five;
            } else if num_jokers == 3 {
                hand_kind = HandKind::Four;
            } else if num_jokers == 2 {
                hand_kind = HandKind::Three ;
            } else if num_jokers == 1 {
                hand_kind = HandKind::OnePair;
            }
        }
        return hand_kind;
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return if self.hand_kind == other.hand_kind {
            for (self_hand_card, other_hand_card) in self.hand.iter().zip(other.hand.iter()) {
                if self_hand_card != other_hand_card {
                    return self_hand_card.partial_cmp(other_hand_card);
                }
            }
            Some(std::cmp::Ordering::Equal)
        } else {
            self.hand_kind.partial_cmp(&other.hand_kind)
        }
    }
}

impl Ord for Hand2{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
