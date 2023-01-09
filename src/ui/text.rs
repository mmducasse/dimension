use xf::num::{ivec2::{IVec2, i2}, irect::{ir, rect, IRect}};

use crate::graphics::{textures::TextureId, buffer::draw_texture};


const CHAR_SPACING: IVec2 = i2(6, 8);
const CHAR_BOUNDS: IRect = rect(0, 1, 6, 6);

pub fn draw(s: &str, org: IVec2) {
    let texture = TextureId::Text.texture();

    let mut pos = org;

    for c in s.chars() {
        let src_pos = lookup(c);
        let src = ir(src_pos + CHAR_BOUNDS.pos, CHAR_BOUNDS.size);
        draw_texture(&texture, src, pos);

        pos.x += CHAR_SPACING.x;
    }
}

fn lookup(c: char) -> IVec2 {
    fn rel(c1: char, c2: char) -> usize {
        c1 as usize - c2 as usize
    }

    let src_pos = match c {
        _ if c.is_uppercase() => todo!(),
        _ if c.is_lowercase() => todo!(),
        _ if c.is_numeric() => i2(rel(c, '0') as i32, 4),
        '.' => i2(0, 5),
        ',' => i2(1, 5),
        '!' => i2(2, 5),
        '?' => i2(3, 5),
        ':' => i2(4, 5),
        ';' => i2(5, 5),
        '$' => i2(6, 5),
        '(' => i2(7, 5),
        ')' => i2(8, 5),
        ' ' => i2(9, 5),
        _ => i2(3, 5),
    };

    src_pos * CHAR_SPACING
}