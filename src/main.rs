use winit::window::Window;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture};
use pixels::wgpu::Color;


use glam::f32::Vec2;

use std::rc::Rc;
use std::cell::RefCell;

const delta_t: f32 = 1.1;

fn main() {

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(800, 600);
        WindowBuilder::new()
            .with_title("Zoom example")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };





    let mut planet = Planet {pos: Vec2::new(200.0, 200.0), radius: 5.0, vel: Vec2::new(0.0, 0.0), mass: 1.1};


    let mut planet2 = Planet {pos: Vec2::new(400.0, 300.0), radius: 10.0, vel: Vec2::new(0.0, 0.0), mass: 10000000000000.0};
    // good values between two planets are 1.1 and  10000000000000.0

    let init_vel = calc_init_orbital_velocity(&planet, &planet2);
    planet.vel = init_vel;


    println!("Will escape orbit: {}", check_escape_velocity(&planet, &planet2));
    

    let mut pixels = create_pixel_buffer(&window, 800, 600);

    let clear_color = Color { r: 0.031, g: 0.004, b: 0.157, a: 0.2 as f64}; 
    pixels.clear_color(Color::BLACK);
    
    event_loop.run(move |event, _, control_flow| {


    // let mut planet_list: Rc<RefCell<Vec<&mut Planet>>> = Rc::new(RefCell::new(vec![&mut planet,  &mut planet2]));
    let mut planet_list = vec![&planet, &planet2];

        match event {
            Event::MainEventsCleared => {
                pixels.frame_mut().fill(0 as u8);
                let mut accel_list: Vec<Vec2>  = vec![];
                for s in &planet_list {
                    let mut accel = Vec2::new(0.0, 0.0);
                    for  p in &planet_list {
                        if s == p {
                            continue;
                        }
                        accel += calc_accel(s, p);
                        
                        
                    }
                    // println!("{:?}", accel);
                    accel_list.push(accel);
                }
                // planet.render(&mut pixels);
                // planet2.render(&mut pixels);
                planet.update(accel_list[0]);
                planet2.update(accel_list[1]);
                planet.render(&mut pixels);
                planet2.render(&mut pixels);
                pixels.render();
            }
            _ => {}
        } 
    });

    drop(planet);
    drop(planet2);

    
}

fn create_pixel_buffer(window: &Window, w: u32, h: u32) -> Pixels {
    let surface_texture = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        &window
    );
    Pixels::new(w, h, surface_texture).unwrap()
}


#[derive(Copy, Clone, PartialEq)]
pub struct Planet {
    pos: Vec2,
    radius: f32,
    vel: Vec2,
    mass: f32
}

impl Planet {
    pub fn render (self, px: &mut Pixels) {
        for y in ((self.pos.y-self.radius) as usize)..((self.pos.y+self.radius) as usize) {
            for x in ((self.pos.x-self.radius) as usize)..((self.pos.x+self.radius) as usize) {
                let circle_check = ((x as f32 - self.pos.x).powf(2.0) + (y as f32 - self.pos.y).powf(2.0));
                if (circle_check < self.radius.powf(2.0)) || (circle_check == self.radius.powf(2.0)) {
                    let index = y * 800 * 4 + x * 4 as usize;
                    if index > (800 * 600 * 4) {
                        continue;
                    }
                    px.frame_mut()[index..index+4].copy_from_slice(vec![255u8, 0u8, 0u8, (1.0 * 255.0) as u8].as_slice());
                }
            }
        }
    }

    pub fn update (&mut self, accel: Vec2) {
        // self.vel += Vec2::new(0.01, 0.01);  //accel
        self.vel += accel * delta_t;
        self.pos += self.vel * delta_t;
        // println!("{}",self.vel);
    }
}

pub fn calc_accel(self_planet: &Planet, planet: &Planet) -> Vec2 {
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let mut resultant_force = Vec2::new(0.0, 0.0);
        //ma = Gmm/r^2   -> a = Gm/r^2
    let dist = ((planet.pos.x - self_planet.pos.x).powf(2.0) + (planet.pos.y - self_planet.pos.y).powf(2.0)).sqrt().abs(); 
    let magnitude = (G * self_planet.mass * planet.mass) / dist.powi(2);
    let force = (planet.pos - self_planet.pos).normalize_or_zero() * magnitude;
    resultant_force += force;
    let accel = force / self_planet.mass;
    accel
}


pub fn calc_init_orbital_velocity(planet: &Planet, sun: &Planet) -> Vec2 {
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let dist = ((sun.pos.x - planet.pos.x).powf(2.0) + (sun.pos.y - planet.pos.y).powf(2.0)).sqrt().abs();
    let v = ((G * sun.mass) / dist).sqrt();
    let force = (sun.pos - planet.pos).perp().normalize_or_zero() * v.sqrt().abs();
    force
}

pub fn check_escape_velocity(planet: &Planet, sun: &Planet) -> bool {
// checks whether a given planet will escape the orbit of a given sun
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let dist = ((sun.pos.x - planet.pos.x).powf(2.0) + (sun.pos.y - planet.pos.y).powf(2.0)).sqrt().abs();
    let escape_vel = ((2.0 * G * sun.mass) / dist ).sqrt();
    if planet.vel.length() > escape_vel {
        true
    } else {
        false
    }
}
