use std::ops; // Used for operator overloading


#[derive(Copy, Clone)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

// Constructor ================
#[allow(dead_code)]
pub fn new(x: f32, y: f32) -> Vector2 {
	return Vector2 { x: x, y: y };
}

// Implementations ================
#[allow(dead_code)]
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

	pub fn rotate(&mut self, rad: f32) {
		let x = self.x;
		let y = self.y;
		self.x = rad.cos() * x - rad.sin() * y;
		self.y = rad.sin() * x + rad.cos() * y;
	}
}

// Operators ================
impl ops::Add<Vector2> for Vector2 {
	type Output = Vector2;
	fn add(self, rhs: Vector2) -> Vector2 {
		return Vector2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		};
	}
}

impl ops::Sub<Vector2> for Vector2 {
	type Output = Vector2;
	fn sub(self, rhs: Vector2) -> Vector2 {
		return Vector2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		};
	}
}

impl ops::Mul<Vector2> for Vector2 {
	type Output = Vector2;
	fn mul(self, rhs: Vector2) -> Vector2 {
		return Vector2 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		};
	}
}
