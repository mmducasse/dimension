use xf::num::ivec2::IVec2;



pub const fn add(a: IVec2, b: IVec2) -> IVec2 {
    IVec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

pub const fn mul(a: IVec2, b: IVec2) -> IVec2 {
    IVec2 {
        x: a.x * b.x,
        y: a.y * b.y,
    }
}