use ortalib::{
    Card,
    Chips,
    Mult,
    PokerHand,
    JokerCard,
    Round,
    Edition,
    Enhancement,
    Rank,
    Suit,
    Joker,
};
use crate::jokers::JokerCardCalculator;
use crate::pokerservice::evaluate_hand;
use crate::modifiers::{ get_editions, get_played_enhancements };

/// Trait defining scoring behaviour for On Scored Cards and On Held Cards.
pub trait ScoringCard {
    fn get_values(
        &self,
        explain: &bool,
        curr_chips: &Chips,
        curr_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult);

    fn add_scores(
        &self,
        explain: &bool,
        curr_chips: &Chips,
        curr_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult);

    fn check_jokers(
        &self,
        explain: &bool,
        bonus_chips: &mut Chips,
        bonus_mult: &mut Mult,
        data: &ScoringData<'_>
    );

    fn retriggers(
        &self,
        explain: &bool,
        bonus_chips: &Chips,
        bonus_mult: &Mult,
        data: &ScoringData<'_>
    ) -> (Chips, Mult);
}

/// Data structure storing round data
pub struct ScoringData<'a> {
    pub played_cards: Vec<Card>,
    pub scored_cards: Vec<ScoringPlayedCard>,
    pub round: &'a Round,
    pub pokerhands: Vec<PokerHand>,
    pub hand_cards: Vec<HandCard>,
    pub joker_cards: Vec<JokerCard>,
}

/// Card which scored and was also played
#[derive(Debug, Clone, Copy)]
pub struct ScoringPlayedCard {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Option<Enhancement>,
    pub edition: Option<Edition>,
    pub scored_card_index: usize,
    pub is_face: bool,
}

/// Card in hand which can score
#[derive(Debug, Clone, Copy)]
pub struct HandCard {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Option<Enhancement>,
    pub hand_card_index: usize,
}

