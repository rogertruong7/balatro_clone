use ortalib::{ Card, Enhancement, Joker, PokerHand, Rank, Suit, SuitColor };
use std::collections::HashMap;
use crate::cardscorer::{ ScoringData, ScoringPlayedCard };
use crate::utils::rank_value;

/// Evaluates a given poker hand and returns the possible poker hands along with the
/// cards that contribute to the winning hand.
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `data` - A reference to `ScoringData` containing round information, such as jokers used.
///
/// # Returns
/// A tuple containing:
/// - A vector of `PokerHand` values representing the types of hands found in the played hand.
/// - A vector of `ScoringPlayedCard` that contains the cards contributing to the best poker hands.
pub fn evaluate_hand(
    played_hand: &Vec<Card>,
    data: &ScoringData
) -> (Vec<PokerHand>, Vec<ScoringPlayedCard>) {
    let mut rank_count: HashMap<&Rank, u8> = HashMap::new();
    let mut suit_count: HashMap<&Suit, i32> = HashMap::new();
    let mut ranks: Vec<u8> = Vec::new();
    let mut suits: Vec<Suit> = Vec::new();
    let mut pokerhands: Vec<PokerHand> = Vec::new();
    let mut winning_cards: Vec<ScoringPlayedCard> = Vec::new();
    let mut is_fourfingers: bool = false;
    let mut is_shortcut: bool = false;
    let mut is_smeared: bool = false;

    data.round.jokers.iter().for_each(|joker_card| {
        match joker_card.joker {
            Joker::FourFingers => {
                is_fourfingers = true;
            }
            Joker::Shortcut => {
                is_shortcut = true;
            }
            Joker::SmearedJoker => {
                is_smeared = true;
            }
            _ => {}
        }
    });

    // Count occurrences of each rank and suit
    for card in played_hand {
        if let Some(enhancement) = card.enhancement {
            if enhancement == Enhancement::Wild {
                *suit_count.entry(&Suit::Clubs).or_insert(0) += 1;
                *suit_count.entry(&Suit::Spades).or_insert(0) += 1;
                *suit_count.entry(&Suit::Diamonds).or_insert(0) += 1;
                *suit_count.entry(&Suit::Hearts).or_insert(0) += 1;
            } else {
                if is_smeared {
                    if card.suit.color() == SuitColor::Black {
                        *suit_count.entry(&Suit::Clubs).or_insert(0) += 1;
                        *suit_count.entry(&Suit::Spades).or_insert(0) += 1;
                    } else {
                        *suit_count.entry(&Suit::Diamonds).or_insert(0) += 1;
                        *suit_count.entry(&Suit::Hearts).or_insert(0) += 1;
                    }
                } else {
                    *suit_count.entry(&card.suit).or_insert(0) += 1;
                }
            }
        } else {
            if is_smeared {
                if card.suit.color() == SuitColor::Black {
                    *suit_count.entry(&Suit::Clubs).or_insert(0) += 1;
                    *suit_count.entry(&Suit::Spades).or_insert(0) += 1;
                } else {
                    *suit_count.entry(&Suit::Diamonds).or_insert(0) += 1;
                    *suit_count.entry(&Suit::Hearts).or_insert(0) += 1;
                }
            } else {
                *suit_count.entry(&card.suit).or_insert(0) += 1;
            }
        }
        *rank_count.entry(&card.rank).or_insert(0) += 1;

        ranks.push(card.rank.rank_value() as u8);
        suits.push(card.suit);
    }

    ranks.sort();
    ranks.reverse();

    let mut is_flush: bool = false;
    let mut is_four_fingers_flush: bool = false;
    if is_fourfingers {
        is_four_fingers_flush = suit_count.values().any(|&count| count >= 4);
    } else {
        is_flush = suit_count.values().any(|&count| count == 5);
    }

    let mut four_finger_straight_indices: Vec<usize> = Vec::new();
    let mut four_finger_shortcut_straight_indices: Vec<usize> = Vec::new();

    let mut is_four_fingers_straight: bool = false;
    let is_straight = is_straight(
        &played_hand,
        &is_fourfingers,
        &mut is_four_fingers_straight,
        &mut four_finger_straight_indices
    );

    let mut four_shortcut_straight: bool = false;
    let mut shortcut_straight: bool = false;

    if is_shortcut {
        shortcut_straight = is_shortcut_straight(
            &played_hand,
            &is_shortcut,
            &mut four_shortcut_straight,
            &mut four_finger_shortcut_straight_indices
        );
    }

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
    // 5 of same suit, 5 straight but shortcutted
    if is_flush && shortcut_straight {
        pokerhands.push(PokerHand::StraightFlush);
    }
    if is_four_fingers_flush && is_straight {
        pokerhands.push(PokerHand::StraightFlush);
    }
    // 4 of same suit, straight but 4
    if is_four_fingers_flush && is_four_fingers_straight {
        pokerhands.push(PokerHand::StraightFlush);
    }
    // 4 of same suit, straight but 4 and shortcutted
    if is_four_fingers_flush && four_shortcut_straight {
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
    // 4 flush
    if is_four_fingers_flush {
        pokerhands.push(PokerHand::Flush);
    }
    if is_straight {
        pokerhands.push(PokerHand::Straight);
    }
    // 5 Shortcut straight
    if shortcut_straight {
        pokerhands.push(PokerHand::Straight);
    }
    // 4 straight
    if is_four_fingers_straight {
        pokerhands.push(PokerHand::Straight);
    }
    // 4 straight with shortcut
    if four_shortcut_straight {
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
    } else if is_flush && shortcut_straight {
        // 5 of same suit, 5 straight but shortcutted
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_four_fingers_flush && is_straight {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_four_fingers_flush && is_four_fingers_straight {
        // 4 of same suit, straight but 4 and shortcutted
        collect_four_flush(&played_hand, &mut winning_cards, &is_smeared);
    } else if is_four_of_a_kind {
        collect_four_of_a_kind(&played_hand, &rank_count, &mut winning_cards);
    } else if is_full_house {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_flush {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_four_fingers_flush {
        // Four fingers flush
        collect_four_flush(&played_hand, &mut winning_cards, &is_smeared);
    } else if is_straight {
        collect_all_five(&played_hand, &mut winning_cards);
    } else if shortcut_straight {
        // 5 Shortcut straight
        collect_all_five(&played_hand, &mut winning_cards);
    } else if is_four_fingers_straight {
        // 4 straight
        collect_indices(&played_hand, &mut winning_cards, &four_finger_straight_indices);
    } else if four_shortcut_straight {
        // 4 straight with shortcut
        collect_indices(&played_hand, &mut winning_cards, &four_finger_shortcut_straight_indices);
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

/// Checks whether a hand contains a valid straight (5 consecutive ranks).
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `is_four_fingers` - A boolean indicating whether "Four Fingers" is active.
/// * `is_four_fingers_straight` - A mutable reference to a boolean indicating if it's a four-card straight.
/// * `indexes` - A mutable reference to a vector of indices that tracks the winning cards.
///
/// # Returns
/// `true` if the hand contains a valid straight, `false` otherwise.
fn is_straight(
    played_hand: &Vec<Card>,
    is_four_fingers: &bool,
    is_four_fingers_straight: &mut bool,
    indexes: &mut Vec<usize>
) -> bool {
    // We don't store any indexes unless a 4-card straight is found.
    indexes.clear();

    // For non-four-fingers, require exactly 5 cards.
    // For four-fingers, at least 4 cards are required.
    if (!*is_four_fingers && played_hand.len() != 5) || (*is_four_fingers && played_hand.len() < 4) {
        return false;
    }

    // Create a vector of (rank_value, original_index) pairs and sort by rank.
    let mut card_indices: Vec<(u8, usize)> = played_hand
        .iter()
        .enumerate()
        .map(|(i, card)| (rank_value(card.rank), i))
        .collect();
    card_indices.sort_by_key(|&(rank, _)| rank);

    // Check for special Ace–low 5‑card straight.
    if played_hand.len() == 5 {
        let sorted_ranks: Vec<u8> = card_indices
            .iter()
            .map(|&(r, _)| r)
            .collect();
        if sorted_ranks == vec![2, 3, 4, 5, 14] {
            // Found an Ace–low straight, but do not record indexes.
            return true;
        }
    }

    // Standard 5‑card straight check.
    if played_hand.len() == 5 {
        let mut rank_values: Vec<u8> = played_hand
            .into_iter()
            .map(|&card| rank_value(card.rank))
            .collect();
        rank_values.sort();
        let mut is_five_straight = true;
        for i in 0..rank_values.len() - 1 {
            if rank_values[i] != rank_values[i + 1] - 1 {
                is_five_straight = false;
                break;
            }
        }
        if is_five_straight {
            // Do not record indexes when a full 5-card straight is found.
            return true;
        }
    }

    // If four-fingers is allowed, check for any contiguous 4-card straight.
    if *is_four_fingers {
        let len = card_indices.len();
        for start in 0..=len - 4 {
            let group = &card_indices[start..start + 4];
            let group_ranks: Vec<u8> = group
                .iter()
                .map(|&(r, _)| r)
                .collect();
            let mut is_four_straight = true;
            for i in 0..group_ranks.len() - 1 {
                if group_ranks[i] != group_ranks[i + 1] - 1 {
                    is_four_straight = false;
                    break;
                }
            }
            if is_four_straight {
                // Only record indexes for the winning 4-card straight.
                indexes.extend(group.iter().map(|&(_, i)| i));
                *is_four_fingers_straight = true;
                return false;
            }
        }
        // If no valid 4-card straight is found, ensure indexes remain empty.
        indexes.clear();
    }

    false
}

/// Checks whether a hand contains a valid shortcut straight (cards with ranks that differ by 1 or 2).
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `is_four_fingers` - A boolean indicating whether "Four Fingers" is active.
/// * `four_shortcut_straight` - A mutable reference to a boolean indicating if it's a four-card shortcut straight.
/// * `indexes` - A mutable reference to a vector of indices that tracks the winning cards.
///
/// # Returns
/// `true` if the hand contains a valid shortcut straight, `false` otherwise.
fn is_shortcut_straight(
    played_hand: &Vec<Card>,
    is_four_fingers: &bool,
    four_shortcut_straight: &mut bool,
    indexes: &mut Vec<usize>
) -> bool {
    // We don't store any indexes unless a 4-card shortcut straight is found.
    indexes.clear();

    // For non-four-fingers, require exactly 5 cards.
    // For four-fingers, at least 4 cards are required.
    if (!*is_four_fingers && played_hand.len() != 5) || (*is_four_fingers && played_hand.len() < 4) {
        return false;
    }

    // Create a vector of (rank_value, original_index) pairs and sort by rank.
    let mut card_indices: Vec<(u8, usize)> = played_hand
        .iter()
        .enumerate()
        .map(|(i, card)| (rank_value(card.rank), i))
        .collect();
    card_indices.sort_by_key(|&(rank, _)| rank);

    // Check for a full 5‑card shortcut straight.
    if played_hand.len() == 5 {
        let sorted_ranks: Vec<u8> = card_indices
            .iter()
            .map(|&(r, _)| r)
            .collect();
        let mut is_full_shortcut = true;
        for i in 0..sorted_ranks.len() - 1 {
            let diff = sorted_ranks[i + 1] - sorted_ranks[i];
            // Allowed differences: consecutive (1) or a gap of one rank (2).
            if diff != 1 && diff != 2 {
                is_full_shortcut = false;
                break;
            }
        }
        if is_full_shortcut {
            // Found a full shortcut straight; do not record indexes.
            return true;
        }
    }

    // If four-fingers is allowed, check for any contiguous 4-card shortcut straight.
    if *is_four_fingers {
        let len = card_indices.len();
        for start in 0..=len - 4 {
            let group = &card_indices[start..start + 4];
            let group_ranks: Vec<u8> = group
                .iter()
                .map(|&(r, _)| r)
                .collect();
            let mut is_four_seq = true;
            for i in 0..group_ranks.len() - 1 {
                let diff = group_ranks[i + 1] - group_ranks[i];
                if diff != 1 && diff != 2 {
                    is_four_seq = false;
                    break;
                }
            }
            if is_four_seq {
                // Record indexes only for the winning 4-card shortcut straight.
                indexes.extend(group.iter().map(|&(_, i)| i));
                *four_shortcut_straight = true;
                return false;
            }
        }
        // If no valid 4-card shortcut straight is found, clear indexes.
        indexes.clear();
    }

    false
}

/// Collects all five cards from the hand and adds them to the winning cards vector.
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `winning_cards` - A mutable reference to a vector of `ScoringPlayedCard` where the winning cards are stored.
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

/// Collects the four of a kind cards from the hand and adds them to the winning cards vector.
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `rank_count` - A reference to a `HashMap` of `Rank` and their occurrences in the played hand.
/// * `winning_cards` - A mutable reference to a vector of `ScoringPlayedCard` where the winning cards are stored.
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

/// Collects the three of a kind cards from the hand and adds them to the winning cards vector.
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `rank_count` - A reference to a `HashMap` of `Rank` and their occurrences in the played hand.
/// * `winning_cards` - A mutable reference to a vector of `ScoringPlayedCard` where the winning cards are stored.
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

/// Collects pairs from the hand and adds them to the winning cards vector.
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `rank_count` - A reference to a `HashMap` of `Rank` and their occurrences in the played hand.
/// * `winning_cards` - A mutable reference to a vector of `ScoringPlayedCard` where the winning cards are stored.
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

/// Collects the highest card from the hand and adds it to the winning cards vector.
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `winning_cards` - A mutable reference to a vector of `ScoringPlayedCard` where the highest card is stored.
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

/// Collects cards from the hand based on specific indices for Four Fingers + Shortcut
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `winning_cards` - A mutable reference to a vector of `ScoringPlayedCard` where the selected cards are stored.
/// * `indices` - A vector of indices indicating the cards that should be added to the winning hand.
fn collect_indices(
    played_hand: &Vec<Card>,
    winning_cards: &mut Vec<ScoringPlayedCard>,
    indices: &Vec<usize>
) {
    played_hand
        .into_iter()
        .enumerate()
        .for_each(|(index, card)| {
            indices.into_iter().for_each(|value| {
                if index == *value {
                    winning_cards.push(ScoringPlayedCard {
                        rank: card.rank,
                        suit: card.suit,
                        enhancement: card.enhancement,
                        edition: card.edition,
                        scored_card_index: winning_cards.len(),
                        is_face: card.rank.is_face(),
                    });
                }
            });
        });
}

/// Collects cards from a four figners flush hand and adds them to the winning cards vector.
///
/// # Arguments
/// * `played_hand` - A vector of `Card` objects representing the hand played.
/// * `winning_cards` - A mutable reference to a vector of `ScoringPlayedCard` where the selected flush cards are stored.
/// * `is_smeared` - A boolean indicating whether the cards have a smeared effect, affecting which suits to count.
fn collect_four_flush(
    played_hand: &Vec<Card>,
    winning_cards: &mut Vec<ScoringPlayedCard>,
    is_smeared: &bool
) {
    // Count the number of cards for each suit.
    let mut suit_counts: HashMap<Suit, usize> = HashMap::new();
    for card in played_hand.iter() {
        if *is_smeared {
            if card.suit.color() == SuitColor::Black {
                *suit_counts.entry(Suit::Clubs).or_insert(0) += 1;
                *suit_counts.entry(Suit::Spades).or_insert(0) += 1;
            } else {
                *suit_counts.entry(Suit::Diamonds).or_insert(0) += 1;
                *suit_counts.entry(Suit::Hearts).or_insert(0) += 1;
            }
        } else {
            *suit_counts.entry(card.suit).or_insert(0) += 1;
        }
    }

    // Determine if there is a suit with at least 4 cards.
    let flush_suit = suit_counts.into_iter().find_map(|(suit, count)| {
        if count >= 4 { Some(suit) } else { None }
    });

    // If such a suit exists, iterate through played_hand in order
    // and collect the first 4 cards of that suit.
    if let Some(target_suit) = flush_suit {
        let mut collected = 0;
        for card in played_hand.iter() {
            if *is_smeared {
                if card.suit.color() == target_suit.color() {
                    winning_cards.push(ScoringPlayedCard {
                        rank: card.rank,
                        suit: card.suit,
                        enhancement: card.enhancement,
                        edition: card.edition,
                        scored_card_index: winning_cards.len(),
                        is_face: card.rank.is_face(),
                    });
                    collected += 1;
                    if collected == 4 {
                        break;
                    }
                }
            } else {
                if card.suit == target_suit {
                    winning_cards.push(ScoringPlayedCard {
                        rank: card.rank,
                        suit: card.suit,
                        enhancement: card.enhancement,
                        edition: card.edition,
                        scored_card_index: winning_cards.len(),
                        is_face: card.rank.is_face(),
                    });
                    collected += 1;
                    if collected == 4 {
                        break;
                    }
                }
            }
        }
    }
}
