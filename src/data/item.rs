use xf::num::ivec2::{IVec2, i2};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Glasses,
    Gloves,
    Boots,


}

impl ItemType {
    pub const COUNT: usize = 3;

    pub const ALL: [ItemType; Self::COUNT] = [
        ItemType::Glasses,
        ItemType::Gloves,
        ItemType::Boots,
    ];

    pub fn p16_src(self) -> IVec2 {
        use ItemType::*;

        match self {
            Glasses => i2(0, 0),
            Gloves => i2(1, 0),
            Boots => i2(2, 0),
        }
    }
}

