pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        let denom: f32 = self.length();
        Vec3 {x: self.x / denom, y: self.y / denom, z: self.z / denom}
    }
}

pub fn dot_prod(u: &Vec3, v: &Vec3) -> f32 {
    (u.x * v.x) + (u.y * v.y) + (u.z + v.z)
}

pub fn cross_prod(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: (u.y * v.z) - (v.y * u.z),
        y: (u.z * v.x) - (v.z * u.x),
        z: (u.x * v.y) - (v.x * u.y)
    }
}
