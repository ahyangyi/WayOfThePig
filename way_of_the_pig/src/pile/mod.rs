use crate::game;

pub mod none;

pub trait Pile {
    fn make() -> Self;
    fn enabled(&self) -> bool;
    fn top(&self) -> Option<game::CardType>;
    fn pop(&mut self) -> Option<game::CardType>;
    fn remaining_cards(&self) -> u8;
}

macro_rules! make_simple_pile {
    ( $pile:ident, $card:ident ) => {
        pub mod $pile {
            use crate::game;

            pub struct Pile {
                cards: u8,
            }

            impl crate::pile::Pile for Pile {
                #[inline]
                fn make() -> Self {
                    // FIXME depend on player count
                    Pile {
                        cards: 8,
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

make_simple_pile!(province, Province);
make_simple_pile!(colony, Colony);
