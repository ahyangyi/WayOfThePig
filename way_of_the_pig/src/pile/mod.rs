use crate::game;

pub mod none;

pub trait Pile {
    fn make<const N: usize>() -> Self;
    fn enabled(&self) -> bool;
    fn top(&self) -> Option<game::CardType>;
    fn pop(&mut self) -> Option<game::CardType>;
    fn remaining_cards(&self) -> u8;
}

macro_rules! make_simple_pile {
    ( $pile:ident, $card:ident, $card_count:expr ) => {
        pub mod $pile {
            use crate::game;

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
                fn top(&self) -> Option<game::CardType> {
                    if self.cards == 0 {
                        return None;
                    }
                    Some(game::CardType::$card)
                }

                #[inline]
                fn pop(&mut self) -> Option<game::CardType> {
                    if self.cards == 0 {
                        return None;
                    }
                    self.cards -= 1;
                    Some(game::CardType::$card)
                }

                #[inline]
                fn remaining_cards(&self) -> u8 {
                    self.cards
                }
            }
        }
    };
}

make_simple_pile!(province, Province, [8, 8, 12, 15, 18]);
make_simple_pile!(duchy, Duchy, [8, 8, 12, 12, 12]);
make_simple_pile!(estate, Estate, [8, 8, 12, 12, 12]);
make_simple_pile!(gold, Gold, [30, 30, 30, 60, 60]);
make_simple_pile!(silver, Silver, [40, 40, 40, 80, 80]);
make_simple_pile!(copper, Copper, [46, 39, 32, 85, 78]);
make_simple_pile!(curse, Curse, [10, 20, 30, 40, 50]);

make_simple_pile!(colony, Colony, [8, 8, 12, 12, 12]);
make_simple_pile!(platinum, Platinum, [12, 12, 12, 12, 12]);

make_simple_pile!(smithy, Smithy, [10, 10, 10, 10, 10]);
make_simple_pile!(patrol, Patrol, [10, 10, 10, 10, 10]);
