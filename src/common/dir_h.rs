use xf::{data::dir4::Dir4, num::ivec2::IVec2};


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DirH {
    L,
    R,
}

impl DirH {
    pub fn to_dir4(self) -> Dir4 {
        match self {
            DirH::L => Dir4::W,
            DirH::R => Dir4::E,
        }
    }

    pub fn unit(self) -> IVec2 {
        self.to_dir4().unit()
    }
}