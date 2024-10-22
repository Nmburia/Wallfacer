use glam::Vec2;
use crate::planet::Planet;

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
    let dist = ((sun.pos.x - planet.pos.x).powf(2.0) + (sun.pos.y - planet.pos.y).powf(2.0)).sqrt().abs();
    let escape_vel = ((2.0 * G * sun.mass) / dist ).sqrt();
    if planet.vel.length() > escape_vel {
        true
    } else {
        false
    }
}

pub fn get_gradient(vel: Vec2) -> f32 {
    let norm = vel.normalize_or_zero();
    let y_d =  norm.y;
    let x_d = norm.x;
    y_d / x_d
}
