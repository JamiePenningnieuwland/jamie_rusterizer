extern crate gltf;
use crate::texture::Texture;
use crate::geometry::*;
use std::path::Path;

use glam::{Vec2,Vec3, Vec4, Quat};

pub struct Model{

    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,

}
impl Model{

    pub fn load(file_path: &Path) -> Self{

        let resource: Model;
        let (document, buffers, images) = gltf::import(file_path.clone()).expect("Failed to get model.");

        let mut meshes = Vec::new();
        let mut _materials = vec![Material::default(); document.materials().len()];
        if _materials.len() == 0 {
            _materials.push(Material::default());
        }
        
        if document.nodes().len() > 0 {
            Self::process_node(document.nodes().next().as_ref().unwrap(), &buffers, &images, &file_path, &mut meshes, &mut _materials);
        }

        resource = Model{
            meshes: meshes,
            materials: _materials
        };
        resource
    }
  
//this function is mostly taken from https://github.com/JasondeWolff/rusterizer
fn process_node(node: &gltf::Node, buffers: &Vec<gltf::buffer::Data>, _images: &Vec<gltf::image::Data>, base_path: &Path, meshes: &mut Vec<Mesh>, materials: &mut Vec<Material>) {
    let (translation, rotation, scale) = node.transform().decomposed();
    let _translation = Vec3::new(translation[0], translation[1], translation[2]);
    let _rotation = Quat::from_xyzw(rotation[3], rotation[0], rotation[1], rotation[2]); // Correct order?!?!?!?
    let _scale = Vec3::new(scale[0], scale[1], scale[2]);

    match node.mesh() {
        Some(mesh) => {
            for primitive in mesh.primitives() {
                if primitive.mode() == gltf::mesh::Mode::Triangles {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    let positions = {
                        let iter = reader
                            .read_positions()
                            .expect("Failed to process mesh node. (Vertices must have positions)");

                        iter.map(|arr| -> Vec3 { Vec3::from(arr) }).collect::<Vec<_>>()
                    };

                    let mut vertices: Vec<Vertex> = positions
                        .into_iter()
                        .map(|position| {
                            Vertex {
                                position: Vec4::from((position, 1.0)),
                                ..Vertex::default()
                            }
                    }).collect();

                    let indices = reader
                        .read_indices()
                        .map(|read_indices| {
                            read_indices.into_u32().collect::<Vec<_>>()
                        }).expect("Failed to process mesh node. (Indices are required)");

                    if let Some(normals) = reader.read_normals() {
                        for (i, normal) in normals.enumerate() {
                            vertices[i].normal = Vec3::from(normal);
                        }
                    }

                    let mut tex_coord_channel = 0;
                    while let Some(tex_coords) = reader.read_tex_coords(tex_coord_channel) {
                        for (i, tex_coord) in tex_coords.into_f32().enumerate() {
                            match tex_coord_channel {
                                0 => vertices[i].texcoords = Vec2::new(tex_coord[0].fract(), tex_coord[1].fract()),
                                _ => {}
                            }
                        }

                        tex_coord_channel += 1;
                    }

                    // if let Some(tangents) = reader.read_tangents() {
                    //     for (i, tangent) in tangents.enumerate() {
                    //         vertices[i].tangent = Vec4::from(tangent);
                    //     }
                    // } else {
                    //     // Source: 2001. http://www.terathon.com/code/tangent.html
                    //     let mut tan1 = vec![Vec3::default(); vertices.len()];
                    //     let mut tan2 = vec![Vec3::default(); vertices.len()];

                    //     for i in (0..indices.len()).step_by(3) {
                    //         let i1 = indices[i + 0] as usize;
                    //         let i2 = indices[i + 1] as usize;
                    //         let i3 = indices[i + 2] as usize;
                        
                    //         let v1 = vertices[i1].position;
                    //         let v2 = vertices[i2].position;
                    //         let v3 = vertices[i3].position;
                        
                    //         let w1 = vertices[i1].texcoords;
                    //         let w2 = vertices[i2].texcoords;
                    //         let w3 = vertices[i3].texcoords;
                        
                    //         let x1 = v2.x - v1.x;
                    //         let x2 = v3.x - v1.x;
                    //         let y1 = v2.y - v1.y;
                    //         let y2 = v3.y - v1.y;
                    //         let z1 = v2.z - v1.z;
                    //         let z2 = v3.z - v1.z;

                    //         let s1 = w2.x - w1.x;
                    //         let s2 = w3.x - w1.x;
                    //         let t1 = w2.y - w1.y;
                    //         let t2 = w3.y - w1.y;

                    //         let r = 1.0 / (s1 * t2 - s2 * t1);

                    //         let sdir = Vec3::new(
                    //             (t2 * x1 - t1 * x2) * r,
                    //             (t2 * y1 - t1 * y2) * r,
                    //             (t2 * z1 - t1 * z2) * r
                    //         );

                    //         let tdir = Vec3::new(
                    //             (s1 * x2 - s2 * x1) * r,
                    //             (s1 * y2 - s2 * y1) * r,
                    //             (s1 * z2 - s2 * z1) * r
                    //         );
                        
                    //         tan1[i1] += sdir;
                    //         tan1[i2] += sdir;
                    //         tan1[i3] += sdir;
                        
                    //         tan2[i1] += tdir;
                    //         tan2[i2] += tdir;
                    //         tan2[i3] += tdir;
                    //     }
                    
                    //     for i in 0..vertices.len() {
                    //         let n = vertices[i].normal;
                    //         let t = tan1[i];
                        
                    //         let xyz = (t - (n * n.dot(t))).normalize();
                        
                    //         let w;
                    //         if n.cross(t).dot(tan2[i]) < 0.0 {
                    //             w = -1.0;
                    //         } else {
                    //             w = 1.0;
                    //         }

                    //         vertices[i].tangent = Vec4::new(xyz.x, xyz.y, xyz.z, w);
                    //     }
                    // }

                    // if let Some(colors) = reader.read_colors(0) {
                    //     let colors = colors.into_rgba_f32();
                    //     for (i, color) in colors.enumerate() {
                    //         vertices[i].color = Vec4::from(color);
                    //     }
                    // }
                    
                    let prim_material = primitive.material();
                    let pbr = prim_material.pbr_metallic_roughness();
                    let material_idx = primitive.material().index().unwrap_or(0);

                    let material = &mut materials[material_idx];
                    if material.index == None {
                        material.index = Some(material_idx);
                        material.name = prim_material.name().map(|s| s.into()).unwrap_or(String::from("Unnamed"));
                        material.base_color_factor = Vec4::from(pbr.base_color_factor());
                        material.metallic_factor = pbr.metallic_factor();
                        material.roughness_factor = pbr.roughness_factor();
                        material.emissive_factor = Vec3::from(prim_material.emissive_factor());

                        if let Some(color_tex) = pbr.base_color_texture() {
                            material.base_color_texture = Texture::load_texture_from_gltf(&color_tex.texture(), base_path);
                        }

                        if let Some(normal_tex) = prim_material.normal_texture() {
                            material.normal_texture = Texture::load_texture_from_gltf(&normal_tex.texture(),base_path);
                            material.normal_scale = normal_tex.scale();
                        }

                        if let Some(mr_tex) = pbr.metallic_roughness_texture() {
                            material.metallic_roughness_texture = Texture::load_texture_from_gltf(&mr_tex.texture(),base_path);
                        }

                        if let Some(occlusion_tex) = prim_material.occlusion_texture() {
                            material.occlusion_texture = Texture::load_texture_from_gltf(&occlusion_tex.texture(),base_path);
                            material.occlusion_strength = occlusion_tex.strength();
                        }

                        if let Some(emissive_tex) = prim_material.emissive_texture() {
                            material.emissive_texture = Texture::load_texture_from_gltf(&emissive_tex.texture(),base_path);
                        }
                    }

                    meshes.push(Mesh {
                        vertices: vertices,
                        indices: indices,
                        material_index: material_idx
                    });
                } else {
                    panic!("Failed to process mesh node. (Trying to parse a non-triangle)");
                }
            }
        },
        None => {}
    };
}
}