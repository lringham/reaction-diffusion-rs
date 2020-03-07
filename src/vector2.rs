use std::ops;


// trait Vector {
// 	fn len(&self) -> f32;
// 	fn normalize(&mut self);
// 	fn zero(&mut self);
// 	fn print();
// }

pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

pub fn new(x: f32, y: f32) -> Vector2 {
    return Vector2{x, y};
}

// ================ implementations

impl Vector2 {
	pub fn len(&self) -> f32 {
		return (self.x * self.x + self.y * self.y).sqrt();
	}

	pub fn normalize(&mut self) {
		let l = self.len();
		self.x /= l;
		self.y /= l;
	}

	pub fn zero(&mut self) {
		self.x = 0.0;
		self.y = 0.0;
    }
	
	pub fn dot(&self, rhs: Vector2) -> f32 {
		return self.x * rhs.x + self.y * rhs.y;
	}

    pub fn print(&self) {
        println!("({}, {})", self.x, self.y);
	}
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, _rhs: Vector2) -> Vector2 {
		return new(self.x + _rhs.x, self.y + _rhs.y);
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, _rhs: Vector2) -> Vector2 {
		return new(self.x - _rhs.x, self.y - _rhs.y);
    }
}

impl ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, _rhs: Vector2) -> Vector2 {
		return new(self.x * _rhs.x, self.y * _rhs.y);
    }
}