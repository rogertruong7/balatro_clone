use ortalib::{ Card, Enhancement, PokerHand, Rank, Suit };
use std::collections::HashMap;
use crate::cardscorer::ScoringPlayedCard;
use crate::utils::rank_value;

pub fn evaluate_hand(played_hand: &Vec<Card>) -> (Vec<PokerHand>, Vec<ScoringPlayedCard>) {
    let mut rank_count: HashMap<&Rank, u8> = HashMap::new();
    let mut suit_count: HashMap<&Suit, i32> = HashMap::new();
    let mut ranks: Vec<u8> = Vec::new();
    let mut suits: Vec<Suit> = Vec::new();
    let mut pokerhands: Vec<PokerHand> = Vec::new();
    let mut winning_cards: Vec<ScoringPlayedCard> = Vec::new();
    // Count occurrences of each rank and suit
    for card in played_hand {
        if let Some(enhancement) = card.enhancement {
            if enhancement == Enhancement::Wild {
                *suit_count.entry(&Suit::Clubs).or_insert(0) += 1;
                *suit_count.entry(&Suit::Diamonds).or_insert(0) += 1;
                *suit_count.entry(&Suit::Spades).or_insert(0) += 1;
                *suit_count.entry(&Suit::Hearts).or_insert(0) += 1;
            } else {
                *suit_count.entry(&card.suit).or_insert(0) += 1;
            }
        } else {
            *suit_count.entry(&card.suit).or_insert(0) += 1;
        }
        *rank_count.entry(&card.rank).or_insert(0) += 1;

        ranks.push(card.rank.rank_value() as u8);
        suits.push(card.suit);
    }

    ranks.sort();
    ranks.reverse();

    let is_flush = suit_count.values().any(|&count| count == 5);
    let is_straight = is_straight(&played_hand);
    let is_five_of_a_kind = rank_count.values().any(|&count| count == 5);
    let is_four_of_a_kind = rank_count.values().any(|&count| count >= 4);
    let is_full_house =
        rank_count.values().any(|&count| count == 3) &&
        rank_count.values().any(|&count| count == 2);
    let is_three_of_a_kind = rank_count.values().any(|&count| count >= 3);
    let is_two_pair =
        rank_count
            .values()
            .filter(|&&count| count >= 2)
            .count() == 2;
    let is_pair = rank_count.values().any(|&count| count >= 2);

    if is_flush && is_five_of_a_kind {
        pokerhands.push(PokerHand::FlushFive);
    }
    if is_flush && is_full_house {
        pokerhands.push(PokerHand::FlushHouse);
    }
    if is_five_of_a_kind {
        pokerhands.push(PokerHand::FiveOfAKind);
    }
    if is_flush && is_straight {
        pokerhands.push(PokerHand::StraightFlush);
    }
    if is_four_of_a_kind {
        pokerhands.push(PokerHand::FourOfAKind);
    }
    if is_full_house {
        pokerhands.push(PokerHand::FullHouse);
    }
    if is_flush {
        pokerhands.push(PokerHand::Flush);
    }
    if is_straight {
        pokerhands.push(PokerHand::Straight);
    }
    if is_three_of_a_kind {
        pokerhands.push(PokerHand::ThreeOfAKind);
    }
    if is_two_pair {
        pokerhands.push(PokerHand::TwoPair);
    }
    if is_pair {
        pokerhands.push(PokerHand::Pair);
    }
    pokerhands.push(PokerHand::HighCard);

    if is_flush && is_five_of_a_kind {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_flush && is_full_house {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_five_of_a_kind {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_flush && is_straight {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_four_of_a_kind {
        collect_four_of_a_kind(&played_hand, &rank_count, &mut winning_cards);
    } else if is_full_house {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_flush {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_straight {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_three_of_a_kind {
        collect_three_of_a_kind(&played_hand, &rank_count, &mut winning_cards);
    } else if is_two_pair {
        collect_pairs(&played_hand, &rank_count, &mut winning_cards);
    } else if is_pair {
        collect_pairs(&played_hand, &rank_count, &mut winning_cards);
    } else {
        collect_high_card(&played_hand, &mut winning_cards);
    }

    pokerhands.sort();
    pokerhands.reverse();

    return (pokerhands, winning_cards);
}

fn is_straight(played_hand: &Vec<Card>) -> bool {
    if played_hand.len() != 5 {
        return false;
    }
    let mut rank_values: Vec<u8> = played_hand
        .into_iter()
        .map(|&card| rank_value(card.rank))
        .collect();
    rank_values.sort_unstable(); // Sort in ascending order
    if rank_values == vec![2, 3, 4, 5, 14] {
        return true;
    }
    // Standard straight check
    for i in 0..rank_values.len() - 1 {
        if rank_values[i] != rank_values[i + 1] - 1 {
            return false;
        }
    }
    true
}

// Collecting functions for various poker hands
fn collect_all_five(played_hand: &Vec<Card>, winning_cards: &mut Vec<ScoringPlayedCard>) {
    played_hand.into_iter().for_each(|card| {
        winning_cards.push(ScoringPlayedCard {
            rank: card.rank,
            suit: card.suit,
            enhancement: card.enhancement,
            edition: card.edition,
            scored_card_index: winning_cards.len(),
            is_face: card.rank.is_face(),
        });
    });
}

// Have to make sure they all collect in order top to down
fn collect_four_of_a_kind(
    played_hand: &Vec<Card>,
    rank_count: &HashMap<&Rank, u8>,
    winning_cards: &mut Vec<ScoringPlayedCard>
) {
    played_hand.into_iter().for_each(|card| {
        if let Some(&count) = rank_count.get(&card.rank) {
            if count == 4 {
                winning_cards.push(ScoringPlayedCard {
                    rank: card.rank,
                    suit: card.suit,
                    enhancement: card.enhancement,
                    edition: card.edition,
                    scored_card_index: winning_cards.len(),
                    is_face: card.rank.is_face(),
                });
            }
        }
    });
}

fn collect_three_of_a_kind(
    played_hand: &Vec<Card>,
    rank_count: &HashMap<&Rank, u8>,
    winning_cards: &mut Vec<ScoringPlayedCard>
) {
    played_hand.into_iter().for_each(|card| {
        if let Some(&count) = rank_count.get(&card.rank) {
            if count == 3 {
                winning_cards.push(ScoringPlayedCard {
                    rank: card.rank,
                    suit: card.suit,
                    enhancement: card.enhancement,
                    edition: card.edition,
                    scored_card_index: winning_cards.len(),
                    is_face: card.rank.is_face(),
                });
            }
        }
    });
}

fn collect_pairs(
    played_hand: &Vec<Card>,
    rank_count: &HashMap<&Rank, u8>,
    winning_cards: &mut Vec<ScoringPlayedCard>
) {
    played_hand.into_iter().for_each(|card| {
        if let Some(&count) = rank_count.get(&card.rank) {
            if count == 2 {
                winning_cards.push(ScoringPlayedCard {
                    rank: card.rank,
                    suit: card.suit,
                    enhancement: card.enhancement,
                    edition: card.edition,
                    scored_card_index: winning_cards.len(),
                    is_face: card.rank.is_face(),
                });
            }
        }
    });
}

fn collect_high_card(played_hand: &Vec<Card>, winning_cards: &mut Vec<ScoringPlayedCard>) {
    let card = played_hand
        .iter()
        .max_by_key(|card| rank_value(card.rank) as u8)
        .unwrap();
    winning_cards.push(ScoringPlayedCard {
        rank: card.rank,
        suit: card.suit,
        enhancement: card.enhancement,
        edition: card.edition,
        scored_card_index: winning_cards.len(),
        is_face: card.rank.is_face(),
    });
}
