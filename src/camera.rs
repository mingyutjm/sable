use glam;

pub(crate) struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_lh(self.eye, self.target, self.up);
        let proj = glam::Mat4::perspective_lh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
        proj * view
    }
}