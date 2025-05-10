use ortalib::{ Chips, Mult, Round };

use crate::cardscorer::ScoringData;

pub struct RoundScorer {
    round: Round,
    total_chips: f64,
    total_mult: f64,
}

impl RoundScorer {
    pub fn new(round: Round) -> RoundScorer {
        RoundScorer { round, total_chips: 0.0, total_mult: 0.0 }
    }

    pub fn get_score(&mut self, explain: bool) -> (Chips, Mult) {
        self.calculate(explain);
        return (self.total_chips, self.total_mult);
    }

    fn calculate(&mut self, explain: bool) {
        if explain {
            println!("{:?}", self.round);
        }

        let round = &self.round;
        let mut scoring_data = ScoringData {
            played_cards: round.cards_played.clone(),
            scored_cards: Vec::new(),
            round,
            pokerhands: Vec::new(),
            hand_cards: Vec::new()
        };

        let (played_chips, played_mult) = scoring_data.get_score(&explain);
        self.total_chips += played_chips;
        self.total_mult += played_mult;
    }
}
