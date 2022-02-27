use crate::pile;

pub struct Pile {
    provinces: u8,
}

impl pile::Pile for Pile {
    fn make() -> Self {
        // FIXME depend on player count
        Pile {
            provinces: 8,
        }
    }
}
