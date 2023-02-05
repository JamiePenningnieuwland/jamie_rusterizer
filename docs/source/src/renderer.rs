
use glam::{Vec2,Vec3, Vec4Swizzles, Vec4, Mat4};

use crate::utils::*;

use crate::geometry::*;
use crate::model::Model;
use crate::camera::Camera;
use crate::transform::Transform;

pub struct Renderer {
    buffer: Vec<u32>,// = vec![0; WIDTH * HEIGHT];
    z_buffer: Vec<f32>, //vec![f32::INFINITY; WIDTH * HEIGHT];
    viewport_size: (u32,u32),
    pub camera: Camera,
    ndc_to_screen_m: Mat4
}

impl Renderer {

    pub fn new(viewport_w: usize, viewport_h: usize)-> Self{

        let half_width = viewport_w as f32 / 2.0;
        let half_height = viewport_h as f32 / 2.0;
        let aspect_ratio = viewport_w as f32/ viewport_h as f32;
        Self{

            viewport_size: (viewport_w as u32,viewport_h as u32),
            buffer: vec![0; viewport_w * viewport_h],
            z_buffer:  vec![f32::INFINITY; viewport_w * viewport_h],
            camera: Camera{
                aspect_ratio,
                transform: Transform::from_translation(glam::vec3(0.0, 0.0, 0.0)),
                far: 100.0,
                ..Default::default()},
                
            ndc_to_screen_m: glam::mat4(
                glam::vec4(-half_width, 0., 0., 0.),
                glam::vec4(0., -half_height, 0., 0.),
                glam::vec4(0., 0., 1., 0.),
                glam::vec4(half_width, half_height, 0., 1.),
            )
        }
    }
   
    pub fn buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
    
    pub fn clear(&mut self){

        let max_x: i32 = self.viewport_size.0 as i32;
        let min_x: i32 = 0;
    
        let max_y: i32 = self.viewport_size.1 as i32;
        let min_y: i32 = 0;

        for y in min_y..max_y {
            for x in min_x..max_x {

            let i = (x + y * self.viewport_size.0 as i32) as usize;

            self.buffer[i] = to_argb8(255,234,182,118);
            self.z_buffer[i] = f32::INFINITY;
        }
        }
    }
  
    pub fn raster_mesh(&mut self, mesh: &Mesh, material: &Material, model_matrix: Transform){
    
        let _mvp =  self.camera.projection() * self.camera.view().inverse()* model_matrix.local();
        let triangle_count = mesh.indices.len() / 3;
        for i in 0..triangle_count{
            let v1 = mesh.vertices[mesh.indices[(i * 3 + 0) as usize] as usize];
            let v0 = mesh.vertices[mesh.indices[(i * 3 + 1) as usize] as usize];
            let v2 = mesh.vertices[mesh.indices[(i * 3 + 2) as usize] as usize];
            
            let mut vertices: [Vertex; 3];
            let mut temp_vertices: Vec<Vertex> = Vec::default();
            let mut clipped_vertices: Vec<Vertex>= Vec::default();
            vertices = [v0,v1,v2];

            for vertex in vertices.iter_mut(){
           
                vertex.position =  _mvp * vertex.position;
                vertex.w = vertex.position.w;
                clipped_vertices.push(*vertex);

            }
            if Self::clip_plane(&mut clipped_vertices, &mut temp_vertices, 0) &&
            Self::clip_plane(&mut clipped_vertices, &mut temp_vertices, 1) &&
            Self::clip_plane(&mut clipped_vertices, &mut temp_vertices, 2){

         
               for vertex in clipped_vertices.iter_mut(){
                   //vertex.test = vertex.position;
                   vertex.w = vertex.position.w;
                   vertex.position = vertex.position / vertex.position.w;
                   vertex.position = self.ndc_to_screen_m * vertex.position;
                   vertex.texcoords = vertex.texcoords;
                   let normal = model_matrix.local() * Vec4::from((vertex.normal, 0.0));
                   vertex.normal = normal.xyz();
                  
                   
                }
                let initial_vertex = clipped_vertices[0];
                for j in 1..clipped_vertices.len() - 1
                {
                    
                    self.fill_triangle(& [initial_vertex, clipped_vertices[j], clipped_vertices[j + 1]],  material);
                }
            }
            

        }
        
    }
    pub fn raster_model(&mut self, model: &Model, model_matrix: Transform){

        for mesh in model.meshes.iter() {

            self.raster_mesh(&mesh,&model.materials[mesh.material_index], model_matrix);
        }
    }
    
