use winit::window::Window;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use colored::Colorize;

use rand;
use rand::Rng;
use std::time::{Duration, Instant};

use glam::f32::Vec2;
use pixels::wgpu::Color;

mod physics;
use physics::*;

mod planet;
use planet::*;

mod util;
use util::*;

const delta_t: f32 = 1.1;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const BRIGHTNESS_THRESHOLD: u8 = 100;
const BLUR_RADIUS: usize = 5;

// use std::time::{Duration, Instant};
//
// fn main() {
// let start = Instant::now();
// expensive_function();
// let duration = start.elapsed();

// println!("Time elapsed in expensive_function() is: {:?}", duration);
// }

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

    // let mut planet = Planet {pos: Vec2::new(200.0, 200.0), radius: 5.0, vel: Vec2::new(0.0, 0.0), mass: 100000.1};
    let mut planet = Planet::new(
        "Earth",
        Vec2::new(200.0, 200.0),
        5.0,
        Vec2::new(0.0, 0.0),
        100_000.1,
        PlanetColor::red(),
    );

    // good values between two planets are 1.1 and  10000000000000.0
    // let mut planet2 = Planet {pos: Vec2::new(400.0, 300.0), radius: 10.0, vel: Vec2::new(0.05, 0.0), mass: 10_000_000_000_000_0.0};
    let mut planet2 = Planet::new(
        "Sun",
        Vec2::new(400.0, 300.0),
        10.0,
        Vec2::new(0.0, 0.0),
        10_000_000_000.0,
        PlanetColor::white(),
    );
    let init_vel = calc_init_orbital_velocity(&planet, &planet2);
    planet.vel = init_vel;

    println!(
        "Will escape orbit: {}",
        check_escape_velocity(&planet, &planet2)
    );

    let mut planet3 = Planet::new(
        "Moon",
        Vec2::new(500.0, 500.0),
        3.0,
        Vec2::new(0.0, 0.0),
        1000.5,
        PlanetColor::blue(),
    );
    // let init_vel = calc_init_orbital_velocity(&planet3, &planet2);
    // planet3.vel = init_vel;

    println!(
        "Will escape orbit: {}",
        check_escape_velocity(&planet3, &planet2)
    );

    // let planet4 = Planet::create_satellite(&planet2, "Satellite", 8.0, 400.0, PlanetColor::green());
    // let planet5 = Planet::create_satellite(
    // &planet,
    // "Tiny Satellite",
    // 2.0,
    // 100.0,
    // PlanetColor::new(150, 14, 21, 255),
    // );

    let mut pixels = create_pixel_buffer(&window, WIDTH as u32, HEIGHT as u32);

    pixels.clear_color(Color::BLACK);

    let mut planet_list = Box::new(vec![planet, planet2, planet3]);
    // let mut planet_list: &mut Vec<Planet> = &mut planets;

    event_loop.run(move |event, _, control_flow| {
        // let mut planet_list: Rc<RefCell<Vec<&mut Planet>>> = Rc::new(RefCell::new(vec![&mut planet,  &mut planet2]));

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
                        if check_collision(&s, &p) {
                            accel += calc_collision(s, p);
                        }
                        accel += calc_accel(&s, &p);
                    }
                    // println!("{:?}", accel);
                    accel_list.push(accel);
                }
                for i in 0..planet_list.len() {
                    planet_list[i].update(accel_list[i]);
                    planet_list[i].render(&mut pixels);
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
