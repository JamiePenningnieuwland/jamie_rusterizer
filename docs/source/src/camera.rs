use crate::transform::Transform;
use glam::Mat4;

pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub fov: f32, // in radians
    pub aspect_ratio: f32,
    pub transform: Transform,
    pub speed: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            near: 0.1,
            far: 100.0,
            fov: std::f32::consts::PI /4.0,
            aspect_ratio: 1.0,
            transform: Transform::IDENTITY,
            speed: 1.0,
        }
    }
}
impl Camera {
    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(
            self.fov,
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    // pub fn view(&self) -> Mat4 {
    //     Mat4::look_at_rh(
    //         self.transform.translation,
    //         self.transform.translation + self.transform.forward(),
    //         self.transform.up(),
    //     )
    // }
pub fn view(&self) -> Mat4{
   let rotation_matrix =  Mat4::from_quat(self.transform.rotation);
   let translation_matrix = Mat4::from_translation(self.transform.translation);
   let view =  translation_matrix * rotation_matrix;
    view
}


}