const PI:f32 = 3.142;

pub enum Type {
    Linear,
    Circle,
}

pub struct Shape {
    width: f32,
    height: f32,
    shape_type: Type,
}

pub fn new_shape(width: f32, height: f32, shape_type: Type) -> Shape {
    Shape {
        width: width,
        height: height,
        shape_type: shape_type,
    }
}

impl Shape {
    pub fn get_area(&self) -> f32 {
        if let  Type::Circle = self.shape_type {
            PI * self.width * self.width
        } else {
            self.width * self.height
        }
    }
}
