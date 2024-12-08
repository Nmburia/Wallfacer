use std::time::Instant;

use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, SwashCache};
use glam::Vec2;
use pixels::Pixels;

use crate::{
    physics::{calc_accel, check_collision},
    planet::Planet,
    HEIGHT, WIDTH,
};

pub struct PlanetSystem<'a> {
    pub list: Vec<Planet<'a>>,
    pub timestep: f32,
    systeminfo: SystemInfo,
}

impl<'a> PlanetSystem<'a> {
    pub fn empty(timestep: f32) -> Self {
        let systeminfo = SystemInfo::new();
        Self {
            list: vec![],
            timestep,
            systeminfo,
        }
    }

    pub fn from_vec(timestep: f32, planet_list: Vec<Planet<'a>>) -> Self {
        let systeminfo = SystemInfo::new();
        Self {
            list: planet_list,
            timestep,
            systeminfo,
        }
    }

    pub fn add_planet(&mut self, planet: Planet<'a>) {
        self.list.push(planet);
    }

    pub fn update_system(&mut self) {
        let mut accel_list: Vec<Vec2> = vec![];
        let mut merge_list: Vec<(usize, usize)> = vec![];
        for s in self.list.iter() {
            let mut accel = Vec2::new(0.0, 0.0);
            for p in self.list.iter() {
                if s == p {
                    continue;
                }
                if check_collision(&s, &p) {
                    if s.mass >= p.mass {
                        //p gets merged into s
                        let s_index = self.list.iter().position(|x| *x == *s);
                        let p_index = self.list.iter().position(|x| *x == *p);
                        if (s_index != None) & (p_index != None) {
                            if !merge_list
                                .iter()
                                .any(|(_, b)| (*b == p_index.unwrap()) | (*b == s_index.unwrap()))
                            {
                                merge_list.push((s_index.unwrap(), p_index.unwrap()));
                            }
                        }
                    }
                }
                accel += calc_accel(&s, &p);
            }
            accel_list.push(accel);
        }
        for i in 0..self.list.len() {
            self.list[i].update(self.timestep, accel_list[i]);
        }
        for pair in merge_list.iter() {
            let mass_a = self.list[pair.0].mass;
            let vel_a = self.list[pair.0].vel;
            let mass_b = self.list[pair.1].mass;
            let vel_b = self.list[pair.1].vel;

            let init_momentum = mass_a * vel_a + mass_b * vel_b;
            let final_mass = mass_a + mass_b;
            let final_velocity = init_momentum / final_mass;

            self.list.remove(pair.1);
            self.list[pair.0].mass += final_mass;
            self.list[pair.0].vel += final_velocity;
        }
    }

    pub fn render_system(&mut self, pixels: &mut Pixels) {
        for p in self.list.iter() {
            p.render(pixels);
        }
    }

    pub fn update_and_render(&mut self, pixels: &mut Pixels) {
        self.update_system();
        self.render_system(pixels);
        self.print_info(pixels);
    }

    pub fn print_info(&mut self, pixels: &mut Pixels) {
        self.systeminfo.render_info(pixels);
    }
}

struct SystemInfo {
    last_frame_time: Instant,
    font_system: FontSystem,
    buffer: Buffer,
    swash_cache: SwashCache,
}

impl SystemInfo {
    pub fn new() -> Self {
        let now = Instant::now();
        let mut font_system = FontSystem::new();
        let buffer = Buffer::new(&mut font_system, Metrics::new(16.0, 16.0));
        let swash_cache = SwashCache::new();
        Self {
            last_frame_time: now,
            font_system,
            buffer,
            swash_cache,
        }
    }

    pub fn render_info(&mut self, pixels: &mut Pixels) {
        let now = Instant::now();
        let delta_t = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;
        let fps = 1.0 / delta_t.as_secs_f32();
        let text = format!("FPS: {}", fps);
        let text_color = Color::rgb(0xFF, 0xFF, 0xFF);

        self.buffer.set_text(
            &mut self.font_system,
            text.as_str(),
            Attrs::new(),
            cosmic_text::Shaping::Advanced,
        );

        self.buffer.draw(
            &mut self.font_system,
            &mut self.swash_cache,
            text_color,
            |x, y, w, h, color| {
                let frame = pixels.frame_mut();
                // Loop over each pixel in the rectangle
                for dy in 0u32..h {
                    for dx in 0u32..w {
                        // Calculate the index in the pixel buffer
                        let px = x as u32 + dx;
                        let py = y as u32 + dy;
                        if px < WIDTH as u32 && py < HEIGHT as u32 {
                            let index = (py as u32 * WIDTH as u32 * 4 + px as u32 * 4) as usize;

                            if index + 3 < frame.len() {
                                frame[index] = color.r();
                                frame[index + 1] = color.g();
                                frame[index + 2] = color.b();
                                frame[index + 3] = color.a();
                            }
                        }
                    }
                }
            },
        );
    }
}
