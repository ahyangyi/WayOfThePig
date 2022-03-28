use crate::card;

pub mod none;

pub trait Pile {
    fn make<const N: usize>() -> Self;
    fn enabled(&self) -> bool;
    fn top(&self) -> Option<card::CardType>;
    fn pop(&mut self) -> Option<card::CardType>;
    fn remaining_cards(&self) -> u8;
}

macro_rules! make_simple_pile {
    ( $pile:ident, $card:ident, $card_count:expr ) => {
        pub mod $pile {
            use crate::card;

            pub struct Pile {
                cards: u8,
            }

            impl crate::pile::Pile for Pile {
                #[inline]
                fn make<const N: usize>() -> Self {
                    // FIXME depend on player count
                    Pile {
                        cards: $card_count[N - 2],
                    }
                }

                #[inline]
                fn enabled(&self) -> bool {
                    true
                }

                #[inline]
                fn top(&self) -> Option<card::CardType> {
                    if self.cards == 0 {
                        return None;
                    }
                    Some(card::CardType::$card)
                }

                #[inline]
                fn pop(&mut self) -> Option<card::CardType> {
                    if self.cards == 0 {
                        return None;
                    }
                    self.cards -= 1;
                    Some(card::CardType::$card)
                }

                #[inline]
                fn remaining_cards(&self) -> u8 {
                    self.cards
                }
            }
        }
    };
}

macro_rules! make_simple_action_pile {
    ( $pile:ident, $card:ident) => {
        make_simple_pile!($pile, $card, [10, 10, 10, 10, 10]);
    };
}

macro_rules! make_simple_victory_pile {
    ( $pile:ident, $card:ident) => {
        make_simple_pile!($pile, $card, [8, 8, 12, 12, 12]);
    };
}

make_simple_pile!(province, Province, [8, 8, 12, 15, 18]);
make_simple_victory_pile!(duchy, Duchy);
make_simple_victory_pile!(estate, Estate);
make_simple_victory_pile!(gold, Gold);
make_simple_pile!(silver, Silver, [40, 40, 40, 80, 80]);
make_simple_pile!(copper, Copper, [46, 39, 32, 85, 78]);
make_simple_pile!(curse, Curse, [10, 20, 30, 40, 50]);

make_simple_victory_pile!(colony, Colony);
make_simple_pile!(platinum, Platinum, [12, 12, 12, 12, 12]);

// Base
make_simple_action_pile!(market, Market);
make_simple_action_pile!(militia, Militia);
make_simple_action_pile!(smithy, Smithy);
make_simple_action_pile!(village, Village);

// Intrigue
make_simple_victory_pile!(harem, Harem);
make_simple_action_pile!(patrol, Patrol);

// Hinterlands
make_simple_action_pile!(oasis, Oasis);

// Nocturne
make_simple_action_pile!(faithful_hound, FaithfulHound);
