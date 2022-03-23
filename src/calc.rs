pub fn chamfer_shift_x(angle: f64, radius: f64) -> f64 {
    angle.to_radians().tan() * chamfer_shift_z(angle, radius)
}

pub fn chamfer_shift_z(angle: f64, radius: f64) -> f64 {
    radius - radius / (90_f64 - angle / 2_f64).to_radians().tan()
}