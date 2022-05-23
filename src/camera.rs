use crate::math::Vec3;

pub struct Camera {
    pub v_width: f32,
    pub v_height: f32,
    pub focal_length: f32,
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub ll_corner: Vec3
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        let aspect_ratio = width / height;
        let v_height = 2.0;
        let v_width = aspect_ratio * v_height;
        let focal_length = 1.0;
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(v_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, v_height, 0.0);
        let ll_corner = Vec3::new(
            origin.x - (0.5 * horizontal.x) - (0.5 * vertical.x),
            origin.y - (0.5 * horizontal.y) - (0.5 * vertical.y),
            origin.z - (0.5 * horizontal.z) - (0.5 * vertical.z) - focal_length
            );

        Self {
            v_width,
            v_height,
            focal_length,
            origin,
            horizontal,
            vertical,
            ll_corner
        }
    }

    pub fn direction(&self, u: f32, v: f32) -> Vec3 {
        Vec3::new(
            self.ll_corner.x + (u * self.horizontal.x) + (v * self.vertical.x) - self.origin.x,
            self.ll_corner.y + (u * self.horizontal.y) + (v * self.vertical.y) - self.origin.y,
            self.ll_corner.z + (u * self.horizontal.z) + (v * self.vertical.z) - self.origin.z
            )
    }
} 
