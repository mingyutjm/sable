use crate::camera::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ConstantBuffer {
    worldViewProj: [[f32; 4]; 4],
}

impl ConstantBuffer {
    pub fn new() -> Self {
        Self {
            worldViewProj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        // self.worldViewProj = camera.build_view_projection_matrix().to_cols_array_2d();
        self.worldViewProj = camera.build_view_projection_matrix().transpose().to_cols_array_2d();
    }
}