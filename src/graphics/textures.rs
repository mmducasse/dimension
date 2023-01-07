use std::rc::Rc;

use xf::gl::texture::Texture;

use super::image::xf_texture_from_bytes;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum TextureId {
    // Player, NPCs, Mobs, Items
    Player,
    Items,

    // UI
    Hud,
}

const COUNT: usize = 3;

const fn get_bytes(id: TextureId) -> &'static [u8] {
    use TextureId::*;

    match id {
        Player => include_bytes!("../../assets/Sprites/Player.png"),
        Items => include_bytes!("../../assets/Sprites/Items.png"),
        Hud => include_bytes!("../../assets/Sprites/Hud.png")
    }
}

static mut TEXTURE_CACHE: Option<Vec<Option<Rc<Texture>>>> = None;

impl TextureId {
    /// Loads the texture associated with this ID.
    pub fn texture(self) -> Rc<Texture> {
        unsafe {
            if let None = TEXTURE_CACHE {
                let mut vec = vec![];
                for _ in 0..COUNT {
                    vec.push(None);
                }

                TEXTURE_CACHE = Some(vec);
            }

            if let Some(texture_cache) = &mut TEXTURE_CACHE {
                let idx = self as usize;
                if texture_cache[idx].is_none() {
                    let bytes = get_bytes(self);
                    let texture = xf_texture_from_bytes(bytes);
                    texture_cache[idx] = Some(Rc::new(texture));
                }
    
                if let Some(texture_ref) = &texture_cache[idx] {
                    return texture_ref.clone()
                }
            }

            panic!("Texture wasn't cached for some reason")
        }
    }
}