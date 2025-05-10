use ortalib::{
    Chips,
    Edition,
    Enhancement,
    Joker,
    JokerCard,
    Mult,
    PokerHand,
    Rank,
    Suit,
    SuitColor,
};
use crate::cardscorer::{ HandCard, ScoringData, ScoringPlayedCard };
use crate::modifiers::{ get_foil_holo, get_polychrome };
use std::collections::HashMap;

pub trait JokerCardCalculator {
    fn calculate_independent_cards(
        &self,
        explain: &bool,
        curr_chips: &Chips,
        curr_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult);

    fn calculate_on_played_cards(
        &self,
        explain: &bool,
        current_chips: &Chips,
        current_mult: &Mult,
        played_card: &ScoringPlayedCard,
        scored_card_index: usize,
        data: &ScoringData
    ) -> (Chips, Mult);

    fn calculate_on_held_cards(
        &self,
        explain: &bool,
        current_chips: &Chips,
        current_mult: &Mult,
        held_card: &HandCard,
        data: &ScoringData
    ) -> (Chips, Mult);
    fn set_splash(&self, data: &mut ScoringData);
    fn set_pareidolia(&self, data: &mut ScoringData);
}

impl JokerCardCalculator for JokerCard {
    fn set_splash(&self, data: &mut ScoringData) {
        match self.joker {
            Joker::Splash => {
                let new_scored_cards: Vec<ScoringPlayedCard> = data.played_cards
                    .iter()
                    .enumerate()
                    .map(|(index, card)| ScoringPlayedCard {
                        rank: card.rank,
                        suit: card.suit,
                        enhancement: card.enhancement,
                        edition: card.edition,
                        scored_card_index: index,
                        is_face: card.rank.is_face(),
                    })
                    .collect();

                data.scored_cards = new_scored_cards;
            }
            _ => {}
        }
    }

    fn set_pareidolia(&self, data: &mut ScoringData) {
        match self.joker {
            Joker::Pareidolia => {
                data.scored_cards.iter_mut().for_each(|card| {
                    card.is_face = true;
                });
            }
            _ => {}
        }
    }

