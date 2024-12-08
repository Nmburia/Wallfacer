use glam::Vec2;
use pixels::Pixels;

use crate::{
    physics::{calc_accel, check_collision},
    planet::Planet,
};

pub struct PlanetSystem<'a> {
    pub list: Vec<Planet<'a>>,
    pub timestep: f32,
}

impl<'a> PlanetSystem<'a> {
    pub fn empty(timestep: f32) -> Self {
        Self {
            list: vec![],
            timestep,
        }
    }

    pub fn from_vec(timestep: f32, planet_list: Vec<Planet<'a>>) -> Self {
        Self {
            list: planet_list,
            timestep,
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
    }
}
