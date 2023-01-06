use std::rc::Rc;

use xf::gl::texture::Texture;

use super::image::xf_texture_from_bytes;

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum TextureId {
    // Player, NPCs, Mobs
    Player,
}

fn get_bytes(id: TextureId) -> &'static [u8] {
    use TextureId::*;

    match id {
        Player => include_bytes!("../../assets/Sprites/Player.png"),
    }
}

impl TextureId {
    /// Loads the texture associated with this ID.
    pub fn texture(self) -> Rc<Texture> {
        let bytes = get_bytes(self);
        let texture = xf_texture_from_bytes(bytes);
        Rc::new(texture)
    }
}