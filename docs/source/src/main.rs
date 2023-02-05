extern crate glam;
extern crate minifb;

use std::path;

use path::Path;
use glam::{Vec3,Vec2,Vec4, Quat, EulerRot, Mat4};
use minifb::{Key, Window, WindowOptions};

pub use renderer::Renderer;
pub mod renderer;

use transform::Transform;


const WIDTH: usize = 600;
const HEIGHT: usize = 600;

pub mod geometry;
pub use geometry::*;

pub mod texture;
pub use texture::Texture;

pub mod model;
pub use model::Model;

pub mod utils;
pub use utils::*;

pub mod transform;
pub mod camera;

mod timer;
use timer::Timer;

fn create_quad() -> Mesh{

    let v0 = Vertex{position: Vec4::new(-1.0, -1.0, 0.0,1.0),w : 1.0, texcoords: Vec2::new(0.0,0.0), normal: Vec3::new(0.0,0.0,0.0)};
    let v1 = Vertex{position: Vec4::new(-1.0, 1.0, 0.0,1.0),w : 1.0,texcoords: Vec2::new(0.0,1.0), normal: Vec3::new(0.0,0.0,0.0)};
    let v2 = Vertex{position: Vec4::new(1.0, 1.0,0.0,1.0),w : 1.0,texcoords: Vec2::new(1.0,1.0), normal: Vec3::new(0.0,0.0,0.0)};
  
  
          
    let v3 =  Vertex{position: Vec4::new(1.0, -1.0, 0.0,1.0),w : 1.0,texcoords: Vec2::new(1.0,0.0), normal: Vec3::new(0.0,0.0,0.0)};
    let v4 =  Vertex{position: Vec4::new(-1.0, -1.0, 0.0,1.0),w : 1.0,texcoords: Vec2::new(0.0,0.0), normal: Vec3::new(0.0,0.0,0.0)};
    let v5 =  Vertex{position: Vec4::new(1.0, 1.0, 0.0,1.0),w : 1.0,texcoords: Vec2::new(1.0,1.0), normal: Vec3::new(0.0,0.0,0.0)};
 
    let mesh = Mesh{
        vertices: vec![v0,v1,v2, v3, v4,v5],
        indices: vec![0,1,2,3,4,5],
        material_index: 0
    };
    mesh
}
fn update_camera(delta_time: f32, the_renderer: &mut Renderer, window: &Window){
   let mut rotation = Vec3::default();
    let sens = 0.8;
    let movement_speed = 2.0;
    
    if window.is_key_down(Key::Up)
    {
        rotation.x += 1.0;
    
    }
    if window.is_key_down(Key::Down)
    {
        rotation.x -= 1.0;
    
    }
    if window.is_key_down(Key::Left)
    {
        rotation.y -= 1.0;
    
    }
    if window.is_key_down(Key::Right)
    {
        rotation.y += 1.0;
        
    }
    
    rotation *= delta_time * sens;
    let cam_rotation:(f32,f32,f32);
    cam_rotation = the_renderer.camera.transform.rotation.to_euler(EulerRot::YXZ);
    rotation = Vec3::new(rotation.y + cam_rotation.0, rotation.x + cam_rotation.1, cam_rotation.2);
    the_renderer.camera.transform.rotation = Quat::from_euler(EulerRot::YXZ, rotation.x, rotation.y, rotation.z).normalize();

    let mut movement = Vec3::default();
    if window.is_key_down(Key::D)
    {
        movement.x -= 1.0;
    
    }
    if window.is_key_down(Key::A)
    {
        movement.x += 1.0;
    
    }
    if window.is_key_down(Key::Space)
    {
        movement.y += 1.0;
        
    }
    if window.is_key_down(Key::LeftShift)
    {
        movement.y -= 1.0;

    }
    if window.is_key_down(Key::S)
    {
        movement.z += 1.0;
    
    }
    if window.is_key_down(Key::W)
    {
        movement.z -= 1.0;

    }
    if movement.length_squared() > 0.1
    {
        movement = movement.normalize();
        let view = Mat4::from_quat(the_renderer.camera.transform.rotation.normalize());
        
        let temp = view.mul_vec4(Vec4::new(movement.x, movement.y, movement.z, 0.0));
        movement = Vec3::new(temp.x, temp.y, temp.z);
        the_renderer.camera.transform.translation += movement * delta_time * movement_speed;

    }
  
}
fn main() {

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut the_renderer = Renderer::new(WIDTH, HEIGHT);
    let mut mat = Material::default();
    mat.base_color_texture = Texture::load(Path::new("assets/bojan.jpg"));
    let model0 = Model::load(Path::new("assets/DamagedHelmet/glTF/DamagedHelmet.gltf"));
    //let mesh0 = create_quad();
    //let pos = Vec3::new(0.0,0.0,0.0);
    

    let mut delta_time: f32;
    let mut delta_timer = Timer::new();
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut r = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        the_renderer.clear();
        
        //deltaTime
        delta_time = delta_timer.elapsed() as f32;
        delta_timer.reset();
        r += 1.0 * delta_time;
        //update camera
        update_camera(delta_time,&mut the_renderer, &window);

        let rot = Quat::from_euler(EulerRot::XYZ, 1.5, 0.0, 0.0);
        //let rot = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
        let trans = Transform::from_translation_rotation(Vec3::new(0.0,0.0,-4.0), rot);
    
        //render mesh
        //the_renderer.raster_mesh(&mesh0, &mat, trans) ;
        the_renderer.raster_model(&model0,trans) ;
       

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(the_renderer.buffer(), WIDTH, HEIGHT).unwrap();

    }
}
