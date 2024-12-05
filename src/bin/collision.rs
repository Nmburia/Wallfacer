use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const delta_t: f32 = 0.1;

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
    let mut pixels = create_pixel_buffer(&window, WIDTH as u32, HEIGHT as u32);

    let target = Planet::new(
        "Target",
        Vec2::new(500.0, 400.0),
        10.0,
        Vec2::new(0.0, 0.0),
        10_000_000.0,
        PlanetColor::white(),
    );
    let satellite = Planet::new(
        "Satellite",
        Vec2::new(200.0, 100.0),
        10.0,
        Vec2::new(1.0, 1.0) * 15.0,
        100000.0,
        PlanetColor::blue(),
    );
    let mut planet_list = Box::new(vec![target, satellite]);

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            pixels.frame_mut().fill(0 as u8);
            let mut accel_list: Vec<Vec2> = vec![];
            let mut merge_list: Vec<(usize, usize)> = vec![];
            for s in planet_list.iter() {
                let mut accel = Vec2::new(0.0, 0.0);
                for p in planet_list.iter() {
                    if s == p {
                        continue;
                    }
                    if check_collision(&s, &p) {
                        if s.mass >= p.mass {
                            //p gets merged into s
                            let s_index = planet_list.iter().position(|x| *x == *s);
                            let p_index = planet_list.iter().position(|x| *x == *p);
                            if (s_index != None) & (p_index != None) {
                                if !merge_list.iter().any(|(a, b)| {
                                    (*b == p_index.unwrap()) | (*b == s_index.unwrap())
                                }) {
                                    merge_list.push((s_index.unwrap(), p_index.unwrap()));
                                }
                            }
                        }
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
            for pair in merge_list.iter() {
                let mass_a = planet_list[pair.0].mass;
                let vel_a = planet_list[pair.0].vel;
                let mass_b = planet_list[pair.1].mass;
                let vel_b = planet_list[pair.1].vel;

                let init_momentum = mass_a * vel_a + mass_b * vel_b;
                let final_mass = mass_a + mass_b;
                let final_velocity = init_momentum / final_mass;

                planet_list.remove(pair.1);
                planet_list[pair.0].mass += final_mass;
                planet_list[pair.0].vel += final_velocity;
            }
            pixels.render();
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
