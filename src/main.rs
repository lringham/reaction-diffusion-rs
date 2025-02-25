#[derive(Debug, Clone)]
struct ActivatorSubstrate {
    activator: Vec<f32>,
    substrate: Vec<f32>,
}

struct GrayScottParameters {
    f: f32,
    k: f32,
    dt: f32,
    da: f32,
    ds: f32,
}

fn get_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn laplacian(concentrations: &[f32], x: usize, y: usize, width: usize, height: usize) -> f32 {
    // Repeating BCs
    let left = if x == 0 { width - 1 } else { x - 1 };
    let right = if x >= width - 1 { 0 } else { x + 1 };
    let up = if y == 0 { height - 1 } else { y - 1 };
    let down = if y >= height - 1 { 0 } else { y + 1 };

    // Calculate laplacian
    -4.0 * concentrations[get_index(x, y, width)]
        + concentrations[get_index(left, y, width)]
        + concentrations[get_index(x, up, width)]
        + concentrations[get_index(x, down, width)]
        + concentrations[get_index(right, y, width)]
}

fn concentration_to_color(concentration: f32) -> char {
    let color_map = b" .:;=+xX%$";
    let concentration = 9.0 * (concentration / 0.4);
    let byte = *color_map
        .get(concentration.round() as usize)
        .or_else(|| color_map.last())
        .unwrap();
    byte.into()
}

fn gray_scott(
    read_from: &ActivatorSubstrate,
    write_to: &mut ActivatorSubstrate,
    width: usize,
    height: usize,
    params: &GrayScottParameters,
) {
    for x in 0..width {
        for y in 0..height {
            let index = get_index(x, y, width);
            let a = read_from.activator[index];
            let s = read_from.substrate[index];
            let lap_a = laplacian(&read_from.activator, x, y, width, height);
            let lap_s = laplacian(&read_from.substrate, x, y, width, height);

            write_to.activator[index] =
                (params.da * lap_a + s * a * a - (params.f + params.k) * a) * params.dt + a;
            write_to.substrate[index] =
                (params.ds * lap_s - s * a * a + params.f * (1.0 - s)) * params.dt + s;
        }
    }
}

fn gray_scott_gauss_seidel(
    buff: &mut ActivatorSubstrate,
    width: usize,
    height: usize,
    params: &GrayScottParameters,
) {
    for x in 0..width {
        for y in 0..height {
            let index = get_index(x, y, width);
            let a = buff.activator[index];
            let s = buff.substrate[index];
            let lap_a = laplacian(&buff.activator, x, y, width, height);
            let lap_s = laplacian(&buff.substrate, x, y, width, height);

            buff.activator[index] =
                (params.da * lap_a + s * a * a - (params.f + params.k) * a) * params.dt + a;
            buff.substrate[index] =
                (params.ds * lap_s - s * a * a + params.f * (1.0 - s)) * params.dt + s;
        }
    }
}

fn draw_pattern(width: usize, height: usize, concentrations: &[f32]) {
    println!("{}", "_".repeat(width));
    for x in 0..height {
        for y in 0..width {
            let index = get_index(x, y, width);
            let concentration = concentrations[index];
            print!("{}", concentration_to_color(concentration));
        }
        println!();
    }
    println!("{}", "_".repeat(width));
}

fn main() {
    // Reaction-diffusion parameters for the Gray-Scott model
    let parameters = GrayScottParameters {
        f: 0.042,
        k: 0.063,
        dt: 0.4,
        da: 0.25,
        ds: 0.5,
    };

    // Initialize the domain with a square of activator
    let width: usize = 100;
    let height: usize = 100;
    let mut reaction_diffusion = ActivatorSubstrate {
        activator: vec![0.0; width * height],
        substrate: vec![1.0; width * height],
    };

    for x in 0..width / 10 {
        for y in 0..height / 10 {
            let index = get_index(x, y, width);
            reaction_diffusion.activator[index] = 0.5;
        }
    }

    // Run the simulation
    let iters: usize = 10000;
    let use_gauss_seidel = false;
    if use_gauss_seidel {
        for _ in 0..iters {
            gray_scott_gauss_seidel(&mut reaction_diffusion, width, height, &parameters);
        }
        draw_pattern(width, height, &reaction_diffusion.activator);
    } else {
        let mut reaction_diffusion0 = reaction_diffusion;
        let mut reaction_diffusion1 = reaction_diffusion0.clone();
        let mut mode = false;
        for _ in 0..iters {
            if mode {
                gray_scott(
                    &reaction_diffusion0,
                    &mut reaction_diffusion1,
                    width,
                    height,
                    &parameters,
                );
            } else {
                gray_scott(
                    &reaction_diffusion1,
                    &mut reaction_diffusion0,
                    width,
                    height,
                    &parameters,
                );
            }
            mode = !mode;
        }
        draw_pattern(width, height, &reaction_diffusion0.activator);
    }
}
