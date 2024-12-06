use crate::planet::Planet;
use glam::Vec2;

pub fn calc_accel(self_planet: &Planet, planet: &Planet) -> Vec2 {
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    // let G: f32 = 0.0;
    let mut resultant_force = Vec2::new(0.0, 0.0);
    //ma = Gmm/r^2   -> a = Gm/r^2
    let dist = ((planet.pos.x - self_planet.pos.x).powf(2.0)
        + (planet.pos.y - self_planet.pos.y).powf(2.0))
    .sqrt()
    .abs();
    let magnitude = (G * self_planet.mass * planet.mass) / dist.powi(2);
    let force = (planet.pos - self_planet.pos).normalize_or_zero() * magnitude;
    resultant_force += force;
    let accel = force / self_planet.mass;
    accel
}

pub fn calc_init_orbital_velocity(planet: &Planet, sun: &Planet) -> Vec2 {
    // V = ((Gm)/r).sqrt()
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let dist = ((sun.pos.x - planet.pos.x).powf(2.0) + (sun.pos.y - planet.pos.y).powf(2.0)).sqrt();
    let v = ((G * sun.mass) / dist).sqrt();
    let force = (sun.pos - planet.pos).perp().normalize_or_zero() * v;
    force
}

pub fn check_escape_velocity(planet: &Planet, sun: &Planet) -> bool {
    // checks whether a given planet will escape the orbit of a given sun
    let G: f32 = 6.6 * (10.0_f32).powf(-11.0_f32);
    let dist = ((sun.pos.x - planet.pos.x).powf(2.0) + (sun.pos.y - planet.pos.y).powf(2.0))
        .sqrt()
        .abs();
    let escape_vel = ((2.0 * G * sun.mass) / dist).sqrt();
    if planet.vel.length() > escape_vel {
        true
    } else {
        false
    }
}

pub fn check_collision(self_planet: &Planet, planet: &Planet) -> bool {
    let check = (self_planet.pos.distance(planet.pos) <= self_planet.radius + planet.radius);
    if check == true {
        println!("Collision detected!")
    }
    check
}

pub fn check_collision_flag(self_planet: &Planet) -> bool {
    self_planet.in_collision
}

pub fn calc_collision(self_planet: &Planet, planet: &Planet) -> Vec2 {
    let old_vel = self_planet.vel;
    let part_a = (2.0 * planet.mass * self_planet.mass / (self_planet.mass + planet.mass));
    let part_b = ((self_planet.vel - planet.vel).dot(self_planet.pos - planet.pos))
        / (self_planet.pos.distance(planet.pos).powi(2));
    let part_c = self_planet.pos - planet.pos;
    let mut collision_force = part_a * part_b * part_c;
    // collision_force = -collision_force.reflect(planet.vel);
    collision_force
}

pub fn calc_collision2(self_planet: &Planet, planet: &Planet) -> Vec2 {
    //calculates the force that planet applies to self planet
    //here self_planet is a, other planet is b
    println!("here");
    let normal = self_planet.pos - planet.pos;
    let norm_normalised = normal.normalize_or_zero();
    let m_a = self_planet.mass;
    let m_b = planet.mass;
    let init_v_a = self_planet.vel;
    let init_v_b = planet.vel;
    let final_v_a = init_v_a
        - (2.0 * m_b / (m_a + m_b))
            * ((init_v_a - init_v_b).dot(normal) / norm_normalised.powf(2.0))
            * normal;
    final_v_a
}

pub fn get_gradient(vel: Vec2) -> f32 {
    let norm = vel.normalize_or_zero();
    let y_d = norm.y;
    let x_d = norm.x;
    y_d / x_d
}

pub fn process_physics_updates(planet_list: &mut Vec<Planet>) {
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
    for i in 0..planet_list.len() {
        planet_list[i].update(accel_list[i]);
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
}
