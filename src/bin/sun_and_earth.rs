use pixels::wgpu::Color;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use colored::Colorize;

use rand;
use rand::Rng;

use glam::f32::Vec2;
use Wallfacer::{physics::*, planet::*, util::*};

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as u32, HEIGHT as u32);
        WindowBuilder::new()
            .with_title("Zoom example")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let sun = Planet::new(
        "Sun",
        Vec2::new(500.0, 400.0),
        10.0,
        Vec2::new(0.0, 0.0),
        10_000_000_000_000_0.0,
        PlanetColor::white(),
    );

    let earth = Planet::create_satellite(&sun, "Earth", 8.0, 400.0, PlanetColor::green());

    let mut pixels = create_pixel_buffer(&window, WIDTH as u32, HEIGHT as u32);

    pixels.clear_color(Color::BLACK);

    let mut planet_list = Box::new(vec![sun, earth]);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                pixels.frame_mut().fill(0 as u8);
                let mut accel_list: Vec<Vec2> = vec![];
                for s in planet_list.iter() {
                    let mut accel = Vec2::new(0.0, 0.0);
                    for p in planet_list.iter() {
                        if s == p {
                            continue;
                        }
                        accel += calc_accel(&s, &p);
                    }
                    accel_list.push(accel);
                }
                for i in 0..planet_list.len() {
                    planet_list[i].update(accel_list[i]);
                    planet_list[i].render(&mut pixels);
                    planet_list[i].render_force2(&mut pixels);
                }
                // let bright_areas = extract_bright_areas(pixels.frame());

                // Step 2: Apply Gaussian blur to the bright areas
                // let blurred_bright_areas = gaussian_blur(&bright_areas);
                //
                // Step 3: Combine the original frame with the blurred bright areas
                // combine_images(&mut pixels, &blurred_bright_areas);
                pixels.render();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