    fn calculate_independent_cards(
        &self,
        explain: &bool,
        current_chips: &Chips,
        current_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = *current_chips;
        let mut curr_mult: Mult = *current_mult;

        let pokerhands = &data.pokerhands;
        let cards_in_hand = &data.hand_cards;
        let scored_cards = &data.scored_cards;
        let joker_cards = &data.joker_cards;

        let card_title = format!("{:?}", self.joker);
        let (edition_chips, edition_mult) = get_foil_holo(
            self.edition,
            &card_title,
            explain,
            &curr_chips,
            &curr_mult
        );

        let mut is_smeared: bool = false;

        data.joker_cards.iter().for_each(|joker_card| {
            match joker_card.joker {
                Joker::SmearedJoker => {
                    is_smeared = true;
                }
                _ => {}
            }
        });

        curr_chips = edition_chips;
        curr_mult = edition_mult;

        match self.joker {
            Joker::Joker => {
                curr_mult += 4.0;
                if *explain {
                    println!("Joker +4 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                }
            }
            Joker::JollyJoker => {
                if pokerhands.contains(&PokerHand::Pair) {
                    curr_mult += 8.0;
                    if *explain {
                        println!("Jolly Joker +8 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::ZanyJoker => {
                if pokerhands.contains(&PokerHand::ThreeOfAKind) {
                    curr_mult += 12.0;
                    if *explain {
                        println!("Zany Joker +12 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::MadJoker => {
                if pokerhands.contains(&PokerHand::TwoPair) {
                    curr_mult += 10.0;
                    if *explain {
                        println!("Mad Joker +10 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::CrazyJoker => {
                if pokerhands.contains(&PokerHand::Straight) {
                    curr_mult += 12.0;
                    if *explain {
                        println!("Crazy Joker +12 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::DrollJoker => {
                if pokerhands.contains(&PokerHand::Flush) {
                    curr_mult += 10.0;
                    if *explain {
                        println!("Droll Joker +10 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::SlyJoker => {
                if pokerhands.contains(&PokerHand::Pair) {
                    curr_chips += 50.0;
                    if *explain {
                        println!("Sly Joker +50 Chips ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::WilyJoker => {
                if pokerhands.contains(&PokerHand::ThreeOfAKind) {
                    curr_chips += 100.0;
                    if *explain {
                        println!("Wily Joker +100 Chips ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::CleverJoker => {
                if pokerhands.contains(&PokerHand::TwoPair) {
                    curr_chips += 80.0;
                    if *explain {
                        println!("Clever Joker +80 Chips ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::DeviousJoker => {
                if pokerhands.contains(&PokerHand::Straight) {
                    curr_chips += 100.0;
                    if *explain {
                        println!("Devious Joker +100 Chips ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::CraftyJoker => {
                if pokerhands.contains(&PokerHand::Flush) {
                    curr_chips += 80.0;
                    if *explain {
                        println!("Crafty Joker +80 Chips ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::AbstractJoker => {
                curr_mult += 3.0 * (joker_cards.len() as f64);
                if *explain {
                    println!("Abstract Joker +3 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                }
            }
            Joker::Blackboard => {
                let contains_red_suit = cards_in_hand
                    .iter()
                    .any(|card| {
                        (card.suit == Suit::Diamonds || card.suit == Suit::Hearts) &&
                            card.enhancement != Some(Enhancement::Wild)
                    });

                if !contains_red_suit {
                    curr_mult *= 3.0;
                    if *explain {
                        println!("Blackboard Joker x3 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                }
            }
            Joker::FlowerPot => {
                let mut num_wild: u8 = 0;
                let mut suit_count: HashMap<Suit, u8> = HashMap::new();
                let mut colour_count: HashMap<SuitColor, u8> = HashMap::new();
                scored_cards.iter().for_each(|card| {
                    if let Some(enhancement) = card.enhancement {
                        if enhancement == Enhancement::Wild {
                            num_wild += 1;
                            *suit_count.entry(Suit::Clubs).or_insert(0) += 1;
                            *suit_count.entry(Suit::Diamonds).or_insert(0) += 1;
                            *suit_count.entry(Suit::Spades).or_insert(0) += 1;
                            *suit_count.entry(Suit::Hearts).or_insert(0) += 1;
                            *colour_count.entry(SuitColor::Black).or_insert(0) += 1;
                            *colour_count.entry(SuitColor::Red).or_insert(0) += 1;
                        } else {
                            *suit_count.entry(card.suit).or_insert(0) += 1;
                            *colour_count.entry(card.suit.color()).or_insert(0) += 1;
                        }
                    } else {
                        *suit_count.entry(card.suit).or_insert(0) += 1;
                        *colour_count.entry(card.suit.color()).or_insert(0) += 1;
                    }
                });
                if is_smeared && num_wild == 0 {
                    // Check for atleast 2 Reds and 2 Blacks
                    if
                        colour_count
                            .iter()
                            .filter(|&(_, &count)| count >= 2)
                            .count() == 2
                    {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    }
                } else if is_smeared && num_wild != 0 {
                    if num_wild >= 4 {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    } else if
                        num_wild == 3 &&
                        colour_count
                            .iter()
                            .filter(|&(_, &value)| value > 4)
                            .count() >= 1
                    {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    } else if
                        num_wild == 2 &&
                        (colour_count
                            .iter()
                            .filter(|&(_, &value)| value >= 3)
                            .count() == 2 ||
                            colour_count
                                .iter()
                                .filter(|&(_, &value)| value >= 4)
                                .count() >= 1)
                    {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    } else if
                        num_wild == 1 &&
                        colour_count
                            .iter()
                            .filter(|&(_, &value)| value >= 3)
                            .count() >= 1 &&
                        colour_count
                            .iter()
                            .filter(|&(_, &value)| value >= 2)
                            .count() == 2
                    {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    }
                } else if suit_count.len() == 4 && num_wild == 0 {
                    curr_mult *= 3.0;
                    if *explain {
                        println!("Flower Pot Joker x3 Mult ( {:?} x {:?} )", curr_chips, curr_mult);
                    }
                } else if num_wild != 0 {
                    if num_wild >= 4 {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    } else if
                        num_wild == 3 &&
                        suit_count
                            .iter()
                            .filter(|&(_, &value)| value > 3)
                            .count() >= 1
                    {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    } else if
                        num_wild == 2 &&
                        suit_count
                            .iter()
                            .filter(|&(_, &value)| value > 2)
                            .count() >= 2
                    {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    } else if
                        num_wild == 1 &&
                        suit_count
                            .iter()
                            .filter(|&(_, &value)| value > 1)
                            .count() >= 3
                    {
                        curr_mult *= 3.0;
                        if *explain {
                            println!(
                                "Flower Pot Joker x3 Mult ( {:?} x {:?} )",
                                curr_chips,
                                curr_mult
                            );
                        }
                    }
                }
            }
            Joker::Blueprint => {
                if pokerhands.contains(&PokerHand::Flush) {
                    curr_chips += 80.0;
                    if *explain {
                        println!(
                            "Blueprint Joker +80 Chips ( {:?} x {:?} )",
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            _ => {}
        }
        let (final_chips, final_mult) = match self.edition {
            Some(Edition::Polychrome) =>
                get_polychrome(&card_title, explain, &curr_chips, &curr_mult),
            _ => (curr_chips, curr_mult),
        };

        return (final_chips, final_mult);
    }

    fn calculate_on_played_cards(
        &self,
        explain: &bool,
        current_chips: &Chips,
        current_mult: &Mult,
        played_card: &ScoringPlayedCard,
        scored_card_index: usize,
        data: &ScoringData
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = *current_chips;
        let mut curr_mult: Mult = *current_mult;

        let scored_cards = &data.scored_cards;
        let mut is_smeared: bool = false;
        data.joker_cards.iter().for_each(|joker_card| {
            match joker_card.joker {
                Joker::SmearedJoker => {
                    is_smeared = true;
                }
                _ => {}
            }
        });

        match self.joker {
            Joker::GreedyJoker => {
                if
                    played_card.suit == Suit::Diamonds ||
                    played_card.enhancement == Some(Enhancement::Wild) ||
                    (is_smeared && played_card.suit == Suit::Hearts)
                {
                    curr_mult += 3.0;
                    if *explain {
                        println!(
                            "Greedy Joker {:?}{:?} +3 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::LustyJoker => {
                if
                    played_card.suit == Suit::Hearts ||
                    played_card.enhancement == Some(Enhancement::Wild) ||
                    (is_smeared && played_card.suit == Suit::Diamonds)
                {
                    curr_mult += 3.0;
                    if *explain {
                        println!(
                            "Lusty Joker {:?}{:?} +3 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::WrathfulJoker => {
                if
                    played_card.suit == Suit::Spades ||
                    played_card.enhancement == Some(Enhancement::Wild) ||
                    (is_smeared && played_card.suit == Suit::Clubs)
                {
                    curr_mult += 3.0;
                    if *explain {
                        println!(
                            "Wrathful Joker {:?}{:?} +3 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::GluttonousJoker => {
                if
                    played_card.suit == Suit::Clubs ||
                    played_card.enhancement == Some(Enhancement::Wild) ||
                    (is_smeared && played_card.suit == Suit::Spades)
                {
                    curr_mult += 3.0;
                    if *explain {
                        println!(
                            "Gluttonous Joker {:?}{:?} +3 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::Fibonacci => {
                if
                    played_card.rank == Rank::Ace ||
                    played_card.rank == Rank::Two ||
                    played_card.rank == Rank::Three ||
                    played_card.rank == Rank::Five ||
                    played_card.rank == Rank::Eight
                {
                    curr_mult += 8.0;
                    if *explain {
                        println!(
                            "Fibonacci Joker {:?}{:?} +8 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::ScaryFace => {
                if played_card.is_face {
                    curr_chips += 30.0;
                    if *explain {
                        println!(
                            "Scary Joker {:?}{:?} +30 Chips ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::EvenSteven => {
                if !played_card.rank.is_face() && played_card.rank.rank_value() % 2.0 == 0.0 {
                    curr_mult += 4.0;
                    if *explain {
                        println!(
                            "Even Steven Joker {:?}{:?} +4 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::OddTodd => {
                if !played_card.rank.is_face() && played_card.rank.rank_value() % 2.0 == 1.0 {
                    curr_chips += 31.0;
                    if *explain {
                        println!(
                            "Odd Todd Joker {:?}{:?} +31 Chips ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::Photograph => {
                let first_face_card_index = scored_cards.iter().position(|card| card.is_face);
                if *explain {
                    println!("{:?} {:?}", first_face_card_index, scored_card_index);
                }
                if first_face_card_index == Some(scored_card_index) {
                    curr_mult *= 2.0;
                    if *explain {
                        println!(
                            "Photograph Joker {:?}{:?} x2 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            Joker::SmileyFace => {
                if played_card.is_face {
                    curr_mult += 5.0;
                    if *explain {
                        println!(
                            "Smiley Face Joker {:?}{:?} +5 Mult ( {:?} x {:?} )",
                            played_card.rank,
                            played_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            _ => {}
        }
        return (curr_chips, curr_mult);
    }

    fn calculate_on_held_cards(
        &self,
        explain: &bool,
        current_chips: &Chips,
        current_mult: &Mult,
        held_card: &HandCard,
        data: &ScoringData
    ) -> (Chips, Mult) {
        let curr_chips: Chips = *current_chips;
        let mut curr_mult: Mult = *current_mult;
        let cards_in_hand = &data.hand_cards;

        match self.joker {
            Joker::RaisedFist => {
                let smallest_card = cards_in_hand
                    .iter()
                    .enumerate() // Enumerate to get the index
                    .rev() // Reverse the order
                    .min_by_key(|(_, card)| card.rank.rank_value() as u8); // Get the smallest card

                if let Some((index, card)) = smallest_card {
                    if index == held_card.hand_card_index {
                        let increase_value = 2.0 * card.rank.rank_value();
                        curr_mult += increase_value;
                        if *explain {
                            println!(
                                "Raised Fist Joker {:?}{:?} + {:?} Mult ( {:?} x {:?} )",
                                card.rank,
                                card.suit,
                                increase_value,
                                curr_chips,
                                curr_mult
                            );
                        }
                    }
                }
            }
            Joker::Baron => {
                if held_card.rank == Rank::King {
                    curr_mult *= 1.5;
                    if *explain {
                        println!(
                            "Baron Joker {:?}{:?} 1.5x Mult ( {:?} x {:?} )",
                            held_card.rank,
                            held_card.suit,
                            curr_chips,
                            curr_mult
                        );
                    }
                }
            }
            _ => {}
        }
        return (curr_chips, curr_mult);
    }
}
