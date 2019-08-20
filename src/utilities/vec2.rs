use nalgebra_glm as glm;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn into_glm_tmat4(self, z: f32) -> glm::TMat4<f32> {
        glm::translate(&glm::identity(), &self.into_glm_vec3(z))
    }

    pub fn into_glm_vec3(self, z: f32) -> glm::TVec3<f32> {
        glm::make_vec3(&[self.x, self.y, z])
    }
}