impl ScoringData<'_> {
    /// Returns final Chips and Multiplier
    pub fn get_score(&mut self, explain: &bool) -> (Chips, Mult) {
        // Finds the strongest pokerhand and puts into scored_cards
        let (pokerhand_chips, pokerhand_mult) = self.find_poker_hand(&explain);

        self.joker_cards = self.round.jokers.clone();
        // Splash, Pareidolia and Blueprint jokers
        self.set_up_post_pokerhand_jokers(explain);

        // Add cards in hand to data
        self.round.cards_held_in_hand
            .iter()
            .enumerate()
            .for_each(|(index, card)| {
                self.hand_cards.push(HandCard {
                    rank: card.rank,
                    suit: card.suit,
                    enhancement: card.enhancement,
                    hand_card_index: index,
                });
            });

        // Calculate Score of Scored Cards and Held cards
        let (scoredcard_chips, scoredcard_mult) = self.add_scoring_cards(
            &explain,
            pokerhand_chips,
            pokerhand_mult
        );

        // Joker editions and indepdendent Jokers
        let (final_chips, final_mult) = self.add_joker_cards(
            &explain,
            scoredcard_chips,
            scoredcard_mult
        );

        return (final_chips, final_mult);
    }

    fn set_up_post_pokerhand_jokers(&mut self, explain: &bool) {
        // Splash
        self.joker_cards
            .clone()
            .iter()
            .for_each(|joker| {
                joker.set_splash(self);
            });

        // Pareidolia
        self.joker_cards
            .clone()
            .iter()
            .for_each(|joker| {
                joker.set_pareidolia(self);
            });

        // Blueprint
        let mut reversed_jokers = self.joker_cards.clone();
        reversed_jokers.reverse();

        if reversed_jokers.len() > 1 {
            for i in 1..reversed_jokers.len() {
                if reversed_jokers[i].joker == Joker::Blueprint {
                    reversed_jokers[i].joker = reversed_jokers[i - 1].clone().joker;
                }
            }
        }
        reversed_jokers.reverse();
        self.joker_cards = reversed_jokers;

        if *explain {
            println!("blueprint fixed cards {:?}", self.joker_cards);
        }
    }

    /// Get Poker Hand
    fn find_poker_hand(&mut self, explain: &bool) -> (Chips, Mult) {
        // Finds Pokerhand and manages Four Fingers, Shortcut and Smeared Joker
        let (result, scored_cards) = evaluate_hand(&self.played_cards, &self);

        self.scored_cards = scored_cards;
        let poker_hand = result.get(0).unwrap().clone();
        self.pokerhands = result;
        if *explain {
            println!("{:?} {:?}", poker_hand, poker_hand.hand_value());
        }
        return poker_hand.hand_value();
    }

    /// Adds chips and mults from played and held cards including their jokers
    fn add_scoring_cards(
        &self,
        explain: &bool,
        pokerhand_chips: Chips,
        pokerhand_mult: Mult
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = pokerhand_chips;
        let mut curr_mult: Mult = pokerhand_mult;

        // Each card is a ScoringCard and thus can implement get_values
        self.scored_cards.iter().for_each(|card| {
            let (new_chips, new_mult) = card.get_values(&explain, &curr_chips, &curr_mult, &self);
            curr_chips = new_chips;
            curr_mult = new_mult;
        });

        self.hand_cards.iter().for_each(|card| {
            let (card_chips, card_mult) = card.get_values(&explain, &curr_chips, &curr_mult, &self);
            curr_chips = card_chips;
            curr_mult = card_mult;
        });

        return (curr_chips, curr_mult);
    }

    /// Adds independent joker card chips and mult
    fn add_joker_cards(
        &self,
        explain: &bool,
        current_chips: Chips,
        current_mult: Mult
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = current_chips;
        let mut curr_mult: Mult = current_mult;

        self.joker_cards.iter().for_each(|joker| {
            let (new_chips, new_mult) = joker.calculate_independent_cards(
                explain,
                &curr_chips,
                &curr_mult,
                &self
            );

            curr_chips = new_chips;
            curr_mult = new_mult;
        });

        return (curr_chips, curr_mult);
    }
}

// ScoredPlayingCard Implementation of Scoring Card Functions
impl ScoringCard for ScoringPlayedCard {
    fn add_scores(
        &self,
        explain: &bool,
        curr_chips: &Chips,
        curr_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult) {
        let mut bonus_chips: Chips = *curr_chips;
        let mut bonus_mult: Mult = *curr_mult;

        bonus_chips += self.rank.rank_value();
        if *explain {
            println!(
                "{:?}{:?} +{:?} Chips ( {:?} x {:?} )",
                self.rank,
                self.suit,
                self.rank.rank_value(),
                bonus_chips,
                bonus_mult
            );
        }

        let card_title = format!("{:?}{:?}", self.rank, self.suit);
        let (enhancement_chips, enhancement_mult) = get_played_enhancements(
            self.enhancement,
            explain,
            &bonus_chips,
            &bonus_mult,
            &card_title
        );

        let (edition_chips, edition_mult) = get_editions(
            self.edition,
            &card_title,
            explain,
            &enhancement_chips,
            &enhancement_mult
        );
        bonus_chips = edition_chips;
        bonus_mult = edition_mult;

        self.check_jokers(explain, &mut bonus_chips, &mut bonus_mult, &data);
        return (bonus_chips, bonus_mult);
    }

    fn get_values(
        &self,
        explain: &bool,
        curr_chips: &Chips,
        curr_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult) {
        let (new_chips, new_mult) = self.add_scores(explain, &curr_chips, &curr_mult, &data);
        let (retriggered_chips, retriggered_mult) = self.retriggers(
            explain,
            &new_chips,
            &new_mult,
            &data
        );
        return (retriggered_chips, retriggered_mult);
    }

