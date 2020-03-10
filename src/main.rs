mod vector2;

fn main() {
	let mut v = vector2::new(1.0, 0.0);
	let v2 = vector2::new(0.2, 3.0);

	let half: f32 = 2.0;
	v.print();
	for _x in 0..4 {
		v.rotate(std::f32::consts::PI / half);
		v.print();
	}

	v.normalize();
	v.print();
	println!("v.len = {}", v.len());

	v = v2 + v;
	v.print();
	v2.print();

	println!("{}", v2.dot(v));
}
