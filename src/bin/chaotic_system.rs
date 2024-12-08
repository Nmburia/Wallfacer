use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glam::f32::Vec2;
use pixels::wgpu::Color;

use Wallfacer::{physics::*, planet::*, system::*, util::*};

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
    let planet2 = Planet::new(
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

    let planet3 = Planet::new(
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

    let planet4 = Planet::create_satellite(&planet2, "Satellite", 8.0, 400.0, PlanetColor::green());
    let planet5 = Planet::create_satellite(
        &planet,
        "Tiny Satellite",
        2.0,
        100.0,
        PlanetColor::new(150, 14, 21, 255),
    );

    let mut pixels = create_pixel_buffer(&window, WIDTH as u32, HEIGHT as u32);

    pixels.clear_color(Color::BLACK);

    let mut planet_list =
        PlanetSystem::from_vec(5.5, vec![planet, planet2, planet3, planet4, planet5]);

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            pixels.frame_mut().fill(0 as u8);
            planet_list.update_and_render(&mut pixels);
            pixels.render().unwrap();
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        _ => {}
    });
}