    fn fill_triangle( &mut self, vertices: &[Vertex; 3], material: &Material) {
        let mut min_x: i32 = self.viewport_size.0 as i32;
        let mut max_x: i32 = 0;
    
        let mut min_y: i32 = self.viewport_size.1 as i32;
        let mut max_y: i32 = 0;
    
        for vertex in vertices.iter() {
            if min_x > vertex.position.x as i32 {
                min_x = vertex.position.x as i32;
            }
            if max_x < vertex.position.x as i32 {
                max_x = vertex.position.x as i32;
            }
            if min_y > vertex.position.y as i32 {
                min_y = vertex.position.y as i32;
            }
        
            if max_y < vertex.position.y as i32 {
                max_y = vertex.position.y as i32;
            }
        }
       
        if min_x < 0 {
            min_x = 0;
        }
        if min_y < 0 {
            min_y = 0;
        }
        if max_x >= self.viewport_size.0 as i32 {
            max_x = self.viewport_size.0 as i32 - 1;
        }
        if max_y >= self.viewport_size.1 as i32 {
            max_y = self.viewport_size.1 as i32 - 1;
        }
    
        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                
                let p = Vec2::new(x as f32, y as f32);
                let index_0 = 0;
                let index_1 = 1;
                let index_2 = 2;
                let m2 = check_edge(p, vertices[index_0].position.xy(), vertices[index_1].position.xy());
    
                let m0 = check_edge(p, vertices[index_1].position.xy(), vertices[index_2].position.xy());
    
                let m1 = check_edge(p, vertices[index_2].position.xy(), vertices[index_0].position.xy());
                if m2 >= 0.0 && m0 >= 0.0 && m1 >= 0.0 {
    
                    let total_area = check_edge(vertices[index_2].position.xy(), vertices[index_0].position.xy(), vertices[index_1].position.xy());
                    let mut bc = Vec3::new(m0 / total_area, m1 / total_area, m2 / total_area);
                    let one_over_w = 1.0 / Vec3 { x: vertices[index_0].w, y: vertices[index_1].w, z: vertices[index_2].w };
                    bc = (bc * one_over_w) / (bc.dot(one_over_w));

                    let depth = bc.x * vertices[index_0].position.z + bc.y * vertices[index_1].position.z + bc.z * vertices[index_2].position.z;
    
                    
                    let i = (x + y * self.viewport_size.0 as i32) as usize;
                    // let uv = v0.uv * bc.x + v1.uv * bc.y + v2.uv * bc.z;
            

                    if depth < self.z_buffer[i] {
                        self.z_buffer[i] = depth;
    
                        let tex_coords = bc.x * vertices[index_0].texcoords + bc.y * vertices[index_1].texcoords + bc.z * vertices[index_2].texcoords;
                        //tex_coords *= 2.0 - 0.5;
                        let color = material.base_color_texture.argb_at_uv(tex_coords.x, tex_coords.y);
                        let normal =  bc.x * vertices[index_0].normal + bc.y * vertices[index_1].normal + bc.z * vertices[index_2].normal;
                        
                        //fragment shader
                        let ambient_strength = 0.1;
                        let ambient = ambient_strength * u32_to_vec3(color);

                        let norm = normal.normalize();
                        let light_dir = Vec3::new(0.0, 0.8,0.8).normalize();
                        let diff = norm.dot(light_dir).max(0.0);
                        let diffuse = diff * Vec3::new(1.0, 1.0, 1.0);
                        let result = (ambient + diffuse) * u32_to_vec3(color);


                        self.buffer[i] = vec3_to_argb8(bc);
                }
            }
            }
        }
    }
fn clip_plane(vertices: &mut Vec<Vertex>, result_vertices: &mut Vec<Vertex>, axis: u32) -> bool
{
	Self::clip_component(vertices, axis, 1.0, result_vertices);
	vertices.clear();

	if result_vertices.is_empty(){

        return false;
    }

	Self::clip_component(result_vertices, axis, -1.0, vertices);
	result_vertices.clear();
	
	!vertices.is_empty()
}
fn clip_component(vertices: &mut Vec<Vertex>, plane: u32, component_factor: f32, result_vertices: &mut Vec<Vertex>){
    
    let mut previous_vertex = vertices[vertices.len() - 1];
    let mut previous_w = previous_vertex.position.w;
	let mut previous_component =  previous_vertex.position[plane as usize] * component_factor;
	let mut previous_inside = previous_component <= previous_w;
    
	for i in 0..vertices.len()
	{
        let current_vertex = vertices[i];
        let current_w = current_vertex.position.w;
		let current_component = current_vertex.position[plane as usize] * component_factor;
		let current_inside = current_component <= current_w;

		//only one is true
		if current_inside ^ previous_inside{
			let lerp_amt = (previous_w - previous_component) /
				((previous_w - previous_component) -
					(current_w - current_component));
 
			
            let mut output_vertex: Vertex = Vertex::default();
			output_vertex.position = lerp(previous_vertex.position, current_vertex.position, lerp_amt);
            output_vertex.w = output_vertex.position.w;
			//outPutVertex.NDC = MathUtil::Lerp(outPutVertex.NDC, currentVertex.NDC, { lerpAmt, lerpAmt, lerpAmt,lerpAmt });
			output_vertex.texcoords = lerp(previous_vertex.texcoords, current_vertex.texcoords, lerp_amt);
			output_vertex.normal = lerp(previous_vertex.normal, current_vertex.normal, lerp_amt);
			
			result_vertices.push(output_vertex);
		}
		if current_inside
		{
			result_vertices.push(current_vertex);
		}
		previous_vertex = current_vertex;
		previous_component = current_component;
		previous_inside = current_inside;
        previous_w = current_w;
	}
}

}