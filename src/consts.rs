
use xf::num::ivec2::{i2, IVec2};

use crate::common::ivec2::mul;


pub const P16: IVec2 = i2(16, 16);

pub const SCREEN_P16_SIZE: IVec2 = i2(14, 10);
pub const SCREEN_SIZE: IVec2 = mul(SCREEN_P16_SIZE, P16);