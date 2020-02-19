use std::fs;

const BLACK: u32 = 0;
const TRANSPARENT: u32 = 2;
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

    println!("What message is produced after decoding your image?");
    decode_image(images);
}

fn decode_image(images: Vec<u32>) {
    let mut decoded_message: [u32; LAYER_LENGTH] = [2; LAYER_LENGTH];
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
    for pixel in 0..LAYER_LENGTH {
        let mut layer = 0;
        while layer < image_layers.len() {
            if image_layers[layer][pixel] != TRANSPARENT {
                decoded_message[pixel] = image_layers[layer][pixel];
                break;
            }
            layer += 1;
        }
    }

    i = 0;
    while i < LAYER_LENGTH {
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                if decoded_message[i] == BLACK {
                    print!(" ");
                } else {
                    print!("|");
                }
                i += 1;
            }
            println!();
        }
    }
}
