#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side {
    Player,
    AI,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            Side::Player => Side::AI,
            Side::AI => Side::Player,
        }
    }
}
