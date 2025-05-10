use ortalib::{ Chips, Mult, Edition, Enhancement };

pub fn get_played_enhancements(
    enhancement: Option<Enhancement>,
    explain: &bool,
    curr_chips: &Chips,
    curr_mult: &Mult,
    card_title: &String
) -> (Chips, Mult) {
    let mut bonus_chips: Chips = *curr_chips;
    let mut bonus_mult: Mult = *curr_mult;
    match enhancement {
        Some(Enhancement::Bonus) => {
            bonus_chips += 30.0;
            if *explain {
                println!(
                    "{} Bonus +30 Chips( {:?} x {:?} )",
                    card_title,
                    bonus_chips,
                    bonus_mult
                );
            }
        }
        Some(Enhancement::Mult) => {
            bonus_mult += 4.0;
            if *explain {
                println!(
                    "{} Mult +4 Mult ( {:?} x {:?} )",
                    card_title,
                    bonus_chips,
                    bonus_mult
                );
            }
        }
        Some(Enhancement::Glass) => {
            bonus_mult *= 2.0;
            if *explain {
                println!(
                    "{} Glass x2 Mult ( {:?} x {:?} )",
                    card_title,
                    bonus_chips,
                    bonus_mult
                );
            }
        }
        _ => {}
    }
    return (bonus_chips, bonus_mult);
}


pub fn get_editions(
    edition: Option<Edition>,
    card_title: &String,
    explain: &bool,
    curr_chips: &Chips,
    curr_mult: &Mult
) -> (Chips, Mult) {
    let (bonus_chips, bonus_mult) = match edition {
        Some(Edition::Polychrome) => get_polychrome(card_title, explain, curr_chips, curr_mult),
        Some(_) => get_foil_holo(edition, card_title, explain, curr_chips, curr_mult),
        _ => (*curr_chips, *curr_mult),
    };
    (bonus_chips, bonus_mult)
}

pub fn get_foil_holo(
    edition: Option<Edition>,
    card_title: &String,
    explain: &bool,
    curr_chips: &Chips,
    curr_mult: &Mult
) -> (Chips, Mult) {
    let mut bonus_chips = *curr_chips;
    let mut bonus_mult = *curr_mult;
    match edition {
        Some(Edition::Foil) => {
            bonus_chips += 50.0;
            if *explain {
                println!(
                    "{} Foil +50 Chips ( {:?} x {:?} )",
                    card_title, bonus_chips, bonus_mult
                );
            }
        }
        Some(Edition::Holographic) => {
            bonus_mult += 10.0;
            if *explain {
                println!(
                    "{} Holographic +10 Mult ( {:?} x {:?} )",
                    card_title, bonus_chips, bonus_mult
                );
            }
        }
        _ => {}
    }
    (bonus_chips, bonus_mult)
}

pub fn get_polychrome(
    card_title: &String,
    explain: &bool,
    curr_chips: &Chips,
    curr_mult: &Mult
) -> (Chips, Mult) {
    let bonus_chips = *curr_chips;
    let bonus_mult = *curr_mult * 1.5;
    if *explain {
        println!(
            "{} Polychrome x1.5 Mult ( {:?} x {:?} )",
            card_title, bonus_chips, bonus_mult
        );
    }
    (bonus_chips, bonus_mult)
}

