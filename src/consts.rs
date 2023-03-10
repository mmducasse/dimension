
use xf::num::{ivec2::{i2, IVec2}, fvec2::{FVec2, f2}};

use crate::common::ivec2::mul;

pub const GRAVITY: FVec2 = f2(0.0, 0.2);

pub const P8: IVec2 = i2(8, 8);
pub const P16: IVec2 = i2(16, 16);

pub const VIEW_P16_ORIGIN: IVec2 = i2(0, 0);
pub const VIEW_P16_SIZE: IVec2 = i2(16, 14);
pub const VIEW_SIZE: IVec2 = mul(VIEW_P16_SIZE, P16);

pub const HUD_P16_ORIGIN: IVec2 = i2(0, VIEW_P16_SIZE.y);
pub const HUD_ORIGIN: IVec2 = mul(HUD_P16_ORIGIN, P16);
pub const HUD_P16_SIZE: IVec2 = i2(16, 2);
pub const HUD_SIZE: IVec2 = mul(HUD_P16_SIZE, P16);

pub const SCREEN_P16_SIZE: IVec2 = i2(16, 16);
pub const SCREEN_SIZE: IVec2 = mul(SCREEN_P16_SIZE, P16);