use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use glam::f32::Vec2;
use Wallfacer::{planet::*, system::*, util::*};

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as u32, HEIGHT as u32);
        WindowBuilder::new()
            .with_title("Solar system")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = create_pixel_buffer(&window, WIDTH as u32, HEIGHT as u32);

    let star = Planet::new(
        "Star",
        Vec2::new(300.0, 400.0),
        10.0,
        Vec2::new(0.0, 0.0),
        10_000_000_000.1,
        PlanetColor::white(),
    );

    let planet1 = Planet::create_satellite(&star, "Planet 1", 5.0, 1000.0, PlanetColor::blue());

    let planet2 = Planet::create_satellite(&planet1, "Satellite", 4.0, 0.1, PlanetColor::green());
    let planet3 = Planet::create_satellite(
        &planet1,
        "Tiny Satellite",
        2.0,
        10.0,
        PlanetColor::new(150, 14, 21, 255),
    );

    let mut planet_list = PlanetSystem::from_vec(0.5, vec![star, planet1, planet2, planet3]);
    for p in planet_list.list.iter() {
        println!(
            "planet '{}' in position x: {}, y: {} ",
            p.name, p.pos.x, p.pos.y
        );
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            pixels.frame_mut().fill(0 as u8);
            planet_list.update_and_render(&mut pixels);
            _ = pixels.render().unwrap();
            for p in planet_list.list.iter() {
                print!(
                    "planet '{}' in position x: {}, y: {} ",
                    p.name, p.pos.x, p.pos.y
                );
            }
            print!("\r");
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
