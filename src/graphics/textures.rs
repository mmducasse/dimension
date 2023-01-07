use std::rc::Rc;

use xf::gl::texture::Texture;

use super::image::xf_texture_from_bytes;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum TextureId {
    // Player, NPCs, Mobs
    Player,

    // UI
    Hud,
}

const fn get_bytes(id: TextureId) -> &'static [u8] {
    use TextureId::*;

    match id {
        Player => include_bytes!("../../assets/Sprites/Player.png"),
        Hud => include_bytes!("../../assets/Sprites/Hud.png")
    }
}

static mut TEXTURE_CACHE: [Option<Rc<Texture>>; 2] = [None, None];

impl TextureId {
    /// Loads the texture associated with this ID.
    pub fn texture(self) -> Rc<Texture> {
        unsafe {
            let idx = self as usize;
            if TEXTURE_CACHE[idx].is_none() {
                let bytes = get_bytes(self);
                let texture = xf_texture_from_bytes(bytes);
                TEXTURE_CACHE[idx] = Some(Rc::new(texture));
            }

            if let Some(texture_ref) = &TEXTURE_CACHE[idx] {
                return texture_ref.clone()
            } else {
                panic!("Texture wasn't cached for some reason")
            }
        }
    }
}