mod vector2;


fn main() {
	let mut v = vector2::new(0.0, 1.0);
	let v2 = vector2::new(0.0, 3.0);
	
	v.print();
	v.normalize();
	v.print();
	v.print();
	println!("{}", v.dot(v2));
}
