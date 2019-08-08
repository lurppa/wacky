pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn default(point: Point) -> f32 {
    (point.x + point.y) / 2.0
}

pub fn radial(point: Point) -> f32 {
    let center = Point {x: 0.5, y: 0.5};
    let distance = {{point.x - center.x}.powi(2) + {point.y - center.y}.powi(2)}.sqrt();
    distance
}
