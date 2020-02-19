use std::fs;

const WIDTH: u32 = 25;
const HEIGHT: u32 = 6;
const LAYER_LENGTH: usize = WIDTH as usize * HEIGHT as usize;

fn main() {
    let filename = "input.txt";
    let images = fs::read_to_string(filename)
        .expect("Failed to read file!")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    println!("Find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
{}", check_image_data(images));
}

fn check_image_data(images: Vec<u32>) -> u32 {
    let mut image_layers: Vec<[u32; LAYER_LENGTH]> = Vec::new();
    let mut layer: [u32; LAYER_LENGTH];
    let mut pixel_counter = 0;
    let mut i;

    while pixel_counter < images.len() {
        layer = [0; LAYER_LENGTH];
        i = 0;
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                layer[i] = images[pixel_counter];
                pixel_counter += 1;
                i += 1;
            }
        }
        image_layers.push(layer);
    }
    let min_zero_index: usize = image_layers
        .iter()
        .enumerate()
        .map(|(i, j)| (j.iter().filter(|&z| *z == 0).count() as u32, i))
        .min()
        .unwrap()
        .1;
    let count_number_one: u32 = image_layers[min_zero_index]
        .iter()
        .filter(|&o| *o == 1)
        .count() as u32;
    let count_number_two: u32 = image_layers[min_zero_index]
        .iter()
        .filter(|&t| *t == 2)
        .count() as u32;

    count_number_one * count_number_two
}
