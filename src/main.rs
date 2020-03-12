#![allow(non_snake_case)]

pub struct RD {
	pub activator_ : Vec<f32>,
	pub substrate_ : Vec<f32>,
}

fn get_index(x: u32, y: u32, width: u32) -> usize {
	return (y * width + x) as usize;
}

fn laplacian(concentrations: &Vec<f32>, x: u32, y: u32, width: u32, height: u32) -> f32 {
	// repeating BCs
	let left  = if x == 0          { width - 1 }  else { x-1 };
	let right = if x >= width - 1  { 0 }          else { x+1 };	
	let up    = if y == 0          { height - 1 } else { y-1 };
	let down  = if y >= height - 1 { 0 }          else { y+1 };

	// calculate laplacian
	return -4.0 * concentrations[get_index(x,     y,    width)] + 
				  concentrations[get_index(left,  y,    width)] +
				  concentrations[get_index(x,     up,   width)] + 
				  concentrations[get_index(x,     down, width)] +
				  concentrations[get_index(right, y,    width)];
}

fn concentration_to_color(concentration : f32) -> char {
	let color_map = " .:;=+xX%$";

	let mut concentration = 9.0 * (concentration / 0.4);
	concentration = concentration.round();
	concentration = if concentration < 0.0 { 0.0 } else if concentration > 9.0 { 9.0 } else { concentration };

	let mut i = concentration as usize;
	if i >= color_map.chars().count() {
		i = color_map.chars().count() - 1;
	}

	return color_map.chars().nth(i).unwrap();
}

fn main() {

	// Domain size
	let width  : u32 = 100;
	let height : u32 = 100;

	// Reaction-diffusion parameters for the Gray-Scott model
	let F 	   : f32 = 0.042;
	let k      : f32 = 0.063;
	let dt     : f32 = 0.2;
	let Da     : f32 = 0.25;
	let Ds     : f32 = 0.5;
	let iters  : u32 = 100000;

	// Initialize the domain
	let mut reaction_diffusion0 = RD{
		activator_: vec![0.0; (width*height) as usize],
		substrate_: vec![1.0; (width*height) as usize],
	};

	let mut reaction_diffusion1 = RD{
		activator_: vec![0.0; (width*height) as usize],
		substrate_: vec![1.0; (width*height) as usize],
	};

	// Add a 10x10 square of activator
	for x in 0..10 {
		for y in 0..10 {
			let index 	      = get_index(x, y, width);
			reaction_diffusion0.activator_[index] = 0.5;
			reaction_diffusion1.activator_[index] = 0.5;
		}
	}

	// Run the simulation
	let mut mode = false;
	for _i in 0..iters
	{
		if mode 
		{ 
			let read_from = &mut reaction_diffusion1;
			let write_to  = &mut reaction_diffusion0;	
			
			for x in 0..width {
				for y in 0..height {
					let index = get_index(x, y, width);
					let a     = read_from.activator_[index];
					let s     = read_from.substrate_[index];
					let lap_a = laplacian(&read_from.activator_, x, y, width, height);
					let lap_s = laplacian(&read_from.substrate_, x, y, width, height);

					write_to.activator_[index] = (Da * lap_a + s*a*a - (F+k)*a)   * dt + a;
					write_to.substrate_[index] = (Ds * lap_s - s*a*a + F*(1.0-s)) * dt + s;
				}
			}
		} else {
			let read_from = &mut reaction_diffusion0;
			let write_to  = &mut reaction_diffusion1;
			
			for x in 0..width {
				for y in 0..height {
					let index = get_index(x, y, width);
					let a     = read_from.activator_[index];
					let s     = read_from.substrate_[index];
					let lap_a = laplacian(&read_from.activator_, x, y, width, height);
					let lap_s = laplacian(&read_from.substrate_, x, y, width, height);

					write_to.activator_[index] = (Da * lap_a + s*a*a - (F+k)*a)   * dt + a;
					write_to.substrate_[index] = (Ds * lap_s - s*a*a + F*(1.0-s)) * dt + s;
				}
			}
		}
		mode = !mode;
	}

	// Display the result
	// Draw border
	for _i in 0..width {
		print!("_");
	}
	print!("\n");

	// Draw the pattern
	for x in 0..height {
		for y in 0..width {
			let index 	      = get_index(x, y, width);
			let concentration = reaction_diffusion0.activator_[index];
			print!("{}", concentration_to_color(concentration));
		}
		print!("\n");
	}

	// Draw border
	for _i in 0..width {
		print!("_");
	}
	print!("\n");
}


