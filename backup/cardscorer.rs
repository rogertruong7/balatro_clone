use ortalib::{ Card, Chips, Mult, PokerHand, Round, Edition, Enhancement, Rank, Suit, Joker };
use crate::jokers::JokerCardCalculator;
use crate::pokerservice::evaluate_hand;
use crate::modifiers::{ get_editions, get_played_enhancements };

pub trait ScoringCard {
    fn get_values(
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
    );
    // Other methods if needed
}

pub struct ScoringData<'a> {
    pub played_cards: Vec<Card>,
    pub scored_cards: Vec<ScoringPlayedCard>,
    pub round: &'a Round,
    pub pokerhands: Vec<PokerHand>,
    pub hand_cards: Vec<HandCard>,
}

// Card which scored and was also played
#[derive(Debug, Clone, Copy)]
pub struct ScoringPlayedCard {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Option<Enhancement>,
    pub edition: Option<Edition>,
    pub scored_card_index: usize,
    pub is_face: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct HandCard {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Option<Enhancement>,
    pub hand_card_index: usize,
}

impl ScoringData<'_> {
    pub fn get_score(&mut self, explain: &bool) -> (Chips, Mult) {
        // TODO Four fingers, Shortcut Jokers, Smeared Joker, set blueprint

        // Puts the pokerhand into scored_cards
        let (pokerhand_chips, pokerhand_mult) = self.find_poker_hand(&explain);
        // Pareidolia, Splash

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
        self.round.jokers
            .clone()
            .iter()
            .for_each(|joker| {
                joker.set_splash(self);
            });
        if *explain {
            println!("-----------------");
            println!("SPLASH UPDATED Scored Cards:");
            self.scored_cards.iter().for_each(|card| {
                println!("{:?}{:?}", card.rank, card.suit);
            });
            println!("-----------------");
        }

        // Pareidolia
        self.round.jokers
            .clone()
            .iter()
            .for_each(|joker| {
                joker.set_pareidolia(self);
            });
        if *explain {
            println!("-----------------");
            println!("Pareidolia UPDATED Face Cards:");
            self.scored_cards.iter().for_each(|card| {
                println!("{:?}{:?} is face card? {:?}", card.rank, card.suit, card.is_face);
            });
            println!("-----------------");
        }
    }

    // Get Poker Hand
    fn find_poker_hand(&mut self, explain: &bool) -> (Chips, Mult) {
        // TODO Four fingers, Shortcut Jokers
        // TODO in evaluatehand, for SmearedJoker, now theres only 2 suits (color)
        let (result, scored_cards) = evaluate_hand(&self.played_cards);
        if *explain {
            println!("Pokerhands found {:?}", result);
            println!("Scored Cards {:?}", scored_cards);
        }

        self.scored_cards = scored_cards;
        let poker_hand = result.get(0).unwrap().clone();
        self.pokerhands = result;
        if *explain {
            println!("{:?} {:?}", poker_hand, poker_hand.hand_value());
        }
        return poker_hand.hand_value();
    }

    fn add_scoring_cards(
        &self,
        explain: &bool,
        pokerhand_chips: Chips,
        pokerhand_mult: Mult
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = pokerhand_chips;
        let mut curr_mult: Mult = pokerhand_mult;

        // Each card is a ScoringCard and thus can implement get_values
        // get_values adds Base Chips, applies modifiers and adds On scored jokers
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

        // TODO Mime

        return (curr_chips, curr_mult);
    }

    fn add_joker_cards(
        &self,
        explain: &bool,
        current_chips: Chips,
        current_mult: Mult
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = current_chips;
        let mut curr_mult: Mult = current_mult;

        self.round.jokers.iter().for_each(|joker| {
            let (new_chips, new_mult) = joker.calculate_independent_cards(
                explain,
                &curr_chips,
                &curr_mult,
                &self
            );

            curr_chips = new_chips;
            curr_mult = new_mult;
        });
        // Each card is a ScoringCard and thus can implement get_values

        return (curr_chips, curr_mult);
    }
}

impl ScoringCard for ScoringPlayedCard {
    fn get_values(
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

    fn retriggers(
        &self,
        explain: &bool,
        bonus_chips: &Chips,
        bonus_mult: &Mult,
        data: &ScoringData<'_>
    ) -> (Chips, Mult) {
        let mut curr_chips: Chips = *bonus_chips;
        let mut curr_mult: Chips = *bonus_mult;
        // Sock and buskin: Retriggers face cards
        data.round.jokers
            .iter()
            .filter(|joker| joker.joker == Joker::SockAndBuskin)
            .for_each(|joker| {
                if self.is_face {
                    let (new_chips, new_mult) = card.get_values(
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
        bonus_chips: &Chips,
        bonus_mult: &Mult,
        data: &ScoringData<'_>
    ) {
        data.round.jokers.iter().for_each(|joker| {
            let (joker_chips, joker_mult) = joker.calculate_on_played_cards(
                explain,
                bonus_chips,
                bonus_mult,
                &self,
                &data.scored_cards,
                self.scored_card_index
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

    fn check_jokers(
        &self,
        explain: &bool,
        bonus_chips: &mut Chips,
        bonus_mult: &mut Mult,
        data: &ScoringData<'_>
    ) {
        data.round.jokers.iter().for_each(|joker| {
            let (joker_chips, joker_mult) = joker.calculate_on_held_cards(
                explain,
                bonus_chips,
                bonus_mult,
                &self,
                &data.hand_cards
            );
            *bonus_chips = joker_chips;
            *bonus_mult = joker_mult;
        });
    }
    
    fn retriggers(
        &self,
        explain: &bool,
        bonus_chips: &mut Chips,
        bonus_mult: &mut Mult,
        data: &ScoringData<'_>
    ) {
        todo!()
    }
}
