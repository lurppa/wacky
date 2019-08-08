pub struct Point {
    pub x: f32,
    pub y: f32,
}

struct NamedFilter {
    pub function: fn(Point) -> f32,
    pub name: String,
}

pub fn get_filter(name: &str) -> fn(Point) -> f32 {
    let named_filters = [
        NamedFilter {
            name: String::from("default"),
            function: default,
        },
        NamedFilter {
            name: String::from("radial"),
            function: radial,
        }
    ];

    for filter in named_filters.iter() {
        if filter.name == name {
            return filter.function;
        }
    };
    default
}

pub fn default(point: Point) -> f32 {
    (point.x + point.y) / 2.0
}

pub fn radial(point: Point) -> f32 {
    let center = Point {x: 0.5, y: 0.5};
    let distance = {{point.x - center.x}.powi(2) + {point.y - center.y}.powi(2)}.sqrt() / {2 as f32}.sqrt();
    distance
}