    fn retriggers(
        &self,
        explain: &bool,
        bonus_chips: &Chips,
        bonus_mult: &Mult,
        data: &ScoringData<'_>
        // has_retriggered: &mut bool
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = *bonus_chips;
        let mut curr_mult: Chips = *bonus_mult;
        // Sock and buskin: Retriggers face cards
        data.joker_cards
            .iter()
            .filter(|joker| joker.joker == Joker::SockAndBuskin)
            .for_each(|_| {
                if self.is_face {
                    let (new_chips, new_mult) = self.add_scores(
                        &explain,
                        &curr_chips,
                        &curr_mult,
                        &data
                    );
                    curr_chips = new_chips;
                    curr_mult = new_mult;
                }
            });
        return (curr_chips, curr_mult);
    }

    fn check_jokers(
        &self,
        explain: &bool,
        bonus_chips: &mut Chips,
        bonus_mult: &mut Mult,
        data: &ScoringData<'_>
    ) {
        data.joker_cards.iter().for_each(|joker| {
            let (joker_chips, joker_mult) = joker.calculate_on_played_cards(
                explain,
                bonus_chips,
                bonus_mult,
                &self,
                self.scored_card_index,
                &data
            );
            *bonus_chips = joker_chips;
            *bonus_mult = joker_mult;
        });
    }
}

impl PartialEq for ScoringPlayedCard {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank &&
            self.suit == other.suit &&
            self.enhancement == other.enhancement &&
            self.edition == other.edition
    }
}

impl ScoringCard for HandCard {
    fn get_values(
        &self,
        explain: &bool,
        curr_chips: &Chips,
        curr_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult) {
        let (new_chips, new_mult) = self.add_scores(explain, &curr_chips, &curr_mult, &data);
        let (retriggered_chips, retriggered_mult) = self.retriggers(
            explain,
            &new_chips,
            &new_mult,
            &data
        );
        return (retriggered_chips, retriggered_mult);
    }

    fn check_jokers(
        &self,
        explain: &bool,
        bonus_chips: &mut Chips,
        bonus_mult: &mut Mult,
        data: &ScoringData<'_>
    ) {
        data.joker_cards.iter().for_each(|joker| {
            let (joker_chips, joker_mult) = joker.calculate_on_held_cards(
                explain,
                bonus_chips,
                bonus_mult,
                &self,
                &data
            );
            *bonus_chips = joker_chips;
            *bonus_mult = joker_mult;
        });
    }

    fn add_scores(
        &self,
        explain: &bool,
        curr_chips: &Chips,
        curr_mult: &Mult,
        data: &ScoringData
    ) -> (Chips, Mult) {
        let mut bonus_chips: Chips = *curr_chips;
        let mut bonus_mult: Mult = *curr_mult;
        if let Some(enhancement) = self.enhancement {
            if enhancement == Enhancement::Steel {
                bonus_mult *= 1.5;
                if *explain {
                    println!(
                        "{:?}{:?} Steel x1.5 Mult ( {:?} x {:?} )",
                        self.rank,
                        self.suit,
                        bonus_chips,
                        bonus_mult
                    );
                }
            }
        }

        self.check_jokers(explain, &mut bonus_chips, &mut bonus_mult, &data);

        return (bonus_chips, bonus_mult);
    }

    fn retriggers(
        &self,
        explain: &bool,
        bonus_chips: &Chips,
        bonus_mult: &Mult,
        data: &ScoringData<'_>
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = *bonus_chips;
        let mut curr_mult: Chips = *bonus_mult;
        // Mime Joker
        data.joker_cards
            .iter()
            .filter(|joker| joker.joker == Joker::Mime)
            .for_each(|_| {
                let (new_chips, new_mult) = self.add_scores(
                    &explain,
                    &curr_chips,
                    &curr_mult,
                    &data
                );
                curr_chips = new_chips;
                curr_mult = new_mult;
            });
        return (curr_chips, curr_mult);
    }
}
