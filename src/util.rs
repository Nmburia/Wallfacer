use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;



pub const delta_t: f32 = 1.1;

pub const WIDTH: usize = 1200;
pub const HEIGHT: usize = 800;
pub const BRIGHTNESS_THRESHOLD: u8 = 100;
pub const BLUR_RADIUS: usize = 5;




pub fn create_pixel_buffer(window: &Window, w: u32, h: u32) -> Pixels {
    let surface_texture = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        &window
    );
    Pixels::new(w, h, surface_texture).unwrap()
}

fn extract_bright_areas(frame: &[u8]) -> Vec<u8> {
    let mut bright_areas = vec![0; frame.len()];

    for i in 0..(frame.len() / 4) {
        let r = frame[4 * i];
        let g = frame[4 * i + 1];
        let b = frame[4 * i + 2];

        let brightness = (r as u32 + g as u32 + b as u32) / 3;

        if brightness as u8 > BRIGHTNESS_THRESHOLD {
            bright_areas[4 * i] = r;
            bright_areas[4 * i + 1] = g;
            bright_areas[4 * i + 2] = b;
            bright_areas[4 * i + 3] = 255;
        } else {
            bright_areas[4 * i] = 0;
            bright_areas[4 * i + 1] = 0;
            bright_areas[4 * i + 2] = 0;
            bright_areas[4 * i + 3] = 0;
        }
    }

    bright_areas
}

fn gaussian_blur(bright_areas: &[u8]) -> Vec<u8> {
    let mut blurred = bright_areas.to_vec();
    let width = WIDTH as usize;
    let height = HEIGHT as usize;

    for _ in 0..BLUR_RADIUS {
        let mut temp = blurred.clone();

        for y in 0..height {
            for x in 0..width {
                let mut sum_r = 0;
                let mut sum_g = 0;
                let mut sum_b = 0;
                let mut count = 0;

                for ky in y.saturating_sub(1)..=(y + 1).min(height - 1) {
                    for kx in x.saturating_sub(1)..=(x + 1).min(width - 1) {
                        let i = 4 * (ky * width + kx);
                        sum_r += blurred[i] as u32;
                        sum_g += blurred[i + 1] as u32;
                        sum_b += blurred[i + 2] as u32;
                        count += 1;
                    }
                }

                let i = 4 * (y * width + x);
                temp[i] = (sum_r / count) as u8;
                temp[i + 1] = (sum_g / count) as u8;
                temp[i + 2] = (sum_b / count) as u8;
                temp[i + 3] = bright_areas[i + 3];
            }
        }

        blurred = temp;
    }

    blurred
}

fn combine_images(frame: &mut Pixels, blurred: &[u8]) {
    for i in 0..(frame.frame().len() / 4) {
        let base_r = frame.frame()[4 * i];
        let base_g = frame.frame()[4 * i + 1];
        let base_b = frame.frame()[4 * i + 2];

        let overlay_r = blurred[4 * i];
        let overlay_g = blurred[4 * i + 1];
        let overlay_b = blurred[4 * i + 2];
        let overlay_a = blurred[4 * i + 3] as f32 / 255.0;

        frame.frame_mut()[4 * i] = ((base_r as f32 * (1.0 - overlay_a) + overlay_r as f32 * overlay_a) as u8).min(255);
        frame.frame_mut()[4 * i + 1] = ((base_g as f32 * (1.0 - overlay_a) + overlay_g as f32 * overlay_a) as u8).min(255);
        frame.frame_mut()[4 * i + 2] = ((base_b as f32 * (1.0 - overlay_a) + overlay_b as f32 * overlay_a) as u8).min(255);
    }
}
