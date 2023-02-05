
use stb_image;
use std::path::Path;
use crate::utils::*;
extern crate gltf;

#[derive(Clone)]
pub struct Texture
{
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub num_channel: usize

}
impl Texture
{

   pub fn load_texture_from_gltf(texture: &gltf::Texture, base_path: &Path) -> Texture {
        let img = texture.source();
        match img.source() {
            gltf::image::Source::Uri { uri, .. } => {
                let base_path = Path::new(base_path);
                let path = base_path.parent().unwrap_or_else(|| Path::new("./")).join(uri);
                Self::load(&path)
            }
            _ => panic!("Failed to process tex. (Only uri support)")
        }
        
    }
    pub fn load(path: &Path)->Self
    {
        let temp_data = stb_image::image::load(path);
        
        if let stb_image::image::LoadResult::ImageU8(image) = temp_data {
           
            let data = (0..image.data.len() / 3)
                .map(|id| {
                    to_argb8(
                        255,
                        image.data[id * 3],
                        image.data[id * 3 + 1],
                        image.data[id * 3 + 2],
                    )
                })
                .collect();
            Self {
                width: image.width,
                height: image.height,
                data,
                num_channel: image.depth,
            }
        }
        else {
            panic!("Unsupported texture type");
        }
    }
    pub fn argb_at_uv(&self, u: f32, v: f32) -> u32 {
        let (u, v) = (u * self.width as f32, v * self.height as f32);
        let id = coords_to_index(u as usize, v as usize, self.width);
        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb8(255, 255, 0, 255)
        }
    }
  
}
impl Default for Texture{

    fn default() -> Self {
        Self { width: 0, height: 0, data: Vec::default(), num_channel: 0 }
    }
}