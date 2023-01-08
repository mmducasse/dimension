use std::str::FromStr;

use xf::num::ivec2::{IVec2, i2};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Glasses,
    Gloves,
    Boots,
    Snorkel,
    KeyRed,
    KeyGreen,
    KeyBlue,
}

impl ItemType {
    pub const COUNT: usize = 7;

    pub const ALL: [ItemType; Self::COUNT] = [
        ItemType::Glasses,
        ItemType::Gloves,
        ItemType::Boots,
        ItemType::Snorkel,
        ItemType::KeyRed,
        ItemType::KeyGreen,
        ItemType::KeyBlue,
    ];

    pub fn p16_src(self) -> IVec2 {
        use ItemType::*;

        match self {
            Glasses => i2(0, 0),
            Gloves => i2(1, 0),
            Boots => i2(2, 0),
            Snorkel => i2(3, 0),
            KeyRed => i2(0, 1),
            KeyGreen => i2(1, 1),
            KeyBlue => i2(2, 1),
        }
    }
}

impl FromStr for ItemType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ItemType::*;

        let type_ = match s {
            "Glasses" => Glasses,
            "Gloves" => Gloves,
            "Boots" => Boots,
            "Snorkel" => Snorkel,
            _ => {
                panic!("Unexpected item type: {}", s)
            }
        };

        Ok(type_)
    }
}
