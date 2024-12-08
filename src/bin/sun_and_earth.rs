use pixels::wgpu::Color;
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
            .with_title("Zoom example")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = create_pixel_buffer(&window, WIDTH as u32, HEIGHT as u32);
    pixels.clear_color(Color::BLACK);

    let sun = Planet::new(
        "Sun",
        Vec2::new(500.0, 400.0),
        10.0,
        Vec2::new(0.0, 0.0),
        10_000_000_000_000_0.0,
        PlanetColor::white(),
    );

    let earth = Planet::create_satellite(&sun, "Earth", 8.0, 400.0, PlanetColor::green());

    let mut planet_list = PlanetSystem::from_vec(0.1, vec![sun, earth]);

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
