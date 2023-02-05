use glam::{Vec2, Vec3,Vec4};
use crate::Texture;

#[derive(Debug, Copy, Clone)]
pub struct Vertex
{
    pub position : Vec4,
    pub w : f32,
    pub texcoords : Vec2,
    pub normal : Vec3,
    //pub tangent : Vec4,
    //pub color : Vec4
}
impl Vertex{
    pub fn new() -> Self{
        Self {
             position: Vec4::new(0.0,0.0,0.0,0.0),
             w: 1.0, 
             texcoords: Vec2::new(0.0,0.0), 
             normal: Vec3::new(0.0,0.0,0.0), 
             //tangent:  Vec4::new(0.0,0.0,0.0,0.0), 
             //color : Vec4::new(0.0,0.0,0.0,0.0)
            }
    }
 
}
impl Default for Vertex {
    fn default() -> Self {
       Self::new()
  }
}
#[derive(Debug, Copy, Clone)]
pub struct Triangle {
   // pub vertices: Vec<Vertex>,
    pub indices : [i32; 3]
}
#[derive(Clone)]
pub struct Mesh{
     pub vertices: Vec<Vertex>,
     pub indices: Vec<u32>,
     pub material_index: usize
 }

impl Mesh
{
    pub fn get_vertices_from_triangle(&self, triangle: &Triangle) -> [&Vertex; 3] {
        [
            &self.vertices[triangle.indices[0] as usize],
            &self.vertices[triangle.indices[1] as usize],
            &self.vertices[triangle.indices[2] as usize],
        ]
    }


}

#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub index: Option<usize>,

    pub base_color_factor: Vec4,
    pub base_color_texture: Texture,

    pub normal_scale: f32,
    pub normal_texture: Texture,

    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Texture,

    pub occlusion_strength: f32,
    pub occlusion_texture: Texture,

    pub emissive_factor: Vec3,
    pub emissive_texture: Texture,
}
impl Default for Material {
    fn default() -> Self {
        Material {
            name: String::from("default"),
            index: None,
            base_color_factor: Vec4::new(1.0, 1.0, 1.0, 1.0),
            base_color_texture: Texture::default(),
            normal_scale: 1.0,
            normal_texture: Texture::default(),
            metallic_factor: 0.0,
            roughness_factor: 1.0,
            metallic_roughness_texture: Texture::default(),
            occlusion_strength: 1.0,
            occlusion_texture: Texture::default(),
            emissive_factor: Vec3::default(),
            emissive_texture:Texture::default(),
        }
    }
}