// #![allow(non_snake_case)]

// pub struct RD {
// 	pub activator_ : Vec<f32>,
// 	pub substrate_ : Vec<f32>,
// }

// fn get_index(x: u32, y: u32, width: u32) -> usize {
// 	return (y * width + x) as usize;
// }

// fn laplacian(concentrations: &Vec<f32>, x: u32, y: u32, width: u32, height: u32) -> f32 {
// 	// repeating BCs
// 	let left  = if x == 0          { width - 1 }  else { x-1 };
// 	let right = if x >= width - 1  { 0 }          else { x+1 };	
// 	let up    = if y == 0          { height - 1 } else { y-1 };
// 	let down  = if y >= height - 1 { 0 }          else { y+1 };

// 	// calculate laplacian
// 	return -4.0 * concentrations[get_index(x,     y,    width)] + 
// 				  concentrations[get_index(left,  y,    width)] +
// 				  concentrations[get_index(x,     up,   width)] + 
// 				  concentrations[get_index(x,     down, width)] +
// 				  concentrations[get_index(right, y,    width)];
// }

// fn concentration_to_color(concentration : f32) -> char {
// 	let color_map = " .:;=+xX%$";

// 	let mut concentration = 9.0 * (concentration / 0.4);
// 	concentration = concentration.round();
// 	concentration = if concentration < 0.0 { 0.0 } else if concentration > 9.0 { 9.0 } else { concentration };

// 	let mut i = concentration as usize;
// 	if i >= color_map.chars().count() {
// 		i = color_map.chars().count() - 1;
// 	}

// 	return color_map.chars().nth(i).unwrap();
// }

// fn main() {

// 	// Domain size
// 	let width  : u32 = 100;
// 	let height : u32 = 100;

// 	// Reaction-diffusion parameters for the Gray-Scott model
// 	let F 	   : f32 = 0.042;
// 	let k      : f32 = 0.063;
// 	let dt     : f32 = 0.2;
// 	let Da     : f32 = 0.25;
// 	let Ds     : f32 = 0.5;
// 	let iters  : u32 = 100000;

// 	// Initialize the domain
// 	let mut reaction_diffusion = vec![
// 		RD{
// 			activator_: vec![0.0; (width*height) as usize],
// 			substrate_: vec![1.0; (width*height) as usize],
// 		},
// 		RD{
// 			activator_: vec![0.0; (width*height) as usize],
// 			substrate_: vec![1.0; (width*height) as usize],
// 		},
// 	];

// 	// Add a 10x10 square of activator
// 	for x in 0..10 {
// 		for y in 0..10 {
// 			let index 	      = get_index(x, y, width);
// 			reaction_diffusion[0].activator_[index] = 0.5;
// 			reaction_diffusion[1].activator_[index] = 0.5;
// 		}
// 	}

// 	// Run the simulation
// 	let mut mode = 0;
// 	for _i in 0..iters
// 	{
// 		let read_from = &mut reaction_diffusion[mode];
// 		mode = (mode + 1) % 2;
// 		let write_to  = &mut reaction_diffusion[mode];
		
// 		for x in 0..width {
// 			for y in 0..height {
// 				let index = get_index(x, y, width);
// 				let a     = read_from.activator_[index];
// 				let s     = read_from.substrate_[index];
// 				let lap_a = laplacian(&read_from.activator_, x, y, width, height);
// 				let lap_s = laplacian(&read_from.substrate_, x, y, width, height);

// 				write_to.activator_[index] = (Da * lap_a + s*a*a - (F+k)*a)   * dt + a;
// 				write_to.substrate_[index] = (Ds * lap_s - s*a*a + F*(1.0-s)) * dt + s;
// 			}
// 		}
		
		
// 	}

// 	// Display the result
// 	// Draw border
// 	for _i in 0..width {
// 		print!("_");
// 	}
// 	print!("\n");

// 	// Draw the pattern
// 	for x in 0..height {
// 		for y in 0..width {
// 			let index 	      = get_index(x, y, width);
// 			let concentration = reaction_diffusion0.activator_[index];
// 			print!("{}", concentration_to_color(concentration));
// 		}
// 		print!("\n");
// 	}

// 	// Draw border
// 	for _i in 0..width {
// 		print!("_");
// 	}
// 	print!("\n");
// }
